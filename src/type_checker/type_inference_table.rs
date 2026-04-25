use std::{collections::HashMap, fmt::Display, ops::Index};

use cknittel_util::{
  from_variants::FromVariants,
  union_find::{UnionFind, UnionFindData, UnionFindMergeError},
};

use crate::type_checker::{
  error::{TypeCheckerError, TypeCheckerResult},
  types::{concrete::ConcreteType, primitive::PrimitiveType},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeConstraint {
  Integral,
  Floating,
}

impl Display for TypeConstraint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(match self {
      TypeConstraint::Integral => "integral",
      TypeConstraint::Floating => "floating",
    })
  }
}

#[derive(Clone, Debug, PartialEq, Eq, FromVariants)]
pub enum InferredType {
  Concrete(ConcreteType),
  Constraint(TypeConstraint),
}

impl InferredType {
  pub fn is_integral(&self) -> bool {
    match self {
      InferredType::Concrete(ConcreteType::Primitive(p)) => p.is_integer(),
      InferredType::Constraint(TypeConstraint::Integral) => true,
      _ => false,
    }
  }

  pub fn is_floating(&self) -> bool {
    match self {
      InferredType::Concrete(ConcreteType::Primitive(p)) => p.is_float(),
      InferredType::Constraint(TypeConstraint::Floating) => true,
      _ => false,
    }
  }

  pub fn is_numeric(&self) -> bool {
    self.is_integral() || self.is_floating()
  }

  pub fn is_bool(&self) -> bool {
    match self {
      InferredType::Concrete(ConcreteType::Primitive(p)) => p.is_bool(),
      _ => false,
    }
  }
}

impl Display for InferredType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      InferredType::Concrete(concrete) => write!(f, "{concrete}"),
      InferredType::Constraint(constraint) => write!(f, "{constraint}"),
    }
  }
}

impl InferredType {
  fn narrows_to(&self, goal: &Self) -> bool {
    use ConcreteType::*;
    use InferredType::*;
    use PrimitiveType::*;
    use TypeConstraint::*;

    matches!(
      (self, goal),
      (Constraint(Integral), Concrete(Primitive(I32 | I64)))
        | (Constraint(Floating), Concrete(Primitive(F32 | F64)))
    )
  }
}

impl UnionFindData for InferredType {
  type Error = TypeCheckerError;

  fn merge(&mut self, other: Self) -> Result<(), UnionFindMergeError<Self, Self::Error>> {
    if self == &other || other.narrows_to(self) {
      Ok(())
    } else if self.narrows_to(&other) {
      *self = other;
      Ok(())
    } else {
      let err = TypeCheckerError::TypeMismatch {
        expected: self.clone(),
        actual: other.clone(),
      };
      Err(UnionFindMergeError::new(err, other))
    }
  }
}

#[derive(Clone, Copy, Default)]
pub struct InferredTypeId(usize);

pub struct TypeInferenceTable {
  uf: UnionFind<InferredType>,
  concrete_type_ids: HashMap<ConcreteType, InferredTypeId>,
}

impl Default for TypeInferenceTable {
  fn default() -> Self {
    Self {
      uf: UnionFind::new_with_data(Vec::new()),
      concrete_type_ids: HashMap::default(),
    }
  }
}

impl TypeInferenceTable {
  pub fn unify(
    &mut self,
    expected: InferredTypeId,
    actual: InferredTypeId,
  ) -> TypeCheckerResult<InferredTypeId> {
    Ok(InferredTypeId(
      self.uf.try_union(expected.0, actual.0)?.id(),
    ))
  }

  pub fn unify_to_concrete(
    &mut self,
    expected: &ConcreteType,
    actual: InferredTypeId,
  ) -> TypeCheckerResult<InferredTypeId> {
    let expected_id = if let Some(expected_id) = self.concrete_type_ids.get(expected) {
      *expected_id
    } else {
      let expected_id = self.push(expected.clone());
      self.concrete_type_ids.insert(expected.clone(), expected_id);
      expected_id
    };
    self.unify(expected_id, actual)
  }

  pub fn push(&mut self, ty: impl Into<InferredType>) -> InferredTypeId {
    InferredTypeId(self.uf.add_set(ty.into()))
  }

  pub fn resolved(&mut self, id: InferredTypeId) -> ConcreteType {
    match self[id].clone() {
      InferredType::Concrete(concrete_type) => concrete_type,
      InferredType::Constraint(TypeConstraint::Floating) => {
        ConcreteType::Primitive(PrimitiveType::F32)
      }
      InferredType::Constraint(TypeConstraint::Integral) => {
        ConcreteType::Primitive(PrimitiveType::I32)
      }
    }
  }
}

impl Index<InferredTypeId> for TypeInferenceTable {
  type Output = InferredType;

  fn index(&self, id: InferredTypeId) -> &Self::Output {
    self.uf.const_find(id.0).data()
  }
}

#[cfg(test)]
pub(crate) mod matchers {
  use super::*;
  use googletest::prelude::*;

  pub fn concrete<'a>(concrete: impl Matcher<&'a ConcreteType>) -> impl Matcher<&'a InferredType> {
    pat!(InferredType::Concrete(concrete))
  }

  pub fn inferred_integral<'a>() -> impl Matcher<&'a InferredType> {
    pat!(InferredType::Constraint(pat!(TypeConstraint::Integral)))
  }

  pub fn inferred_floating<'a>() -> impl Matcher<&'a InferredType> {
    pat!(InferredType::Constraint(pat!(TypeConstraint::Floating)))
  }
}
