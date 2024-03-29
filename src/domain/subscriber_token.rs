use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberToken(String);

/// Generate a random 25-characters-long case-sensitive subscription token.
fn generate_subscription_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

impl AsRef<str> for SubscriberToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn check_alphanumeric(ch: char) -> bool {
    if ('0'..='9').contains(&ch) || ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) {
        return true;
    }
    false
}

impl SubscriberToken {
    pub fn parse(s: String) -> Result<Self, String> {
        // `.trim()` returns a view over the input `s` without trailing
        // whitespace-like characters.
        // `.is_empty` checks if the view contains any character.
        let is_empty_or_whitespace = s.trim().is_empty();
        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and `̊`).
        //
        // `graphemes` returns an iterator over the graphemes in the input `s`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_incorrect_length = s.graphemes(true).count() != 25;
        // Iterate over all characters in the input `s` to check if any of them matches
        // one of the characters in the forbidden array.
        let contains_forbidden_characters = s.chars().any(|g| !check_alphanumeric(g));

        if is_empty_or_whitespace || is_incorrect_length || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber token.", s))
        } else {
            Ok(Self(s))
        }
    }

    pub fn new() -> Self {
        let token = generate_subscription_token();
        Self(token)
    }
}

impl Default for SubscriberToken {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{subscriber_token::generate_subscription_token, SubscriberToken};
    use claim::{assert_err, assert_ok};
    #[test]
    fn a_25_grapheme_long_token_is_valid() {
        let token = "L".repeat(25);
        assert_ok!(SubscriberToken::parse(token));
    }
    #[test]
    fn a_token_longer_than_25_graphemes_is_rejected() {
        let token = "a".repeat(26);
        assert_err!(SubscriberToken::parse(token));
    }
    #[test]
    fn a_token_shorter_than_25_graphemes_is_rejected() {
        let token = "a".repeat(24);
        assert_err!(SubscriberToken::parse(token));
    }
    #[test]
    fn whitespace_only_tokens_are_rejected() {
        let token = " ".to_string();
        assert_err!(SubscriberToken::parse(token));
    }
    #[test]
    fn empty_string_is_rejected() {
        let token = "".to_string();
        assert_err!(SubscriberToken::parse(token));
    }
    #[test]
    fn tokens_containing_an_invalid_character_are_rejected() {
        for token in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let token = token.to_string();
            assert_err!(SubscriberToken::parse(token));
        }
    }
    #[test]
    fn a_valid_token_is_parsed_successfully() {
        let token = generate_subscription_token();
        assert_ok!(SubscriberToken::parse(token));
    }
}
