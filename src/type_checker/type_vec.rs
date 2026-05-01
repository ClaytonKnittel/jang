use std::ops::Index;

use crate::type_checker::types::concrete::ConcreteType;

#[derive(Clone, Copy)]
pub struct TypeId(usize);

#[derive(Default)]
pub struct TypeVec {
  types: Vec<ConcreteType>,
}

impl TypeVec {
  pub fn push(&mut self, ty: ConcreteType) -> TypeId {
    let id = TypeId(self.types.len());
    self.types.push(ty);
    id
  }
}

impl Index<TypeId> for TypeVec {
  type Output = ConcreteType;

  fn index(&self, index: TypeId) -> &Self::Output {
    &self.types[index.0]
  }
}
