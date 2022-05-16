use secrecy::{ExposeSecret, Secret};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct NewPassword(Secret<String>);

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

// # 	Description 	L1 	L2 	L3 	CWE 	NIST §
// 2.1.1 	Verify that user set passwords are at least 12 characters in length (after multiple spaces are combined). (C6) 	✓ 	✓ 	✓ 	521 	5.1.1.2
// 2.1.2 	Verify that passwords of at least 64 characters are permitted, and that passwords of more than 128 characters are denied. (C6) 	✓ 	✓ 	✓ 	521 	5.1.1.2
// 2.1.3 	Verify that password truncation is not performed. However, consecutive multiple spaces may be replaced by a single space. (C6) 	✓ 	✓ 	✓ 	521 	5.1.1.2
// 2.1.4 	Verify that any printable Unicode character, including language neutral characters such as spaces and Emojis are permitted in passwords. 	✓ 	✓ 	✓ 	521 	5.1.1.2
// 2.1.5 	Verify users can change their password. 	✓ 	✓ 	✓ 	620 	5.1.1.2
// 2.1.6 	Verify that password change functionality requires the user's current and new password. 	✓ 	✓ 	✓ 	620 	5.1.1.2
// 2.1.7 	Verify that passwords submitted during account registration, login, and password change are checked against a set of breached passwords either locally (such as the top 1,000 or 10,000 most common passwords which match the system's password policy) or using an external API. If using an API a zero knowledge proof or other mechanism should be used to ensure that the plain text password is not sent or used in verifying the breach status of the password. If the password is breached, the application must require the user to set a new non-breached password. (C6) 	✓ 	✓ 	✓ 	521 	5.1.1.2
// 2.1.8 	Verify that a password strength meter is provided to help users set a stronger password. 	✓ 	✓ 	✓ 	521 	5.1.1.2
// 2.1.9 	Verify that there are no password composition rules limiting the type of characters permitted. There should be no requirement for upper or lower case or numbers or special characters. (C6) 	✓ 	✓ 	✓ 	521 	5.1.1.2
// 2.1.10 	Verify that there are no periodic credential rotation or password history requirements. 	✓ 	✓ 	✓ 	263 	5.1.1.2
// 2.1.11 	Verify that "paste" functionality, browser password helpers, and external password managers are permitted. 	✓ 	✓ 	✓ 	521 	5.1.1.2
// 2.1.12 	Verify that the user can choose to either temporarily view the entire masked password, or temporarily view the last typed character of the password on platforms that do not have this as built-in functionality.
#[cfg(test)]
mod tests {
    use crate::domain::NewPassword;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::Password;
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
}
