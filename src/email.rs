/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
#[cfg(not(feature = "std"))]
use alloc::format;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use core::fmt::Display;
#[cfg(not(feature = "std"))]
use core::fmt::Formatter;
#[cfg(not(feature = "std"))]
use core::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use std::fmt::Display;
#[cfg(feature = "std")]
use std::fmt::Formatter;
#[cfg(feature = "std")]
use std::str::FromStr;
#[cfg(feature = "std")]
use std::string::String;

#[cfg(feature = "std")]
type FormatResult = std::fmt::Result;
#[cfg(not(feature = "std"))]
type FormatResult = core::fmt::Result;

use crate::options::LocalPartValidationOptions;
use crate::{
    DisplayNameSupport, Domain, EmailParseError, EmailValidationOptions, QuotedSupport,
    TextEncoding,
};

const SPECIAL_CHARS: &str = r#""(),:;<>@[\] "#;

const VALID_CHARS: &str = "!#$%&'*+-/=?^_`{|}~";

static DEFAULT_VALIDATION_OPTIONS: EmailValidationOptions = EmailValidationOptions::new();

/// A struct representing an email address.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
#[non_exhaustive]
pub struct EmailAddress {
    display_name: Option<String>,
    domain: Domain,
    local_part: String,
}

impl EmailAddress {
    ///
    /// Tries to create a new email address.
    ///
    /// # Arguments
    ///
    /// * `display_name`: The display name of the email address.
    /// * `local_part`: The local part of the email address.
    /// * `domain`: The domain of the email address.
    /// * `options`: The validation options to use.
    ///
    /// returns: Result<`EmailAddress`, `EmailParseError`> - The parsed email address.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{EmailAddress, EmailValidationOptions, Domain};
    /// use std::error::Error;
    /// use std::str::FromStr;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let domain = Domain::from_str("example.com")?;
    ///     let email = EmailAddress::try_create(
    ///         Some("Test User".to_owned()),
    ///         "test",
    ///         domain,
    ///         &EmailValidationOptions::default()
    ///     )?;
    ///
    ///     assert_eq!(email.display_name(), Some("Test User"));
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail if the email address passed cannot be parsed correctly for the given validation options.
    ///
    pub fn try_create(
        display_name: Option<String>,
        local_part: &str,
        domain: Domain,
        options: &EmailValidationOptions,
    ) -> Result<Self, EmailParseError> {
        Self::parse_local_part(local_part, &options.local_part_options)?;
        Ok(Self {
            display_name,
            domain,
            local_part: local_part.to_owned(),
        })
    }

    ///
    /// Gets the display name of the email address.
    ///
    /// # Arguments
    ///
    /// returns: Option<&str> - The display name of the email address.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::EmailAddress;
    /// use std::error::Error;
    /// use std::str::FromStr;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let email = EmailAddress::from_str("Testing User <test.user@example.com>")?;
    ///     assert_eq!(email.display_name(), Some("Testing User"));
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    ///
    /// Gets the local part of the email address.
    ///
    /// # Arguments
    ///
    /// returns: &str - The local part of the email address.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::EmailAddress;
    /// use std::error::Error;
    /// use std::str::FromStr;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let email = EmailAddress::from_str("Testing User <test.user@example.com>")?;
    ///     assert_eq!(email.local_part(), "test.user");
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub fn local_part(&self) -> &str {
        &self.local_part
    }

    ///
    /// Gets the domain of the email address.
    ///
    /// # Arguments
    ///
    /// returns: &Domain - The domain of the email address.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::EmailAddress;
    /// use std::error::Error;
    /// use std::str::FromStr;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let email = EmailAddress::from_str("Testing User <test.user@example.com>")?;
    ///     assert_eq!(email.local_part(), "test.user");
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub const fn domain(&self) -> &Domain {
        &self.domain
    }

