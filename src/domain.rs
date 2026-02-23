/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::string::ToString;
#[cfg(not(feature = "std"))]
use core::fmt::Display;
#[cfg(not(feature = "std"))]
use core::fmt::Formatter;
#[cfg(not(feature = "std"))]
use core::net::IpAddr;
#[cfg(not(feature = "std"))]
use core::str::FromStr;

use crate::{DomainParseError, DomainSupport, DomainValidationOptions, TextEncoding};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use std::fmt::Display;
#[cfg(feature = "std")]
use std::fmt::Formatter;
#[cfg(feature = "std")]
use std::net::IpAddr;
#[cfg(feature = "std")]
use std::str::FromStr;

#[cfg(feature = "std")]
type FormatResult = std::fmt::Result;
#[cfg(not(feature = "std"))]
type FormatResult = core::fmt::Result;

static DEFAULT_VALIDATION_OPTIONS: DomainValidationOptions = DomainValidationOptions::new();

/// Represents a domain name, which can be either a hostname, local domain, or IP address.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
#[non_exhaustive]
pub enum DomainType {
    /// The domain is a hostname.
    HostName,
    /// The domain is a local domain.
    LocalDomain,
    /// The domain is an IP address.
    IpAddress,
}

/// Represents a domain name, which can be either a hostname, local domain, or IP address.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Domain {
    pub(crate) domain_type: DomainType,
    pub(crate) address: String,
}

impl Domain {
    ///
    /// Gets the domain type.
    ///
    /// # Arguments
    ///
    /// returns: Option<&`DomainType`> - The domain type.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{Domain, DomainType};
    /// use std::error::Error;
    /// use std::str::FromStr;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let email = Domain::from_str("example.com")?;
    ///     assert_eq!(email.domain_type(), &DomainType::HostName);
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub const fn domain_type(&self) -> &DomainType {
        &self.domain_type
    }

    ///
    /// Gets the domain address.
    ///
    /// # Arguments
    ///
    /// returns: Option<&str> - The domain address.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::Domain;
    /// use std::error::Error;
    /// use std::str::FromStr;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let email = Domain::from_str("example.com")?;
    ///     assert_eq!(email.address(), "example.com");
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub fn address(&self) -> &str {
        &self.address
    }

    ///
    /// Tries to parse a domain from a string.
    ///
    /// # Arguments
    ///
    /// * `str`: The string to parse into a domain
    /// * `option`: Validation options for the domain
    ///
    /// returns: Result<`Domain`, `DomainParseError`> - The parsed domain.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{Domain, DomainType, DomainValidationOptions};
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let domain = Domain::try_parse("test.com", &DomainValidationOptions::default())?;
    ///     assert_eq!(domain.domain_type(), &DomainType::HostName);
    ///     assert_eq!(domain.address(), "test.com");
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Failed to parse the domain. See `DomainParseError` for more details.
    ///
    pub fn try_parse(
        str: &str,
        option: &DomainValidationOptions,
    ) -> Result<Self, DomainParseError> {
        let mut text = option.trim_whitespace.trim_string(str);
        text = option.comments.trim(text);
        text = option.trim_whitespace.trim_string(text);
        if text.len() > option.max_length {
            return Err(DomainParseError::DomainLengthExceeded {
                length: text.len(),
                max_length: option.max_length,
            });
        }

        if text.ends_with(']') && text.starts_with('[') {
            return Self::parse_ip_domain(text, option);
        }

        let mut is_hostname = false;
        let mut last_dns_label_index = 0;
        for (i, c) in text.char_indices() {
            if c == '.' {
                is_hostname = true;
                last_dns_label_index = i + 1;
            } else if c != '-'
                && (c < '0' || c > '9' && c < 'A' || c > 'Z' && c < 'a' || c > 'z')
                && !(option.text_encoding == TextEncoding::Utf8 && c > '')
            {
                return Err(DomainParseError::InvalidCharacters {
                    character_set: String::from(c),
                });
            } else if i - last_dns_label_index >= option.max_dns_length {
                return Err(DomainParseError::DnsLabelTooLong {
                    max_dns_length: option.max_dns_length,
                });
            }
        }

        if is_hostname {
            return Ok(Self {
                domain_type: DomainType::HostName,
                address: text.to_owned(),
            });
        }

        match option.domain_support {
            DomainSupport::All | DomainSupport::LocalAndHostName => Ok(Self {
                domain_type: DomainType::LocalDomain,
                address: text.to_owned(),
            }),
            DomainSupport::IpAddressAndHostName | DomainSupport::HostNameOnly => {
                Err(DomainParseError::UnsupportedDomainType)
            }
        }
    }

