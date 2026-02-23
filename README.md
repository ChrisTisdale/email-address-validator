# Email Address Validator

A simple email address validator written in Rust.

## Required Setup

To build this project, you will need to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).

## Building

The application can be built by running `cargo build --release --all-features --all-targets`.

All the tools will be built under the `target/release` folder and can be run

## Contributing

All checkins must pass the following checks:

- clippy check. To validate that clippy passes run:
  - `cargo clippy --all-features --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings`.
  - `cargo clippy --no-default-features --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings`.

- cargo format check. To validate that the rust formatter passes run `cargo fmt --all -- --check`
- no std support. To validate that the project can be built without the standard library run `cargo build --release --no-default-features --all-targets`
