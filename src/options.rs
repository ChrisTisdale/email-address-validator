/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg(not(feature = "std"))]
use core::fmt::Display;
#[cfg(not(feature = "std"))]
use core::fmt::Formatter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use std::fmt::Display;
#[cfg(feature = "std")]
use std::fmt::Formatter;

#[cfg(feature = "std")]
type FormatResult = std::fmt::Result;
#[cfg(not(feature = "std"))]
type FormatResult = core::fmt::Result;

/// Represents the text encoding format for email addresses.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum TextEncoding {
    /// ASCII encoding format.
    Ascii = 0,
    /// UTF-8 encoding format.
    #[default]
    Utf8,
}

impl Display for TextEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            Self::Ascii => write!(f, "Ascii"),
            Self::Utf8 => write!(f, "Utf8"),
        }
    }
}

/// Represents the comment support configuration for email addresses.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum CommentSupport {
    /// Comments are disallowed.
    #[default]
    Disallowed = 0,
    /// Comments are allowed only at the end of the string.
    Trailing,
    /// Comments are allowed at the beginning and end of the string.
    Leading,
    /// Comments are allowed at the beginning and end of the string, but not at the end of a parenthesized string.
    LeadingAndTrailing,
}

impl Display for CommentSupport {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            Self::Disallowed => write!(f, "Disallowed"),
            Self::Trailing => write!(f, "Trailing"),
            Self::Leading => write!(f, "Leading"),
            Self::LeadingAndTrailing => write!(f, "LeadingAndTrailing"),
        }
    }
}

impl CommentSupport {
    #[must_use]
    #[inline]
    pub(crate) fn trim(self, s: &str) -> &str {
        match self {
            Self::Disallowed => s,
            Self::Trailing => {
                if !s.ends_with(')') {
                    return s;
                }

                s.rfind('(').map_or(s, |pos| &s[..pos])
            }
            Self::Leading => {
                if !s.starts_with('(') {
                    return s;
                }

                s.find(')').map_or(s, |pos| &s[pos + 1..])
            }
            Self::LeadingAndTrailing => Self::Leading.trim(Self::Trailing.trim(s)),
        }
    }
}

/// Represents the display name support for email addresses.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum DisplayNameSupport {
    /// Display names are allowed.
    #[default]
    Allowed,
    /// Display names are disallowed.
    Disallowed,
}

impl Display for DisplayNameSupport {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            Self::Allowed => write!(f, "Allowed"),
            Self::Disallowed => write!(f, "Disallowed"),
        }
    }
}

/// Represents the trim whitespace configuration for email addresses.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum TrimWhitespace {
    /// No whitespace trimming is done.
    None = 0,
    /// Whitespace is trimmed from the start of the string.
    Start,
    /// Whitespace is trimmed from the end of the string.
    End,
    #[default]
    /// Whitespace is trimmed from both the start and end of the string.
    Both,
}

impl Display for TrimWhitespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match self {
            Self::None => write!(f, "None"),
            Self::Start => write!(f, "Start"),
            Self::End => write!(f, "End"),
            Self::Both => write!(f, "Both"),
        }
    }
}

impl TrimWhitespace {
    ///
    /// Trims whitespace from the given string based on the configured trimming mode.
    ///
    /// # Arguments
    ///
    /// * `s`: The input string to trim whitespace from.
    ///
    /// returns: &'a str - The trimmed string.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::TrimWhitespace;
    ///
    /// let trimmed = TrimWhitespace::Both.trim_string("   Hello World!   ");
    /// assert_eq!(trimmed, "Hello World!");
    /// ```
    #[must_use]
    #[inline]
    pub fn trim_string<'a>(&self, s: &'a str) -> &'a str {
        match self {
            Self::None => s,
            Self::Start => s.trim_start(),
            Self::End => s.trim_end(),
            Self::Both => s.trim(),
        }
    }
}

