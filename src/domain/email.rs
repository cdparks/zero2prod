use std::str::FromStr;
use validator::validate_email;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Email(String);

impl FromStr for Email {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.trim();
        if validate_email(text) {
            Ok(Email(text.into()))
        } else {
            Err(format!("Invalid email: {}", text))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use claim::{assert_err, assert_ok};

    #[test]
    fn reject_empty_email() {
        let email = "";
        assert_err!(str::parse::<Email>(email));
    }

    #[test]
    fn reject_whitespace_only_email() {
        let email = "    \n";
        assert_err!(str::parse::<Email>(email));
    }

    #[test]
    fn reject_invalid_email() {
        let email = "chris at example dot com";
        assert_err!(str::parse::<Email>(email));
    }

    #[test]
    fn accept_valid_email() {
        let email = "chris@example.com";
        assert_ok!(str::parse::<Email>(email));
    }
}