    fn parse_ip_domain(
        str: &str,
        option: &DomainValidationOptions,
    ) -> Result<Self, DomainParseError> {
        match option.domain_support {
            DomainSupport::HostNameOnly | DomainSupport::LocalAndHostName => {
                Err(DomainParseError::UnsupportedDomainType)
            }
            DomainSupport::IpAddressAndHostName | DomainSupport::All => {
                let addr = &str[1..str.len() - 1];
                if let Some(stripped) = addr.strip_prefix("IPv6:") {
                    let ip: IpAddr = stripped.parse()?;
                    if ip.is_ipv4() {
                        Err(DomainParseError::InvalidCharacters {
                            character_set: "IPv6:".to_owned(),
                        })
                    } else {
                        Ok(Self {
                            domain_type: DomainType::IpAddress,
                            address: stripped.to_owned(),
                        })
                    }
                } else {
                    let _ip: IpAddr = addr.parse()?;
                    Ok(Self {
                        domain_type: DomainType::IpAddress,
                        address: addr.to_owned(),
                    })
                }
            }
        }
    }
}

impl FromStr for Domain {
    type Err = DomainParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s, &DEFAULT_VALIDATION_OPTIONS)
    }
}

impl TryFrom<&str> for Domain {
    type Error = DomainParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_parse(value, &DEFAULT_VALIDATION_OPTIONS)
    }
}

impl TryFrom<String> for Domain {
    type Error = DomainParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_parse(&value, &DEFAULT_VALIDATION_OPTIONS)
    }
}

impl From<IpAddr> for Domain {
    fn from(value: IpAddr) -> Self {
        Self {
            domain_type: DomainType::IpAddress,
            address: value.to_string(),
        }
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self.domain_type {
            DomainType::HostName | DomainType::LocalDomain => write!(f, "{}", self.address),
            DomainType::IpAddress => write!(f, "[{}]", self.address),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DomainValidationOptionsBuilder, TrimWhitespace};

    #[test]
    fn test_host_name_domain_type() {
        let domain = Domain::try_parse(
            "example.com",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::All)
                .build(),
        )
        .unwrap();
        assert_eq!(domain.domain_type(), &DomainType::HostName);
        assert_eq!(domain.address(), "example.com");
    }