/// Represents the quoted support configuration for email addresses.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum QuotedSupport {
    /// Quoted strings are allowed.
    #[default]
    Allowed = 0,
    /// Quoted strings are disallowed.
    Disallowed,
}

/// Represents the domain support configuration for email addresses.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum DomainSupport {
    /// Only Host names are supported.
    HostNameOnly = 0,
    /// Only local domains and host names are supported.
    LocalAndHostName,
    /// Both IP addresses and host names are supported.
    #[default]
    IpAddressAndHostName,
    /// All domain types are supported.
    All,
}

/// Represents the domain validation options for email addresses.
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
#[non_exhaustive]
pub struct DomainValidationOptions {
    pub(crate) domain_support: DomainSupport,
    pub(crate) text_encoding: TextEncoding,
    pub(crate) comments: CommentSupport,
    pub(crate) trim_whitespace: TrimWhitespace,
    pub(crate) max_length: usize,
    pub(crate) max_dns_length: usize,
}

impl DomainValidationOptions {
    /// Creates a new instance of `DomainValidationOptions` with default settings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            domain_support: DomainSupport::IpAddressAndHostName,
            text_encoding: TextEncoding::Utf8,
            comments: CommentSupport::Disallowed,
            trim_whitespace: TrimWhitespace::Both,
            max_length: 255,
            max_dns_length: 63,
        }
    }
}

impl Default for DomainValidationOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the domain validation options for email addresses.
#[derive(Default, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
#[non_exhaustive]
pub struct DomainValidationOptionsBuilder {
    options: DomainValidationOptions,
}

impl DomainValidationOptionsBuilder {
    /// Creates a new instance of `DomainValidationOptionsBuilder` with default settings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            options: DomainValidationOptions::new(),
        }
    }

    ///
    /// Sets the text encoding for domain validation.
    ///
    /// # Arguments
    ///
    /// * `encoding`: The text encoding to use for domain validation.
    ///
    /// returns: A `DomainValidationOptionsBuilder` instance with the specified text encoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{DomainValidationOptionsBuilder, TextEncoding};
    ///
    /// let options = DomainValidationOptionsBuilder::new()
    ///     .with_text_encoding(TextEncoding::Ascii)
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_text_encoding(mut self, encoding: TextEncoding) -> Self {
        self.options.text_encoding = encoding;
        self
    }

    ///
    /// Sets the domain support for domain validation.
    ///
    /// # Arguments
    ///
    /// * `domain_support`: The domain support to use for domain validation.
    ///
    /// returns: A `DomainValidationOptionsBuilder` instance with the specified domain support.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{DomainSupport, DomainValidationOptionsBuilder};
    ///
    /// let options = DomainValidationOptionsBuilder::new()
    ///     .with_domain_support(DomainSupport::HostNameOnly)
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_domain_support(mut self, domain_support: DomainSupport) -> Self {
        self.options.domain_support = domain_support;
        self
    }

    ///
    /// Sets the trim whitespace option for domain validation.
    ///
    /// # Arguments
    ///
    /// * `trim_whitespace`: The trim whitespace option to use for domain validation.
    ///
    /// returns: A `DomainValidationOptionsBuilder` instance with the specified trim whitespace option.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{DomainValidationOptionsBuilder, TrimWhitespace};
    ///
    /// let options = DomainValidationOptionsBuilder::new()
    ///     .with_trim_whitespace(TrimWhitespace::Both)
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_trim_whitespace(mut self, trim_whitespace: TrimWhitespace) -> Self {
        self.options.trim_whitespace = trim_whitespace;
        self
    }

    ///
    /// Sets the maximum length for domain validation.
    ///
    /// # Arguments
    ///
    /// * `max_length`: The maximum length to use for domain validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::DomainValidationOptionsBuilder;
    ///
    /// let options = DomainValidationOptionsBuilder::new()
    ///     .with_max_length(100)
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_max_length(mut self, max_length: usize) -> Self {
        self.options.max_length = max_length;
        self
    }

    ///
    /// Sets the maximum length for dns
    ///
    /// # Arguments
    ///
    /// * `max_dns_length`: The maximum length to use for dns validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::DomainValidationOptionsBuilder;
    ///
    /// let options = DomainValidationOptionsBuilder::new()
    ///     .with_max_dns_length(100)
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_max_dns_length(mut self, max_dns_length: usize) -> Self {
        self.options.max_dns_length = max_dns_length;
        self
    }

    ///
    /// Builds the `DomainValidationOptions` instance with the specified settings.
    ///
    /// # Arguments
    ///
    /// returns: `DomainValidationOptions` - instance with the specified settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::DomainValidationOptionsBuilder;
    ///
    /// let options = DomainValidationOptionsBuilder::default()
    ///     .build();
    /// ```
    #[must_use]
    pub const fn build(self) -> DomainValidationOptions {
        self.options
    }
}

