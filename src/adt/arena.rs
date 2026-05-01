use std::cell::UnsafeCell;

const CHUNK_CAPACITY: usize = 512;

/// A basic arena that provides stable references to stored items.
///
/// We may want to replace this with `typed-arena` or similar
/// in the future. But making something like this is fun.
pub struct Arena<T> {
  chunks: UnsafeCell<Vec<Chunk<T>>>,
}

impl<T> Default for Arena<T> {
  fn default() -> Self {
    Self {
      chunks: UnsafeCell::new(vec![Chunk::new()]),
    }
  }
}

impl<T> Arena<T> {
  const NON_EMPTY: &str = "chunks are non-empty, new() creates a chunk";

  /// Stores and provides a reference to `value`.
  pub fn alloc(&self, value: T) -> &T {
    // This method mutates `chunks` and returns a reference to its content.
    //
    // This is safe because:
    //  - The lifetime of the returned reference is bound to this Arena,
    //    so it cannot outlive the chunk holding its data.
    //  - We specifically avoid invalidating any of the previous
    //    references produced by `alloc`, so it is safe to mutate `chunks`.
    //
    // There are some pathological cases Arena is unsafe for:
    //  - If `value` itself has references to this Arena, and its `Drop`
    //    impl tries to follow those references, they may be invalid
    //    by the time `Vec` gets around to dropping the `value`.
    //    `Vec` makes no guarantee on order of `drop` calls.
    let chunks: &mut Vec<Chunk<T>> = unsafe { &mut *self.chunks.get() };

    // If the last chunk has no space, create another one.
    if chunks.last().expect(Self::NON_EMPTY).is_full() {
      // While this may cause chunks to resize, this is okay
      // because `Chunk` maintains pointer stability on moves.
      //
      // `Chunk` is just a Vec wrapper, and Vec's docs state:
      //  - Vec is and always will be a (pointer, capacity, length) triplet.
      //  - The memory it points to is on the heap.
      //  - Vec will never perform a "small optimization" where elements are
      //    actually stored on the stack... [because] the contents of a Vec
      //    wouldn't have a stable address if it were only moved.
      chunks.push(Chunk::new());
    }

    // Append to the last chunk.
    //
    // This is guaranteed to not affect stability of previously allocated
    // references, because the Vec documentation states that "push and insert
    // will never (re)allocate if the reported capacity is sufficient."
    // We have checked the capacity above, so this is fine.
    chunks.last_mut().expect(Self::NON_EMPTY).push(value)
  }
}

/// A fixed-capacity chunk of values.
struct Chunk<T> {
  values: Vec<T>,
}

impl<T> Chunk<T> {
  fn new() -> Self {
    Self {
      values: Vec::with_capacity(CHUNK_CAPACITY),
    }
  }

  fn is_full(&self) -> bool {
    self.values.len() == self.values.capacity()
  }

  fn push(&mut self, value: T) -> &T {
    debug_assert!(!self.is_full());
    self.values.push(value);
    self.values.last().expect("just pushed")
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::adt::arena::{Arena, CHUNK_CAPACITY};

  #[gtest]
  fn test_alloc_new_chunk() {
    let arena = Arena::default();
    let mut refs = Vec::new();
    for i in 0..(64 * CHUNK_CAPACITY) {
      refs.push(arena.alloc(i));
    }
    for (i, r) in refs.into_iter().enumerate() {
      expect_eq!(r, &i)
    }
  }
}
