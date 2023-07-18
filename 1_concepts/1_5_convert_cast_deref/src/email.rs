use std::{error::Error, fmt::Display};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct EmailString<'a>(&'a str);

impl<'a> EmailString<'a> {
    pub fn from_str(str: &'a str) -> Result<Self, InvalidEmailFormatError> {
        if EMAIL_REGEX.is_match(str) {
            Ok(EmailString(str))
        } else {
            Err(InvalidEmailFormatError)
        }
    }
}

// can't do TryFrom for generic types, see
// https://github.com/rust-lang/rust/issues/50133
impl<'a> TryFrom<&'a str> for EmailString<'a> {
    type Error = InvalidEmailFormatError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        EmailString::from_str(value)
    }
}

impl<'a> Into<&'a str> for EmailString<'a> {
    fn into(self) -> &'a str {
        self.0
    }
}

#[derive(Debug)]
struct InvalidEmailFormatError;

impl Display for InvalidEmailFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("entered email is invalid")
    }
}

impl Error for InvalidEmailFormatError {}

/// Cortesy of https://emailregex.com/
const EMAIL_REGEX_STR: &str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(EMAIL_REGEX_STR).unwrap());

#[cfg(test)]
mod tests {
    use super::EmailString;

    #[test]
    fn valid_email_ok() {
        EmailString::from_str(VALID_EMAIL).expect(VALID_EMAIL_FAIL_ERR);
    }

    #[test]
    fn invalid_email_err() {
        EmailString::from_str(INVALID_EMAIL).expect_err(INVALID_EMAIL_SUCCESS_ERR);
    }

    #[test]
    fn email_str_is_same() {
        let s = EmailString::from_str(VALID_EMAIL).expect(VALID_EMAIL_FAIL_ERR);
        assert_eq!(VALID_EMAIL, s.0);
    }

    #[test]
    fn try_from() {
        EmailString::try_from(VALID_EMAIL).expect(VALID_EMAIL_FAIL_ERR);
        EmailString::try_from(INVALID_EMAIL).expect_err(INVALID_EMAIL_SUCCESS_ERR);
    }

    #[test]
    fn email_is_eq() {
        let email_string = EmailString::try_from(VALID_EMAIL).expect(VALID_EMAIL_FAIL_ERR);
        let s: &str = email_string.into();
        assert_eq!(VALID_EMAIL, s);
    }

    const VALID_EMAIL: &str = "some@email.com";
    const INVALID_EMAIL: &str = "hello, anyone there?";

    const VALID_EMAIL_FAIL_ERR: &str = "Entered a valid email, but it didn't validate";
    const INVALID_EMAIL_SUCCESS_ERR: &str = "Entered invalid email, but didn't error out";
}