/// Represents the local part validation options for email addresses.
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
#[non_exhaustive]
pub struct LocalPartValidationOptions {
    pub(crate) text_encoding: TextEncoding,
    pub(crate) quoted_support: QuotedSupport,
    pub(crate) comments: CommentSupport,
    pub(crate) trim_whitespace: TrimWhitespace,
    pub(crate) max_length: usize,
}

impl LocalPartValidationOptions {
    /// Creates a new instance of `LocalPartValidationOptions` with default settings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            text_encoding: TextEncoding::Utf8,
            quoted_support: QuotedSupport::Allowed,
            comments: CommentSupport::Disallowed,
            trim_whitespace: TrimWhitespace::Both,
            max_length: 64,
        }
    }
}

impl Default for LocalPartValidationOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the local part validation options for email addresses.
#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone)]
#[non_exhaustive]
pub struct EmailValidationOptions {
    pub(crate) local_part_options: LocalPartValidationOptions,
    pub(crate) domain_options: DomainValidationOptions,
    pub(crate) display_name_support: DisplayNameSupport,
}

impl EmailValidationOptions {
    /// Creates a new instance of `EmailValidationOptions` with default settings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            local_part_options: LocalPartValidationOptions::new(),
            domain_options: DomainValidationOptions::new(),
            display_name_support: DisplayNameSupport::Allowed,
        }
    }
}

impl Default for EmailValidationOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents the local part validation options for files.
#[derive(Default, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
#[non_exhaustive]
pub struct ValidationOptionsBuilder {
    options: EmailValidationOptions,
}

