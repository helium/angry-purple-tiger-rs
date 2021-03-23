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
[actions-badge]: https://github.com/helium/angry-purple-tiger-rs/workflows/CI/badge.svg
[actions-url]: https://github.com/helium/angry-purple-tiger-rs/actions?query=workflow%3ACI+branch%3Amain
[discord-badge]: https://img.shields.io/discord/500028886025895936.svg?logo=discord&style=flat-square
[discord-url]: https://discord.gg/helium

## Overview

The Helium API is a REST API service as defined by the
[blockhain-http](https://github.com/helium/blockchain-http) service. This
library attempts to wrap this API in an async, easy to use library that supports
the conventions as exposed by the API. This includes:

* Modular access to each of the main areas of the Helium API
* Support for lazily fetched paged responses

Contributions and helpful suggestions are [always
welcome](https://github.com/helium/helium-api-rs/issues)

## Example

Create a client to the default `api.helium.io` endpoint and ask for a given
account.

```rust,no-run
use helium_api::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    let account = accounts::get(
        &client,
        "13buBykFQf5VaQtv7mWj2PBY9Lq4i1DeXhg7C4Vbu3ppzqqNkTH",
    )
    .await?;
    println!("Account: {:?}", account);
    Ok(())
}
```

See the examples folder and unit tests for more examples.