    ///
    /// Checks if the email address is valid.
    ///
    /// # Arguments
    ///
    /// * `str`: The email address to check.
    ///
    /// returns: bool - True if the email address is valid, false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::EmailAddress;
    ///
    /// assert_eq!(EmailAddress::is_valid("test.user@example.com"), true);
    /// assert_eq!(EmailAddress::is_valid("test.user"), false);
    /// ```
    #[must_use]
    pub fn is_valid(str: &str) -> bool {
        Self::try_parse(str, &DEFAULT_VALIDATION_OPTIONS).is_ok()
    }

    ///
    /// Tries to parse an email address.
    ///
    /// # Arguments
    ///
    /// * `str`: The email address to parse.
    /// * `option`: The validation options to use.
    ///
    /// returns: Result<`EmailAddress`, `EmailParseError`> - The parsed email address, or an error if it cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{EmailAddress, EmailValidationOptions};
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let email_address = EmailAddress::try_parse(
    ///         "Tester <test.user@example.org>",
    ///         &EmailValidationOptions::default()
    ///     )?;
    ///
    ///     assert_eq!(email_address.display_name(), Some("Tester"));
    ///     assert_eq!(email_address.local_part(), "test.user");
    ///     assert_eq!(email_address.domain().address(), "example.org");
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This can fail if the email address passed cannot be parsed correctly for the given validation options.
    ///
    pub fn try_parse(str: &str, option: &EmailValidationOptions) -> Result<Self, EmailParseError> {
        let mut text = str;
        let mut display_name = None;
        if option.display_name_support == DisplayNameSupport::Allowed && str.ends_with('>') {
            let display_end = Self::find_name_end(str)?;
            display_name = Some(str[..(display_end - 1)].trim().to_owned());
            text = &text[display_end..str.len() - 1];
        }

        let domain_seperator = Self::find_local_part_end(text)?;
        if domain_seperator == 0 {
            return Err(EmailParseError::MissingLocalPart);
        }

        let local_part = &text[..domain_seperator];
        let domain_part = &text[domain_seperator + 1..];

        Ok(Self {
            display_name,
            local_part: Self::parse_local_part(local_part, &option.local_part_options)?,
            domain: Domain::try_parse(domain_part, &option.domain_options)?,
        })
    }

    fn find_name_end(str: &str) -> Result<usize, EmailParseError> {
        let mut display_end = 0;
        for (i, c) in str.char_indices() {
            display_end = i + 1;
            if c == '<' {
                break;
            }
        }

        if display_end == str.len() {
            return Err(EmailParseError::InvalidDisplayFormat);
        }

        Ok(display_end)
    }

    fn parse_local_part(
        str: &str,
        option: &LocalPartValidationOptions,
    ) -> Result<String, EmailParseError> {
        let mut cleaned = option.trim_whitespace.trim_string(str);
        cleaned = option.comments.trim(cleaned);
        cleaned = option.trim_whitespace.trim_string(cleaned);
        if cleaned.len() > option.max_length {
            return Err(EmailParseError::LocalPartLengthExceeded {
                length: cleaned.len(),
                max_length: option.max_length,
            });
        }

        let mut dot_found = false;
        let mut escaped = false;
        let quoted = option.quoted_support == QuotedSupport::Allowed
            && cleaned.starts_with('"')
            && cleaned.ends_with('"');
        let text = if quoted {
            &cleaned[1..cleaned.len() - 1]
        } else {
            cleaned
        };

        for (i, c) in text.char_indices() {
            if quoted && c == '\\' {
                let next = text.chars().nth(i + 1);
                match next {
                    None => {
                        return Err(EmailParseError::InvalidEscapeSequence {
                            sequence: "\\".to_owned(),
                        })
                    }
                    Some(n) => {
                        if n != '"' && !(n == '\\' || escaped) {
                            return Err(EmailParseError::InvalidEscapeSequence {
                                sequence: format!("{c}{n}"),
                            });
                        }
                    }
                }

                escaped = !escaped;
            } else if c == '.' {
                if !quoted && dot_found {
                    return Err(EmailParseError::InvalidCharacters {
                        character_set: "..".to_owned(),
                    });
                } else if !quoted && i == 0 {
                    return Err(EmailParseError::InvalidCharacters {
                        character_set: ".".to_owned(),
                    });
                }

                escaped = false;
                dot_found = true;
            } else if (c < '0' || c > '9' && c < 'A' || c > 'Z' && c < 'a' || c > 'z')
                && !VALID_CHARS.contains(c)
                && !(option.text_encoding == TextEncoding::Utf8 && c > '')
            {
                if SPECIAL_CHARS.contains(c) {
                    if !quoted {
                        return Err(EmailParseError::InvalidCharacters {
                            character_set: SPECIAL_CHARS.to_owned(),
                        });
                    } else if !escaped && c == '"' {
                        return Err(EmailParseError::InvalidEscapeSequence {
                            sequence: format!("{c}"),
                        });
                    }
                } else {
                    return Err(EmailParseError::InvalidCharacters {
                        character_set: String::from(c),
                    });
                }

                escaped = false;
                dot_found = false;
            } else {
                dot_found = false;
                escaped = false;
            }
        }

        if dot_found {
            return Err(EmailParseError::InvalidCharacters {
                character_set: ".".to_owned(),
            });
        }

        Ok(cleaned.to_owned())
    }

    fn find_local_part_end(str: &str) -> Result<usize, EmailParseError> {
        let mut comments_found = false;
        for (i, c) in str.char_indices().rev() {
            if !comments_found && c == ')' {
                comments_found = true;
            } else if comments_found && c == '(' {
                comments_found = false;
            } else if !comments_found && c == '@' {
                return Ok(i);
            } else if c == '"' {
                return Err(EmailParseError::MissingDomain);
            }
        }

        Err(EmailParseError::MissingDomain)
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match &self.display_name {
            None => write!(f, "{}@{}", self.local_part, self.domain),
            Some(name) => write!(f, "{} <{}@{}>", name, self.local_part, self.domain),
        }
    }
}

impl TryFrom<String> for EmailAddress {
    type Error = EmailParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_parse(&value, &DEFAULT_VALIDATION_OPTIONS)
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = EmailParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_parse(value, &DEFAULT_VALIDATION_OPTIONS)
    }
}

