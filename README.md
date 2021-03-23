# angry-purple-tiger

animal-based hash digests for humans.. in rust

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![Build Status][actions-badge]][actions-url]
[![Discord chat][discord-badge]][discord-url]

[crates-badge]: https://img.shields.io/crates/v/angry-purple-tiger.svg
[crates-url]: https://crates.io/crates/angry-purple-tiger
[docs-badge]: https://docs.rs/angry-purple-tiger/badge.svg
[docs-url]: https://docs.rs/angry-purple-tiger/latest/angry-purple-tiger/
[actions-badge]: https://github.com/helium/angry-purple-tiger-rs/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/helium/angry-purple-tiger-rs/actions/workflows/rust.yml
[discord-badge]: https://img.shields.io/discord/500028886025895936.svg?logo=discord&style=flat-square
[discord-url]: https://discord.gg/helium

## Overview

Angry Purple Tiger generates animal-based hash diegests meant to be memorable
and human-readable. Angry Purple Tiger is apt for anthropomorphizing project
names, crypto addresses, UUIDs, or any complex string of characters that needs
to be displayed in a user interface.

## Example

```rust,no-run
use angry_purple_tiger::AnimalName;

let address = "112CuoXo7WCcp6GGwDNBo6H5nKXGH45UNJ39iEefdv2mwmnwdFt8";
let animal_name = known.parse::<AnimalName>().expect("animal name");
assert_eq!(animal_name, "feisty-glass-dalmatian")
```
