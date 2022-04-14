use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

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

impl SubscriberToken {
    pub fn new() -> Self {
        let token = generate_subscription_token();
        Self(token)
    }
}