impl ValidationOptionsBuilder {
    /// Creates a new instance of `ValidationOptionsBuilder` with default settings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            options: EmailValidationOptions::new(),
        }
    }

    ///
    /// Sets the domain support for domain validation.
    ///
    /// # Arguments
    ///
    /// * `options`: The domain validation options to use.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified domain validation options.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{DomainValidationOptions, ValidationOptionsBuilder};
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_domain_options(DomainValidationOptions::default())
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_domain_options(mut self, options: DomainValidationOptions) -> Self {
        self.options.domain_options = options;
        self
    }

    ///
    /// Sets the domain support for domain validation.
    ///
    /// # Arguments
    ///
    /// * `domain_support`: The domain support to use for domain validation.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified domain support.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{DomainSupport, ValidationOptionsBuilder};
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_domain_support(DomainSupport::default())
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_domain_support(mut self, domain_support: DomainSupport) -> Self {
        self.options.domain_options.domain_support = domain_support;
        self
    }

    ///
    /// Sets the display name support for email address validation.
    ///
    /// # Arguments
    ///
    /// * `display_name_support`: The display name support to use for email address validation.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified display name support.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{DisplayNameSupport, ValidationOptionsBuilder};
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_display_name_support(DisplayNameSupport::default())
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_display_name_support(
        mut self,
        display_name_support: DisplayNameSupport,
    ) -> Self {
        self.options.display_name_support = display_name_support;
        self
    }

    ///
    /// Sets the allow quoted strings option for email address validation.
    ///
    /// # Arguments
    ///
    /// * `quoted_support`: The allow quoted strings option to use for email address validation.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified allow quoted strings option.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{ValidationOptionsBuilder, QuotedSupport};
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_allow_quoted_strings(QuotedSupport::default())
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_allow_quoted_strings(mut self, quoted_support: QuotedSupport) -> Self {
        self.options.local_part_options.quoted_support = quoted_support;
        self
    }

    ///
    /// Sets the comments support for email address validation.
    ///
    /// # Arguments
    ///
    /// * `comments`: The comment support to use for email address validation.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified comments support.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{CommentSupport, ValidationOptionsBuilder};
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_comments(CommentSupport::default())
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_comments(mut self, comments: CommentSupport) -> Self {
        self.options.local_part_options.comments = comments;
        self.options.domain_options.comments = comments;
        self
    }

    ///
    /// Sets the text encoding support for email address validation.
    ///
    /// # Arguments
    ///
    /// * `encoding`: The text encoding to use for email address validation.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified text encoding support.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{TextEncoding, ValidationOptionsBuilder};
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_text_encoding(TextEncoding::default())
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_text_encoding(mut self, encoding: TextEncoding) -> Self {
        self.options.local_part_options.text_encoding = encoding;
        self.options.domain_options.text_encoding = encoding;
        self
    }

    ///
    /// Sets the trim whitespace option for email address validation.
    ///
    /// # Arguments
    ///
    /// * `trim_whitespace`: The trim whitespace option to use for email address validation.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified trim whitespace option.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::{TrimWhitespace, ValidationOptionsBuilder};
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_trim_whitespace(TrimWhitespace::default())
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_trim_whitespace(mut self, trim_whitespace: TrimWhitespace) -> Self {
        self.options.local_part_options.trim_whitespace = trim_whitespace;
        self.options.domain_options.trim_whitespace = trim_whitespace;
        self
    }

    ///
    /// Sets the maximum length for the domain part of an email address.
    ///
    /// # Arguments
    ///
    /// * `length`: The maximum length for the domain part of an email address.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified maximum domain length.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::ValidationOptionsBuilder;
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_max_domain_length(100)
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_max_domain_length(mut self, length: usize) -> Self {
        self.options.domain_options.max_length = length;
        self
    }

    ///
    /// Sets the maximum length for the local part of an email address.
    ///
    /// # Arguments
    ///
    /// * `length`: The maximum length for the local part of an email address.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified maximum local part length.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::ValidationOptionsBuilder;
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_max_local_part_length(100)
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_max_local_part_length(mut self, length: usize) -> Self {
        self.options.local_part_options.max_length = length;
        self
    }

    ///
    /// Sets the maximum length for dns in the domain part of an email address.
    ///
    /// # Arguments
    ///
    /// * `length`: The maximum length for dns in the domain part of an email address.
    ///
    /// returns: A `ValidationOptionsBuilder` instance with the specified maximum dns length.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::ValidationOptionsBuilder;
    ///
    /// let options = ValidationOptionsBuilder::new()
    ///     .with_max_dns_length(100)
    ///     .build();
    /// ```
    #[must_use]
    pub const fn with_max_dns_length(mut self, length: usize) -> Self {
        self.options.domain_options.max_dns_length = length;
        self
    }

    ///
    /// Builds the `ValidationOptions` instance with the specified settings.
    ///
    /// # Arguments
    ///
    /// returns: `ValidationOptions` - instance with the specified settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use email_address_validator::ValidationOptionsBuilder;
    ///
    /// let options = ValidationOptionsBuilder::default()
    ///     .build();
    /// ```
    #[must_use]
    pub const fn build(self) -> EmailValidationOptions {
        self.options
    }
}
