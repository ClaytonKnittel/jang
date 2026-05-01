use std::cell::{RefCell, RefMut};

/// A basic arena that provides stable references to stored items.
/// [`Arena`] is not thread-safe.
pub struct Arena<T> {
  chunks: RefCell<Vec<Chunk<T>>>,
}

impl<T> Default for Arena<T> {
  fn default() -> Self {
    Self {
      chunks: RefCell::new(vec![Chunk::default()]),
    }
  }
}

impl<T> Arena<T> {
  pub fn alloc(&self, value: T) -> &T {
    let mut chunk = self.chunk_with_capacity();
    let entry: &T = chunk.push(value);

    // Extend the lifetime of the reference to `entry`.
    //
    // This is safe because:
    // - The lifetime of the returned reference is bound to this Arena,
    //   so it cannot outlive the chunk holding its data.
    // - A chunk's underlying Vec is never expanded beyond its capacity,
    //   so it is guarenteed to never reallocate.
    //
    // Vec actually makes quite a few guarentees about how and where
    // its elements are allocated. From the docs:
    //  - Vec is and always will be a (pointer, capacity, length) triplet.
    //  - The memory it points to is on the heap.
    //  - `push` and `insert` will never (re)allocate if the reported
    //    capacity is sufficient.
    //  - Vec will never perform a "small optimization" where elements are
    //    actually stored on the stack... [because] the contents of a Vec
    //    wouldn’t have a stable address if it were only moved.
    unsafe { std::mem::transmute(entry) }
  }

  fn chunk_with_capacity(&self) -> RefMut<'_, Chunk<T>> {
    RefMut::map(self.chunks.borrow_mut(), |chunks| {
      let last_chunk = chunks.last().expect("Arena::default creates a chunk");
      if last_chunk.full() {
        chunks.push(Chunk::default());
      }
      chunks.last_mut().expect("Arena::default creates a chunk")
    })
  }
}

/// A fixed-capacity chunk of values.
struct Chunk<T> {
  values: Vec<T>,
}

const CHUNK_SIZE: usize = 512;

impl<T> Default for Chunk<T> {
  fn default() -> Self {
    Self {
      values: Vec::with_capacity(CHUNK_SIZE),
    }
  }
}

impl<T> Chunk<T> {
  fn full(&self) -> bool {
    self.values.len() == self.values.capacity()
  }

  fn push(&mut self, value: T) -> &T {
    debug_assert!(!self.full());
    self.values.push(value);
    self.values.last().expect("just pushed")
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::adt::arena::{Arena, CHUNK_SIZE};

  #[gtest]
  fn test_alloc_new_chunk() {
    let arena = Arena::default();
    let mut refs = Vec::new();
    for i in 0..(64 * CHUNK_SIZE) {
      refs.push(arena.alloc(i));
    }
    for (i, r) in refs.into_iter().enumerate() {
      expect_eq!(r, &i)
    }
  }
}
