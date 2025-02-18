# link-local-address

`link-local-address` is a Rust library for managing link-local IPv4 addresses. It automatically scans for available addresses, avoiding conflicts and ensuring smooth local network communication.

## Features
- **Dynamic assignment**: Assigns link-local IPv4 addresses dynamically.
- **Network scanning**: Scans the network to find free addresses.
- **ARP-based conflict prevention**: Prevents conflicts using ARP-based probing.
- **Batch allocation**: Supports batch allocation for efficient address management.
- **Asynchronous and configurable**: Fully asynchronous and configurable.
- **Unix-only**: This crate is designed for Unix-based systems (Linux, macOS, BSD).


## Planned Features
- **IPv6 support**: Future versions will extend functionality to support link-local IPv6 addresses.

## When to Use
- Setting up ad-hoc or peer-to-peer networks
- Assigning IPs without relying on DHCP
- Avoiding IP conflicts in local networking environments

## CLI

This project includes a basic command-line interface that leverages the `link-local-address` library to scan for and assign link-local IPv4 addresses. The CLI allows users to dynamically assign addresses, perform network scans, and manage local network configurations without relying on DHCP.

### Features
- **Scan for free addresses**: The CLI tool scans the network to find available link-local IPv4 addresses.
- **Assign addresses**: Assigns link-local IPv4 addresses to interfaces without DHCP.
- **Configurable parameters**: Customize retries, timeouts, and batch sizes via command-line arguments.

The source code for the CLI tool can be found in the following [file](<https://github.com/skullim/link-local-address/blob/main/link-local-address-cli/src/main.rs>).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
  at your option.