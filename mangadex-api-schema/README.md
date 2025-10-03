# mangadex-api-schema-rust

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![Crates.io (recent)](https://img.shields.io/crates/dr/mangadex-api-schema-rust)][crates-url]

[crates-badge]: https://img.shields.io/crates/v/mangadex-api-schema-rust.svg
[crates-url]: https://crates.io/crates/mangadex-api-schema-rust
[docs-badge]: https://img.shields.io/docsrs/mangadex-api-schema-rust.svg
[docs-url]: https://docs.rs/mangadex-api-schema-rust

A collection of structs and schemas for the [mangadex-api](https://github.com/tonymushah/mangadex-api)

## Install

```toml
mangadex-api-schema-rust = "1"
```

## Features

`non_exhaustive` : put all enums to `non_exhaustive` mode _(enabled by default)_

`specta` : enable [`specta`](https://github.com/oscartbeaumont/specta) support

`serialize` : enable [`serde`](https://serde.rs/) serialize support
