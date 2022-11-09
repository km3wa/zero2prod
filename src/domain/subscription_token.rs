pub enum ParseError {
    NotAlphanumeric,
    IncorrectLength,
}

#[derive(Debug)]
pub struct SubscriptionToken<'query>(&'query str);

impl<'query> SubscriptionToken<'query> {
    pub fn parse(str: &'query str) -> Result<Self, ParseError> {
        if !str.chars().all(|g| g.is_ascii_alphanumeric()) {
            Err(ParseError::NotAlphanumeric)
        } else if str.len() != 25 {
            Err(ParseError::IncorrectLength)
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
