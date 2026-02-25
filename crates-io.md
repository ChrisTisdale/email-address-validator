# Email Address Validator

A email address validator is a tool that checks whether a given string is a valid email address according to the
standard email address format. This is useful for validating user input in applications that require email addresses,
such as registration forms or contact forms.

## Getting Started

```commandline
cargo add email_address_validator
```

## Working with this crate

```rust
use email_address_validator::{
    Domain, DomainValidationOptionsBuilder, EmailAddress, EmailParseError,
    ValidationOptionsBuilder
};

fn main() -> Result<(), EmailParseError> {
    let email = EmailAddress::try_parse(
        "Testing User <test.user@example.com>",
        &ValidationOptionsBuilder::new().build()
    )?;

    // Prints Parsed Email: Testing User <test.user@example.com>
    println!("Parsed Email: {}", email);

    // Prints Local Part: test.user
    println!("Local Part: {}", email.local_part());

    // Prints Domain Part: example.com
    println!("Domain Part: {}", email.domain());

    let domain = Domain::try_parse(
        "[1234:5678:9abc:def0:1234:5678:9abc:def0]",
        &DomainValidationOptionsBuilder::new().build()
    )?;

    // Prints Domain: [1234:5678:9abc:def0:1234:5678:9abc:def0]
    println!("Domain: {}", domain.address());

    // Prints Domain Type: IpAddress
    println!("Domain Type: {}", domain.domain_type());

    Ok(())
}
```