    #[test]
    fn test_local_domain_domain_type() {
        let domain = Domain::try_parse(
            "local",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::All)
                .build(),
        )
        .unwrap();
        assert_eq!(domain.domain_type(), &DomainType::LocalDomain);
        assert_eq!(domain.address(), "local");
    }

    #[test]
    fn test_ipv6_without_ipv6_prefix_domain_type() {
        let domain = Domain::try_parse(
            "[::1]",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::All)
                .build(),
        )
        .unwrap();
        assert_eq!(domain.domain_type(), &DomainType::IpAddress);
        assert_eq!(domain.address(), "::1");
    }

    #[test]
    fn test_ip_address_domain_type_with_ipv6_prefix() {
        let domain = Domain::try_parse(
            "[IPv6:::1]",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_ok());
        let domain = domain.unwrap();
        assert_eq!(domain.domain_type(), &DomainType::IpAddress);
        assert_eq!(domain.address(), "::1");
    }

    #[test]
    fn test_ipv4_address_can_be_parsed() {
        let domain = Domain::try_parse(
            "[127.0.0.1]",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_ok());
        let domain = domain.unwrap();
        assert_eq!(domain.domain_type(), &DomainType::IpAddress);
        assert_eq!(domain.address(), "127.0.0.1");
    }

    #[test]
    fn test_sub_domains_are_allowed() {
        let domain = Domain::try_parse(
            "sub.example.com",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_ok());
        let domain = domain.unwrap();
        assert_eq!(domain.domain_type(), &DomainType::HostName);
        assert_eq!(domain.address(), "sub.example.com");
    }

    #[test]
    fn ip_domain_with_invalid_characters_fails() {
        let domain = Domain::try_parse(
            "[IPv6:127.0.0.1]",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_err());
        let domain_error = domain.unwrap_err();
        assert_eq!(
            domain_error,
            DomainParseError::InvalidCharacters {
                character_set: String::from("IPv6:")
            }
        );
    }

    #[test]
    fn invalid_ip_address_fails() {
        let domain = Domain::try_parse(
            "[127.0.0.1.1]",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_err());
        let domain_error = domain.unwrap_err();
        assert!(matches!(
            domain_error,
            DomainParseError::IpParseError { .. }
        ));
    }

    #[test]
    fn fails_parsing_ip_when_not_supported() {
        let domain = Domain::try_parse(
            "[127.0.0.1]",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::HostNameOnly)
                .build(),
        );

        assert!(domain.is_err());
        let domain_error = domain.unwrap_err();
        assert_eq!(domain_error, DomainParseError::UnsupportedDomainType);
    }

    #[test]
    fn fails_parsing_local_domain_when_not_supported() {
        let domain = Domain::try_parse(
            "local",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::HostNameOnly)
                .build(),
        );

        assert!(domain.is_err());
        let domain_error = domain.unwrap_err();
        assert_eq!(domain_error, DomainParseError::UnsupportedDomainType);
    }

    #[test]
    fn invalid_characters_in_domain_name_fails() {
        let domain = Domain::try_parse(
            "test.com/test",
            &DomainValidationOptionsBuilder::new()
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_err());
        let domain_error = domain.unwrap_err();
        assert_eq!(
            domain_error,
            DomainParseError::InvalidCharacters {
                character_set: String::from("/")
            }
        );
    }

    #[test]
    fn utf8_characters_in_domain_name_are_allowed() {
        let domain = Domain::try_parse(
            "test.com❤️test",
            &DomainValidationOptionsBuilder::new()
                .with_text_encoding(TextEncoding::Utf8)
                .build(),
        );

        assert!(domain.is_ok());
        let domain = domain.unwrap();
        assert_eq!(domain.domain_type(), &DomainType::HostName);
        assert_eq!(domain.address(), "test.com❤️test");
    }

    #[test]
    fn utf8_characters_fails_parsing_when_not_supported() {
        let domain = Domain::try_parse(
            "test.com❤️test",
            &DomainValidationOptionsBuilder::new()
                .with_text_encoding(TextEncoding::Ascii)
                .build(),
        );

        assert!(domain.is_err());
        let domain_error = domain.unwrap_err();
        assert!(matches!(
            domain_error,
            DomainParseError::InvalidCharacters { .. }
        ));
    }

    #[test]
    fn domain_can_be_trimmed() {
        let domain = Domain::try_parse(
            "  test.com  ",
            &DomainValidationOptionsBuilder::new()
                .with_trim_whitespace(TrimWhitespace::Both)
                .build(),
        );

        assert!(domain.is_ok());
        let domain = domain.unwrap();
        assert_eq!(domain.domain_type(), &DomainType::HostName);
        assert_eq!(domain.address(), "test.com");
    }

    #[test]
    fn domain_parse_fails_with_max_length_exceeded() {
        let domain = Domain::try_parse(
            "very-long-domain",
            &DomainValidationOptionsBuilder::new()
                .with_max_length(15)
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_err());
        let domain_error = domain.unwrap_err();
        assert_eq!(
            domain_error,
            DomainParseError::DomainLengthExceeded {
                length: 16,
                max_length: 15
            }
        );
    }

    #[test]
    fn domain_at_max_length_succeeds() {
        let domain = Domain::try_parse(
            "very-long-domain",
            &DomainValidationOptionsBuilder::new()
                .with_max_length(16)
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_ok());
        let domain = domain.unwrap();
        assert_eq!(domain.domain_type(), &DomainType::LocalDomain);
        assert_eq!(domain.address(), "very-long-domain");
    }

    #[test]
    fn subdomain_parse_fails_with_max_dns_length_exceeded() {
        let domain = Domain::try_parse(
            "very-long-subdomain.very-long-domain.com",
            &DomainValidationOptionsBuilder::new()
                .with_max_dns_length(18)
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_err());
        let domain_error = domain.unwrap_err();
        assert_eq!(
            domain_error,
            DomainParseError::DnsLabelTooLong { max_dns_length: 18 }
        );
    }

    #[test]
    fn subdomain_at_max_dns_length_succeeds() {
        let domain = Domain::try_parse(
            "very-long-subdomain.very-long-domain.com",
            &DomainValidationOptionsBuilder::new()
                .with_max_dns_length(19)
                .with_domain_support(DomainSupport::All)
                .build(),
        );

        assert!(domain.is_ok());
        let domain = domain.unwrap();
        assert_eq!(domain.domain_type(), &DomainType::HostName);
        assert_eq!(domain.address(), "very-long-subdomain.very-long-domain.com");
    }
}
