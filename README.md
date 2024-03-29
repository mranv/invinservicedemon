# invinservicedemon

**invinservicedemon** is a Rust application designed for monitoring and managing system services. It provides a service checking mechanism to inspect the status of various services on the system.

## Features

- Periodically checks the status of system services.
- Logs service status information.

## Build Checks

[![Rust](https://github.com/mranv/invinservicedemon/actions/workflows/rust-check.yml/badge.svg)](https://github.com/mranv/invinservicedemon/actions/workflows/rust-check.yml)

## Usage

To compile and run the application, follow these steps:

1. Ensure you have Rust installed. You can install it from [rustup](https://rustup.rs/).
2. Clone the repository to your local machine.
3. Navigate to the project directory.
4. Build the application using Cargo:

```bash
cargo build --release
```

5. Strip debug symbols from the binary:

```bash
strip -s target/release/invinservicedemon
```

6. Run the application:

```bash
cargo run
```

By default, the application will periodically check the status of system services and log the information.

### Customizing Output

You can customize the output of the application by modifying the logging configuration. By default, it outputs log messages with a severity level of "info" or higher. You can adjust the log level or change the output destination according to your preferences.

## Additional Commands

- **Cleaning**: To clean up the project directory, removing build artifacts, you can use the following command:

```bash
cargo clean
```

- **Creating Debian Package**: To create a Debian package (.deb) from the Cargo project, you can use `cargo deb`:

```bash
cargo deb
```

- **Creating Binary RPM Package**: To generate a binary RPM package (.rpm) from the Cargo project, you can use `cargo generate-rpm`:

```bash
cargo generate-rpm
```
```bash
cargo build --release && strip -s target/release/invinservicedemon && cargo deb && cargo generate-rpm
```
Please ensure to strip debug symbols and build the release version (`--release`) before generating packages.

## Dependencies

- [`env_logger`](https://crates.io/crates/env_logger): Logging implementation in Rust.
- [`serde_json`](https://crates.io/crates/serde_json): JSON serialization and deserialization.
- [`tokio`](https://crates.io/crates/tokio): Asynchronous runtime for Rust.
- [`log`](https://crates.io/crates/log): Logging facade for Rust applications.

## Installation

You can install the `cargo-deb` and `cargo-generate-rpm` tools to generate Debian and RPM packages from your Cargo project.

```bash
cargo install cargo-deb
cargo install cargo-generate-rpm
```
