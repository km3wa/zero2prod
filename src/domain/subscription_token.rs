//use rand::{thread_rng, Rng, distributions::Alphanumeric};
use unicode_segmentation::UnicodeSegmentation;

pub enum ParseError {
    NotAlphanumeric,
    IncorrectLength,
}

#[derive(Debug)]
pub struct SubscriptionToken<'query>(&'query str);

impl<'query> SubscriptionToken<'query> {
    pub fn parse(str: &'query str) -> Result<Self, ParseError> {
        if str.graphemes(true).count() != 25 {
            Err(ParseError::IncorrectLength)
        } else if !str.chars().all(|g| g.is_ascii_alphanumeric()) {
            Err(ParseError::NotAlphanumeric)
        } else {
            Ok(Self(str))
        }
    }
}

impl AsRef<str> for SubscriptionToken<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
