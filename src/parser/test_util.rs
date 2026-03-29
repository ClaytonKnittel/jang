use googletest::{
  description::Description,
  matcher::MatcherResult,
  prelude::{Matcher, MatcherBase},
};

use crate::parser::token::JangToken;

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
