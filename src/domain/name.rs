use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Name(String);

impl FromStr for Name {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.trim();
        if text.is_empty() {
            return Err("Empty name".into());
        }

        let len = text.graphemes(true).count();
        if len > 256 {
            return Err(format!("Name too long: {}", text));
        }

        const FORBIDDEN: &str = "/()\"<>\\{}[]";
        if text.chars().any(|c| FORBIDDEN.contains(c)) {
            return Err(format!("Invalid name: {}", text));
        }

        Ok(Self(text.into()))
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Name;
    use claim::{assert_err, assert_ok};

    #[test]
    fn reject_empty_name() {
        let name = "";
        assert_err!(str::parse::<Name>(name));
    }

    #[test]
    fn reject_whitespace_only_name() {
        let name = "    \n";
        assert_err!(str::parse::<Name>(name));
    }

    #[test]
    fn reject_forbidden_characters() {
        for c in "/()\"<>\\{}[]".chars() {
            let name = &c.to_string();
            assert_err!(str::parse::<Name>(name));
        }
    }

    #[test]
    fn reject_too_long_name() {
        let name = "a".repeat(257);
        assert_err!(str::parse::<Name>(&name));
    }

    #[test]
    fn accept_long_name() {
        let name = "a".repeat(256);
        assert_ok!(str::parse::<Name>(&name));
    }

    #[test]
    fn accept_valid_name() {
        let name = "Ursula Le Guin";
        assert_ok!(str::parse::<Name>(name));
    }
}