impl FromStr for EmailAddress {
    type Err = EmailParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s, &DEFAULT_VALIDATION_OPTIONS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CommentSupport, DomainParseError, DomainSupport, DomainType, TrimWhitespace,
        ValidationOptionsBuilder,
    };
    #[cfg(not(feature = "std"))]
    use alloc::format;

    #[test]
    fn empty_email_address_fails() {
        let email = EmailAddress::try_parse("", &EmailValidationOptions::default());
        assert!(email.is_err());
        assert_eq!(email.unwrap_err(), EmailParseError::MissingDomain);
    }

    #[test]
    fn valid_email_address_succeeds() {
        let email = EmailAddress::try_parse("test@example.com", &EmailValidationOptions::default());
        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.display_name, None);
        assert_eq!(email.local_part, "test");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned(),
            }
        );
    }

    #[test]
    fn missing_domain_fails() {
        let email = EmailAddress::try_parse("test", &EmailValidationOptions::default());
        assert!(email.is_err());
        assert_eq!(email.unwrap_err(), EmailParseError::MissingDomain);
    }

    #[test]
    fn missing_local_part_fails() {
        let email = EmailAddress::try_parse("@example.com", &EmailValidationOptions::default());
        assert!(email.is_err());
        assert_eq!(email.unwrap_err(), EmailParseError::MissingLocalPart);
    }

    #[test]
    fn invalid_characters_in_local_part_fails() {
        for c in SPECIAL_CHARS.chars() {
            let email = EmailAddress::try_parse(
                &format!("te{c}st@example.com"),
                &EmailValidationOptions::default(),
            );

            assert!(email.is_err());
            assert_eq!(
                email.unwrap_err(),
                EmailParseError::InvalidCharacters {
                    character_set: SPECIAL_CHARS.to_owned()
                }
            );
        }
    }

    #[test]
    fn display_name_is_parsed_correctly() {
        let email = EmailAddress::try_parse(
            "Test User <test@example.com>",
            &ValidationOptionsBuilder::new()
                .with_display_name_support(DisplayNameSupport::Allowed)
                .build(),
        );

        assert!(email.is_ok());

        let email = email.unwrap();
        assert_eq!(email.display_name, Some("Test User".to_owned()));
        assert_eq!(email.local_part, "test");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned(),
            }
        );
    }

    #[test]
    fn display_name_trims_whitespace_correctly() {
        let email = EmailAddress::try_parse(
            "     Test User     <test@example.com>",
            &ValidationOptionsBuilder::new()
                .with_display_name_support(DisplayNameSupport::Allowed)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.display_name, Some("Test User".to_owned()));
    }

    #[test]
    fn email_address_local_part_front_can_be_trimmed() {
        let email = EmailAddress::try_parse(
            "    spaced_front@example.com",
            &ValidationOptionsBuilder::new()
                .with_trim_whitespace(TrimWhitespace::Start)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "spaced_front");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned(),
            }
        );
    }

    #[test]
    fn email_address_local_part_back_can_be_trimmed() {
        let email = EmailAddress::try_parse(
            "spaced_back     @example.com",
            &ValidationOptionsBuilder::new()
                .with_trim_whitespace(TrimWhitespace::End)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "spaced_back");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned(),
            }
        );
    }

    #[test]
    fn invalid_display_name_fails() {
        let email = EmailAddress::try_parse(
            "Test User >test@example.com>",
            &ValidationOptionsBuilder::new()
                .with_display_name_support(DisplayNameSupport::Allowed)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(email.unwrap_err(), EmailParseError::InvalidDisplayFormat);
    }

    #[test]
    fn display_name_when_not_supported_fails() {
        let email = EmailAddress::try_parse(
            "Test User <test@example.com>",
            &ValidationOptionsBuilder::new()
                .with_display_name_support(DisplayNameSupport::Disallowed)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::InvalidCharacters {
                character_set: SPECIAL_CHARS.to_owned()
            }
        );
    }

    #[test]
    fn local_part_exceeds_max_length_fails() {
        let email = EmailAddress::try_parse(
            "very_long_local_path@example.com",
            &ValidationOptionsBuilder::new()
                .with_max_local_part_length(10)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::LocalPartLengthExceeded {
                length: 20,
                max_length: 10
            }
        );
    }

    #[test]
    fn local_part_at_max_length_is_valid() {
        let email = EmailAddress::try_parse(
            "very_long_local_path@example.com",
            &ValidationOptionsBuilder::new()
                .with_max_local_part_length(20)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "very_long_local_path");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned(),
            }
        );
    }

    #[test]
    fn domain_name_exceeds_max_length_fails() {
        let email = EmailAddress::try_parse(
            "test@very_long_domain_name.com",
            &ValidationOptionsBuilder::new()
                .with_max_domain_length(10)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::DomainParseError(DomainParseError::DomainLengthExceeded {
                length: 25,
                max_length: 10
            })
        );
    }

    #[test]
    fn domain_name_at_max_length_is_valid() {
        let email = EmailAddress::try_parse(
            "test@very-long-domain-name.com",
            &ValidationOptionsBuilder::new()
                .with_max_domain_length(25)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "test");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "very-long-domain-name.com".to_owned(),
            }
        );
    }

    #[test]
    fn dots_are_allowed_in_local_part() {
        let email =
            EmailAddress::try_parse("test.user@example.com", &EmailValidationOptions::default());
        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "test.user");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned(),
            }
        );
    }

    #[test]
    fn double_dots_are_invalid() {
        let email =
            EmailAddress::try_parse("test..user@example.com", &EmailValidationOptions::default());
        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::InvalidCharacters {
                character_set: "..".to_owned(),
            }
        );
    }

    #[test]
    fn dot_at_start_is_invalid() {
        let email =
            EmailAddress::try_parse(".test@example.com", &EmailValidationOptions::default());
        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::InvalidCharacters {
                character_set: ".".to_owned(),
            }
        );
    }

    #[test]
    fn dot_at_end_of_local_part_is_invalid() {
        let email =
            EmailAddress::try_parse("test.user.@example.com", &EmailValidationOptions::default());
        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::InvalidCharacters {
                character_set: ".".to_owned(),
            }
        );
    }

    #[test]
    fn utf8_encoding_is_supported() {
        let email = EmailAddress::try_parse(
            "テスト.ユーザー@example.com",
            &EmailValidationOptions::default(),
        );
        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "テスト.ユーザー".to_owned());
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned(),
            }
        );
    }

    #[test]
    fn utf8_encoding_not_accepted_when_options_ascii() {
        let email = EmailAddress::try_parse(
            "テスト.ユーザー@example.com",
            &ValidationOptionsBuilder::default()
                .with_text_encoding(TextEncoding::Ascii)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::InvalidCharacters {
                character_set: "テ".to_owned(),
            }
        );
    }

    #[test]
    fn utf8_display_name_is_accepted() {
        let email = EmailAddress::try_parse(
            "テスト.ユーザー <test.user@example.com>",
            &EmailValidationOptions::default(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.display_name, Some("テスト.ユーザー".to_owned()));
        assert_eq!(email.local_part, "test.user");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned(),
            }
        );
    }

    #[test]
    fn emojis_can_be_used_in_local_part() {
        let email = EmailAddress::try_parse(
            "I❤️CHOCOLATE@example.com",
            &EmailValidationOptions::default(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "I❤️CHOCOLATE".to_owned());
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned(),
            }
        );
    }

    #[test]
    fn ipv4_address_is_valid() {
        let email = EmailAddress::try_parse(
            "postmaster@[123.123.123.123]",
            &ValidationOptionsBuilder::default()
                .with_domain_support(DomainSupport::All)
                .build(),
        );
        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "postmaster");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::IpAddress,
                address: "123.123.123.123".to_owned()
            }
        );
    }

    #[test]
    fn ipv6_address_is_valid() {
        let email = EmailAddress::try_parse(
            "postmaster@[IPv6:2001:0db8:85a3:0000:0000:8a2e:0370:7334]",
            &ValidationOptionsBuilder::default()
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "postmaster");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::IpAddress,
                address: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_owned()
            }
        );
    }

    #[test]
    fn hostname_only_with_ipaddress_will_fail() {
        let email = EmailAddress::try_parse(
            "postmaster@[123.123.123.123]",
            &ValidationOptionsBuilder::default()
                .with_domain_support(DomainSupport::HostNameOnly)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::DomainParseError(DomainParseError::UnsupportedDomainType)
        );
    }

    #[test]
    fn hostname_or_local_domain_with_ipaddress_will_fail() {
        let email = EmailAddress::try_parse(
            "postmaster@[123.123.123.123]",
            &ValidationOptionsBuilder::default()
                .with_domain_support(DomainSupport::LocalAndHostName)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::DomainParseError(DomainParseError::UnsupportedDomainType)
        );
    }

    #[test]
    fn domain_label_length_exceeds_max_length_fails() {
        let email = EmailAddress::try_parse(
            "test.user@longer-than.accepted.domain.com",
            &ValidationOptionsBuilder::default()
                .with_max_dns_length(10)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::DomainParseError(DomainParseError::DnsLabelTooLong {
                max_dns_length: 10
            })
        );
    }

    #[test]
    fn domain_label_length_exceeds_max_length_in_the_middle_fails() {
        let email = EmailAddress::try_parse(
            "test.user@test.longer-than.accepted.domain.com",
            &ValidationOptionsBuilder::default()
                .with_max_dns_length(10)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::DomainParseError(DomainParseError::DnsLabelTooLong {
                max_dns_length: 10
            })
        );
    }

    #[test]
    fn non_ascii_characters_in_domain_label_is_valid() {
        let email = EmailAddress::try_parse(
            "I❤️CHOCOLATE@exa❤️mple.com",
            &ValidationOptionsBuilder::default()
                .with_text_encoding(TextEncoding::Utf8)
                .build(),
        );

        assert!(email.is_ok());
        let err = email.unwrap();
        assert_eq!(err.local_part, "I❤️CHOCOLATE");
        assert_eq!(
            err.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "exa❤️mple.com".to_owned(),
            }
        );
    }

    #[test]
    fn local_domain_when_supported_is_valid() {
        let supported_types = [DomainSupport::All, DomainSupport::LocalAndHostName];
        for support in supported_types {
            let email = EmailAddress::try_parse(
                "postmaster@example",
                &ValidationOptionsBuilder::default()
                    .with_domain_support(support)
                    .build(),
            );

            assert!(email.is_ok());
            let email = email.unwrap();
            assert_eq!(email.local_part, "postmaster");
            assert_eq!(
                email.domain,
                Domain {
                    domain_type: DomainType::LocalDomain,
                    address: "example".to_owned()
                }
            );
        }
    }

    #[test]
    fn local_domain_when_not_supported_is_invalid() {
        let unsupported_types = [
            DomainSupport::HostNameOnly,
            DomainSupport::IpAddressAndHostName,
        ];

        for support in unsupported_types {
            let email = EmailAddress::try_parse(
                "postmaster@example",
                &ValidationOptionsBuilder::default()
                    .with_domain_support(support)
                    .build(),
            );

            assert!(email.is_err());
            assert_eq!(
                email.unwrap_err(),
                EmailParseError::DomainParseError(DomainParseError::UnsupportedDomainType)
            );
        }
    }

    #[test]
    fn local_part_can_be_quoted_when_supported() {
        let email = EmailAddress::try_parse(
            "\"post master\"@example.com",
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Allowed)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "\"post master\"");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned()
            }
        );
    }

    #[test]
    fn local_part_can_escape_quoted_when_supported() {
        let email = EmailAddress::try_parse(
            "\"post\\\" master\"@example.com",
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Allowed)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "\"post\\\" master\"");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned()
            }
        );
    }

    #[test]
    fn local_part_cannot_escape_anything_but_quotes_when_supported() {
        let email = EmailAddress::try_parse(
            "\"post\\ master\"@example.com",
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Allowed)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::InvalidEscapeSequence {
                sequence: "\\ ".to_owned()
            }
        );
    }

    #[test]
    fn local_part_quotes_must_be_escaped_when_supported() {
        let email = EmailAddress::try_parse(
            "\"post\" master\"@example.com",
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Allowed)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::InvalidEscapeSequence {
                sequence: "\"".to_owned()
            }
        );
    }

    #[test]
    fn local_part_quotes_can_not_end_escaped_when_supported() {
        let email = EmailAddress::try_parse(
            "\"post master\\\"@example.com",
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Allowed)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::InvalidEscapeSequence {
                sequence: "\\".to_owned()
            }
        );
    }

    #[test]
    fn local_part_cannot_be_quoted_when_not_supported() {
        let email = EmailAddress::try_parse(
            "\"post master\"@example.com",
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Disallowed)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(
            email.unwrap_err(),
            EmailParseError::InvalidCharacters {
                character_set: SPECIAL_CHARS.to_owned()
            }
        );
    }

    #[test]
    fn domain_cannot_be_quoted() {
        let email = EmailAddress::try_parse(
            "postmaster@\"example.com\"",
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Allowed)
                .build(),
        );

        assert!(email.is_err());
        assert_eq!(email.unwrap_err(), EmailParseError::MissingDomain);
    }

    #[test]
    fn double_dot_is_supported_when_quoted() {
        let email = EmailAddress::try_parse(
            "\"john..doe\"@example.org",
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Allowed)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "\"john..doe\"");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.org".to_owned()
            }
        );
    }

    #[test]
    fn printable_characters_are_supported() {
        for c in VALID_CHARS.chars() {
            let email = EmailAddress::try_parse(
                &format!("test{c}user@example.com"),
                &EmailValidationOptions::default(),
            );
            assert!(email.is_ok());
            let email = email.unwrap();
            assert_eq!(email.local_part, format!("test{c}user"));
            assert_eq!(
                email.domain,
                Domain {
                    domain_type: DomainType::HostName,
                    address: "example.com".to_owned()
                }
            );
        }
    }

    #[test]
    fn just_spaces_is_valid_when_quoted() {
        let email = EmailAddress::try_parse(
            "\" \"@example.com",
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Allowed)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "\" \"");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned()
            }
        );
    }

    #[test]
    fn unusual_characters_are_valid_when_quoted() {
        let email = EmailAddress::try_parse(
            r#""very.(),:;<>[]\".VERY.\"very@\\ \"very\".unusual"@strange.example.com"#,
            &ValidationOptionsBuilder::default()
                .with_allow_quoted_strings(QuotedSupport::Allowed)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(
            email.local_part,
            r#""very.(),:;<>[]\".VERY.\"very@\\ \"very\".unusual""#
        );
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "strange.example.com".to_owned()
            }
        );
    }

    #[test]
    fn trailing_comments_are_ignored_in_domain_part() {
        let email = EmailAddress::try_parse(
            "test@example.com(comments ❤️ is@doesn't matter)",
            &ValidationOptionsBuilder::default()
                .with_comments(CommentSupport::Trailing)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "test");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned()
            }
        );
    }

    #[test]
    fn leading_comments_are_ignored_in_domain_part() {
        let email = EmailAddress::try_parse(
            "test@(comments ❤️ is@doesn't matter)example.com",
            &ValidationOptionsBuilder::default()
                .with_comments(CommentSupport::Leading)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "test");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned()
            }
        );
    }

    #[test]
    fn leading_and_trailing_comments_are_ignored_in_domain_part() {
        let email = EmailAddress::try_parse(
            "test@(comments ❤️ is@doesn't matter)example.com(comments ❤️ is@doesn't matter)",
            &ValidationOptionsBuilder::default()
                .with_comments(CommentSupport::LeadingAndTrailing)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "test");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned()
            }
        );
    }

    #[test]
    fn trailing_comments_are_ignored_in_local_part() {
        let email = EmailAddress::try_parse(
            "test(comments ❤️ is@doesn't matter)@example.com",
            &ValidationOptionsBuilder::default()
                .with_comments(CommentSupport::Trailing)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "test");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned()
            }
        );
    }

    #[test]
    fn leading_comments_are_ignored_in_local_part() {
        let email = EmailAddress::try_parse(
            "(comments ❤️ is@doesn't matter)test@example.com",
            &ValidationOptionsBuilder::default()
                .with_comments(CommentSupport::Leading)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "test");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned()
            }
        );
    }

    #[test]
    fn leading_and_trailing_comments_are_ignored_in_local_part() {
        let email = EmailAddress::try_parse(
            "(comments ❤️ is@doesn't matter)test(comments ❤️ is@doesn't matter)@example.com",
            &ValidationOptionsBuilder::default()
                .with_comments(CommentSupport::LeadingAndTrailing)
                .build(),
        );

        assert!(email.is_ok());
        let email = email.unwrap();
        assert_eq!(email.local_part, "test");
        assert_eq!(
            email.domain,
            Domain {
                domain_type: DomainType::HostName,
                address: "example.com".to_owned()
            }
        );
    }
}
