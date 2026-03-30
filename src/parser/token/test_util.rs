use googletest::{
  description::Description,
  matcher::MatcherResult,
  prelude::{Matcher, MatcherBase},
};

use crate::parser::token::{
  JangToken,
  keyword::Keyword,
  literal::{Literal, NumericLiteral},
  operator::{Op, Operator, Spacing},
};

#[derive(MatcherBase)]
pub struct IdentMatcher {
  expected_name: String,
}

impl Matcher<&JangToken> for IdentMatcher {
  fn matches(&self, actual: &JangToken) -> MatcherResult {
    if let JangToken::Ident(ident) = actual
      && ident.name() == self.expected_name
    {
      MatcherResult::Match
    } else {
      MatcherResult::NoMatch
    }
  }

  fn describe(&self, matcher_result: MatcherResult) -> Description {
    match matcher_result {
      MatcherResult::Match => format!("is an identifier with name {:?}", self.expected_name).into(),
      MatcherResult::NoMatch => {
        format!("is not an identifier with name {:?}", self.expected_name).into()
      }
    }
  }
}

pub fn ident(expected_name: impl Into<String>) -> IdentMatcher {
  IdentMatcher {
    expected_name: expected_name.into(),
  }
}

#[derive(MatcherBase)]
pub struct KeywordMatcher {
  expected: Keyword,
}

impl KeywordMatcher {
  pub fn new(expected: Keyword) -> Self {
    Self { expected }
  }
}

impl Matcher<&JangToken> for KeywordMatcher {
  fn matches(&self, actual: &JangToken) -> MatcherResult {
    if let JangToken::Keyword(keyword) = actual
      && *keyword == self.expected
    {
      MatcherResult::Match
    } else {
      MatcherResult::NoMatch
    }
  }

  fn describe(&self, matcher_result: MatcherResult) -> Description {
    match matcher_result {
      MatcherResult::Match => format!("is keyword {}", self.expected).into(),
      MatcherResult::NoMatch => format!("is not keyword {}", self.expected).into(),
    }
  }
}

#[macro_export]
macro_rules! keyword {
  ($keyword:ident) => {
    $crate::parser::token::test_util::KeywordMatcher::new(
      $crate::parser::token::keyword::Keyword::$keyword,
    )
  };
}

#[derive(MatcherBase)]
pub struct LiteralMatcher {
  expected: Literal,
}

impl Matcher<&JangToken> for LiteralMatcher {
  fn matches(&self, actual: &JangToken) -> MatcherResult {
    if let JangToken::Literal(literal) = actual
      && literal == &self.expected
    {
      MatcherResult::Match
    } else {
      MatcherResult::NoMatch
    }
  }

  fn describe(&self, matcher_result: MatcherResult) -> Description {
    let expected_str = match &self.expected {
      Literal::Numeric(NumericLiteral::Integral(val)) => format!("integer literal value {val:?}"),
      Literal::Numeric(NumericLiteral::Float(val)) => {
        format!("floating point literal value {val:?}")
      }
    };
    match matcher_result {
      MatcherResult::Match => format!("is {expected_str}").into(),
      MatcherResult::NoMatch => format!("is not {expected_str}").into(),
    }
  }
}

pub fn integral(expected: impl Into<String>) -> LiteralMatcher {
  LiteralMatcher {
    expected: Literal::Numeric(NumericLiteral::Integral(expected.into())),
  }
}

pub fn float(expected: impl Into<String>) -> LiteralMatcher {
  LiteralMatcher {
    expected: Literal::Numeric(NumericLiteral::Float(expected.into())),
  }
}

#[derive(MatcherBase)]
pub struct OperatorMatcher {
  expected: Operator,
}

impl OperatorMatcher {
  pub fn new(op: Op, spacing: Spacing) -> Self {
    Self {
      expected: Operator::new(op, spacing),
    }
  }
}

impl Matcher<&JangToken> for OperatorMatcher {
  fn matches(&self, actual: &JangToken) -> MatcherResult {
    if let JangToken::Operator(literal) = actual
      && literal == &self.expected
    {
      MatcherResult::Match
    } else {
      MatcherResult::NoMatch
    }
  }

  fn describe(&self, matcher_result: MatcherResult) -> Description {
    let spacing_str = match &self.expected.spacing() {
      Spacing::Alone => "",
      Spacing::Joint => " (joint)",
    };
    match matcher_result {
      MatcherResult::Match => format!("is {}{spacing_str}", self.expected.op()).into(),
      MatcherResult::NoMatch => format!("is not {}{spacing_str}", self.expected.op()).into(),
    }
  }
}

#[macro_export]
macro_rules! operator {
  ($op:ident) => {
    $crate::parser::token::test_util::OperatorMatcher::new(
      $crate::parser::token::operator::Op::$op,
      $crate::parser::token::operator::Spacing::Alone,
    )
  };
}

#[macro_export]
macro_rules! joint_operator {
  ($op:ident) => {
    $crate::parser::token::test_util::OperatorMatcher::new(
      $crate::parser::token::operator::Op::$op,
      $crate::parser::token::operator::Spacing::Joint,
    )
  };
}
