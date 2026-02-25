/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//!
//! # Email Address Validator
//!
//! Email address validation library for Rust.
//!
//! The email address validation library provides a set of tools for validating email addresses according to the standard email address format.
//! It includes features such as parsing email addresses, validating domain names, and handling different types of email address formats.
//!
//! # Examples
//!
//! ```
//! use email_address_validator::{
//!     Domain, DomainValidationOptionsBuilder, EmailAddress, EmailParseError,
//!     ValidationOptionsBuilder
//! };
//!
//! fn main() -> Result<(), EmailParseError> {
//!     let email = EmailAddress::try_parse(
//!         "Testing User <test.user@example.com>",
//!         &ValidationOptionsBuilder::new().build()
//!     )?;
//!
//!     // Prints Parsed Email: Testing User <test.user@example.com>
//!     println!("Parsed Email: {}", email);
//!
//!     // Prints Local Part: test.user
//!     println!("Local Part: {}", email.local_part());
//!
//!     // Prints Domain Part: example.com
//!     println!("Domain Part: {}", email.domain());
//!
//!     let domain = Domain::try_parse(
//!         "[1234:5678:9abc:def0:1234:5678:9abc:def0]",
//!         &DomainValidationOptionsBuilder::new().build()
//!     )?;
//!
//!     // Prints Domain: 1234:5678:9abc:def0:1234:5678:9abc:def0
//!     println!("Domain: {}", domain.address());
//!
//!     // Prints Domain Type: IpAddress
//!     println!("Domain Type: {}", domain.domain_type());
//!
//!     Ok(())
//! }
//! ```
//!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod domain;
mod email;
mod error;
mod options;

pub use domain::{Domain, DomainType};
pub use email::EmailAddress;
pub use error::{DomainParseError, EmailParseError};
pub use options::{
    CommentSupport, DisplayNameSupport, DomainSupport, DomainValidationOptions,
    DomainValidationOptionsBuilder, EmailValidationOptions, QuotedSupport, TextEncoding,
    TrimWhitespace, ValidationOptionsBuilder,
};
