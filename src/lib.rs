/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

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
