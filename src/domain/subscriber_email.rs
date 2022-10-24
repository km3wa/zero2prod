#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        if validator::validate_email(s.as_str()) {
            Ok(Self(s))
        } else {
            Err("invalid email".to_string())
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};

    // Both `Clone` and `Debug` are required by `quickcheck`
    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);
    impl Arbitrary for ValidEmailFixture {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        dbg!(&valid_email.0);
        SubscriberEmail::parse(valid_email.0).is_ok()
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn valid_email_is_accepted() {
        let email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }
}
