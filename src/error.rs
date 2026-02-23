/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use core::net::AddrParseError;

#[cfg(feature = "std")]
use std::net::AddrParseError;
#[cfg(feature = "std")]
use std::string::String;

use thiserror::Error;

/// Represents errors that can occur during domain parsing.
#[derive(Error, Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum DomainParseError {
    /// The complete domain name is longer than the maximum allowed length.
    #[error("Domain exceeds maximum length of {max_length}: {length}")]
    DomainLengthExceeded { length: usize, max_length: usize },
    /// The domain name contains characters that are not allowed.
    #[error("Invalid characters in domain: {character_set}")]
    InvalidCharacters { character_set: String },
    /// The domain name contains an invalid escape sequence.
    #[error("Unsported Domain Type")]
    UnsupportedDomainType,
    /// The domain name contains an invalid character sequence.
    #[error(transparent)]
    IpParseError(#[from] AddrParseError),
    /// The domain name contains an invalid character sequence.
    #[error("DNS Label exceeds maximum length of {max_dns_length} characters")]
    DnsLabelTooLong { max_dns_length: usize },
}

/// Represents errors that can occur during email address parsing.
#[derive(Error, Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum EmailParseError {
    /// The display name format is invalid.
    #[error("Invalid format for display name")]
    InvalidDisplayFormat,
    /// The email address contains characters that are not allowed.
    #[error("Invalid characters in email address: {character_set}")]
    InvalidCharacters { character_set: String },
    /// The email address contains an invalid escape sequence.
    #[error("Invalid escape sequence in quoted string: {sequence}")]
    InvalidEscapeSequence { sequence: String },
    /// The email address is missing a domain.
    #[error("Missing domain in email address")]
    MissingDomain,
    /// The email address is missing a local part.
    #[error("Missing local part in email address")]
    MissingLocalPart,
    /// The local part of the email address exceeds the maximum allowed length.
    #[error("Local part exceeds maximum length of {max_length}: {length}")]
    LocalPartLengthExceeded { length: usize, max_length: usize },
    /// The domain part of the email address fails to parse.
    #[error(transparent)]
    DomainParseError(#[from] DomainParseError),
}
