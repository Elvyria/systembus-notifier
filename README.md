# systembus-notifier
[![Crates.io](https://img.shields.io/crates/v/systembus-notifier.svg)](https://crates.io/crates/systembus-notifier)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Listener for D-Bus, that picks system notifications (`net.nuetzlich.SystemNotifications`) and redirects them to the user.  
Unlike [systembus-notify](https://github.com/rfjakob/systembus-notify), written in rust and doesn't depend on `libsystemd`.

## Usage

If your user doesn't somehow have access to system bus, you should run `systembus-notifier` as root.

## Installation

Can be installed with `cargo`:

```bash
cargo install systembus-notifier
```

## Building

To build this little thing, you'll need some [Rust](https://www.rust-lang.org/).

```bash
git clone https://github.com/Elvyria/systembus-notifier
cd systembus-notifier
cargo build --release
```

## TODO

- [ ] Multiple users.
