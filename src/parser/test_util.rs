use googletest::{
  description::Description,
  matcher::MatcherResult,
  prelude::{Matcher, MatcherBase},
};

use crate::parser::token::{JangToken, Keyword};

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
    $crate::parser::test_util::KeywordMatcher::new($crate::parser::token::Keyword::$keyword)
  };
}
