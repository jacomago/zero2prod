use secrecy::{ExposeSecret, Secret};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct NewPassword(Secret<String>);

impl AsRef<Secret<String>> for NewPassword {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

impl NewPassword {
    /// Returns an instance of `NewPassword` if the input satisfies all
    /// our validation constraints on new passwords.
    /// It panics otherwise.
    pub fn parse(s: Secret<String>) -> Result<NewPassword, String> {
        // `.trim()` returns a view over the input `s` without trailing
        // whitespace-like characters.
        // `.is_empty` checks if the view contains any character.
        let is_empty_or_whitespace = s.expose_secret().trim().is_empty();
        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and `̊`).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_short = s.expose_secret().graphemes(true).count() < 12;
        let is_too_long = s.expose_secret().graphemes(true).count() > 128;

        if is_empty_or_whitespace || is_too_short || is_too_long {
            Err("Input is not a valid password.".to_string())
        } else {
            Ok(Self(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::NewPassword;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::Password;
    use fake::Fake;
    use secrecy::Secret;

    #[test]
    fn a_12_grapheme_long_pass_is_valid() {
        let pass = Secret::new("�".repeat(12));
        assert_ok!(NewPassword::parse(pass));
    }
    #[test]
    fn a_pass_shorter_than_12_graphemes_is_rejected() {
        let pass = Secret::new("a".repeat(11));
        assert_err!(NewPassword::parse(pass));
    }

    #[test]
    fn a_64_grapheme_long_pass_is_valid() {
        let pass = Secret::new("�".repeat(64));
        assert_ok!(NewPassword::parse(pass));
    }

    #[test]
    fn a_pass_longer_than_128_graphemes_is_rejected() {
        let pass = Secret::new("a".repeat(129));
        assert_err!(NewPassword::parse(pass));
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = Password(12..128).fake_with_rng(g);
            Self(password)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_password_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        NewPassword::parse(Secret::new(valid_password.0)).is_ok()
    }
}
