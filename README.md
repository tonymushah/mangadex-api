# mangadex-api

## Important

This git repo is just a fork from [gondolyr/mangadex-api](https://gitlab.com/gondolyr/mangadex-api) but the project and crate has been yanked so I will now maintain this crate for [special-eureka](https://github.com/tonymushah/special-eureka) and [eureka-manager](https://github.com/tonymushah/eureka-mmanager)

The `mangadex-api` crate provides a convenient, high-level wrapper
[client][library-client] for the [MangaDex API][mangadex-api-url],
written in [Rust][rust-homepage].

It covers all public endpoints covered by [their documentation][mangadex-api-docs-url].

[Documentation (docs.rs)](https://docs.rs/mangadex_api)

[Documentation (Project `main` branch)](https://gondolyr.gitlab.io/mangadex-api/mangadex_api)

Please note that as MangaDex is still in beta, this SDK will be subject to sudden breaking changes.

## Disclaimer

`mangadex-api` is not affiliated with [MangaDex][mangadex-homepage].

# Table of Contents

- [mangadex-api](#mangadex-api)
  - [Important](#important)
  - [Disclaimer](#disclaimer)
- [Table of Contents](#table-of-contents)
- [Requirements](#requirements)
- [How to install](#how-to-install)
- [Dependency Justification](#dependency-justification)
- [Features](#features)
- [HTTP Client](#http-client)
- [Response Structs](#response-structs)
- [Getting Started](#getting-started)
- [Using a custom reqwest Client](#using-a-custom-reqwest-client)
- [Searching manga by title](#searching-manga-by-title)
- [Searching manga by title with reference expansion](#searching-manga-by-title-with-reference-expansion)
- [Downloading chapter pages](#downloading-chapter-pages)
- [Downloading a manga's main cover image](#downloading-a-mangas-main-cover-image)
- [Changelog](#changelog)
- [License](#license)
  - [Contribution](#contribution)
- [Contributing](#contributing)

# Requirements

[Back to top][readme-section-toc]

- [Rust 1.56+][rust-homepage]

# How to install

[Back to top][readme-section-toc]

Add `mangadex-api` to your dependencies:

```toml
[dependencies]
# ...
# Types and schemas are always required
mangadex-api-types-rust = "0.3.3"
mangadex-api-schema-rust = "0.3.2"
mangadex-api = "2.0.2"
```

If you are using [`cargo-edit`](https://github.com/killercup/cargo-edit), run

```bash
cargo add mangadex-api
```

# Dependency Justification

| Dependency                                         | Used for                                                                                                                                 | Included   |
|:---------------------------------------------------|:-----------------------------------------------------------------------------------------------------------------------------------------|:-----------|
| [`anyhow`][dependency-anyhow-docs]                 | Capturing unexpected errors.                                                                                                             | always     |
| [`mangadex-api-types-rust`][dependency-mangadex-api-types]                 | Enums and static data for Mangadex API                                                                                                              | always     |
| [`mangadex-api-schema-rust`][dependency-mangadex-api-schema]                 | Types used for Mangadex API                                                                                                              | always     |
| [`clap`][dependency-clap-docs]                     | Examples demonstrating the library's capabilities                                                                                        | dev builds |
| [`derive_builder`][dependency-derive_builder-docs] | Conveniently generating setters for the API endpoint builders.                                                                           | always     |
| [`fake`][dependency-fake-docs]                     | Generating random data for unit tests.                                                                                                   | dev builds |
| [`futures`][dependency-futures-docs]               | Async request processing.                                                                                                                | always     |
| [`reqwest`][dependency-reqwest-docs]               | Making HTTP requests to the [MangaDex API][mangadex-api-url].                                                                            | always     |
| [`serde`][dependency-serde-docs]                   | Se/dese/rializing HTTP response bodies into structs.                                                                                     | always     |
| [`serde_json`][dependency-serde_json-docs]         | Creating JSON objects for unit tests.                                                                                                    | dev builds |
| [`serde_qs`][dependency-serde_qs-docs]             | Query string serialization for HTTP requests.                                                                                            | always     |
| [`thiserror`][dependency-thiserror-docs]           | Customized error handling.                                                                                                               | always     |
| [`time`][dependency-time-docs]                     | Convenience types for handing time fields.                                                                                               | always     |
| [`tokio`][dependency-tokio-docs]                   | Async runtime to handle futures (not the library) in the examples.                                                                       | dev builds |
| [`url`][dependency-url-docs]                       | Convenient `Url` type for validating and containing URLs.                                                                                | always     |
| [`uuid`][dependency-uuid-docs]                     | Convenient `Uuid` type for validating and containing UUIDs for requests and responses. Also used to randomly generate UUIDs for testing. | always     |
| [`wiremock`][dependency-wiremock-docs]             | HTTP mocking to test the [MangaDex API][mangadex-api-url].                                                                               | dev builds |

# Features

[Back to top][readme-section-toc]

All features are not included by default. To enable them, add any of the following to your project's `Cargo.toml` file.

- `multi-thread`

  Enable the `MangaDexClient` to be thread-safe, at the cost of operations being slightly more expensive.

For example, to enable the `multi-thread` feature, add the following to your `Cargo.toml` file:

```toml
mangadex-api = { version = "2.0.2", features = ["multi-thread"] }
```

# HTTP Client

[Back to top][readme-section-toc]

The [`mangadex_api::MangaDexClient`][library-client] is asynchronous, using
[`reqwest`][reqwest] as the HTTP client.

# Response Structs

[Back to top][readme-section-toc]

The response structs can be found in the [`schemas` module][library-schema-module] and contain the fields in a JSON response.

# Getting Started

[Back to top][readme-section-toc]

This example demonstrates how to fetch a random manga.

```rust
use mangadex_api::v5::MangaDexClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let random_manga = client
        .manga()
        .random()
        .build()?
        .send()
        .await?;

    println!("{:?}", random_manga);

    Ok(())
}
```

# Using a custom reqwest Client

[Back to top][readme-section-toc]

By default, [`mangadex_api::MangaDexClient`][library-client] will use the default
[`reqwest::Client`][reqwest-client] settings.

You may provide your own [`reqwest::Client`][reqwest-client] to customize options such as the
request timeout.

```rust
use reqwest::Client;

use mangadex_api::v5::MangaDexClient;

# async fn run() -> anyhow::Result<()> {
let reqwest_client = Client::builder()
    .timeout(std::time::Duration::from_secs(10))
    .build()?;

let client = MangaDexClient::new(reqwest_client);
# Ok(())
# }
```

# Searching manga by title

[Back to top][readme-section-toc]

Reference: <https://api.mangadex.org/swagger.html#/Manga/get-search-manga>

```rust
use mangadex_api::v5::MangaDexClient;

# async fn run() -> anyhow::Result<()> {
let client = MangaDexClient::default();

let manga_results = client
    .manga()
    .search()
    .title("full metal")
    .build()?
    .send()
    .await?;

println!("manga results = {:?}", manga_results);
# Ok(())
# }
```

# Searching manga by title with reference expansion

[Back to top][readme-section-toc]

Every fetch will include all relationships but with minimal information such as the relationship type and ID. Reference expansion will include the full JSON object in the results for the types that are added to the request.

In the example below, any associated authors in the list of relationships will provide detailed information such as the author's name, biography, and website in the results.

References:

- <https://api.mangadex.org/docs/reference-expansion/>
- Endpoint: <https://api.mangadex.org/swagger.html#/Manga/get-search-manga>
- Author object: <https://api.mangadex.org/swagger.html#/Author/get-author-id>

```rust
use mangadex_api::types::{ReferenceExpansionResource, RelationshipType};
use mangadex_api::v5::schema::RelatedAttributes;
use mangadex_api::v5::MangaDexClient;

# async fn run() -> anyhow::Result<()> {
let client = MangaDexClient::default();

let manga_results = client
    .manga()
    .search()
    .title("full metal")
    .include(&ReferenceExpansionResource::Author)
    .build()?
    .send()
    .await?;

println!("manga results = {:?}", manga_results);

let authors = manga_results.data.iter().filter_map(|manga| {
    for rel in &manga.relationships {
        if rel.type_ == RelationshipType::Author {
            return Some(rel);
        }
    }

    None
});

for author in authors {
    if let Some(RelatedAttributes::Author(author_attributes)) = &author.attributes {
        println!("{} - {}", author.id, author_attributes.name);
    }
}
# Ok(())
# }
```

# Downloading chapter pages

[Back to top][readme-section-toc]

Reference: <https://api.mangadex.org/docs/reading-chapter/>

```rust
// Imports used for downloading the pages to a file.
// They are not used because we're just printing the raw bytes.
// use std::fs::File;
// use std::io::Write;

use reqwest::Url;
use uuid::Uuid;

use mangadex_api::v5::MangaDexClient;

# async fn run() -> anyhow::Result<()> {
let client = MangaDexClient::default();

let chapter_id = Uuid::new_v4();

let at_home = client
    .at_home()
    .server()
    .chapter_id(&chapter_id)
    .build()?
    .send()
    .await?;

let http_client = reqwest::Client::new();

// Original quality. Use `.data.attributes.data_saver` for smaller, compressed images.
let page_filenames = at_home.chapter.data;
for filename in page_filenames {
    // If using the data-saver option, use "/data-saver/" instead of "/data/" in the URL.
    let page_url = at_home
        .base_url
        .join(&format!(
            "/{quality_mode}/{chapter_hash}/{page_filename}",
            quality_mode = "data",
            chapter_hash = at_home.chapter.hash,
            page_filename = filename
        ))
        .unwrap();

    let res = http_client.get(page_url).send().await?;
    // The data should be streamed rather than downloading the data all at once.
    let bytes = res.bytes().await?;

    // This is where you would download the file but for this example,
    // we're just printing the raw data.
    // let mut file = File::create(&filename)?;
    // let _ = file.write_all(&bytes);
    println!("Chunk: {:?}", bytes);
}

# Ok(())
# }
```

# Downloading a manga's main cover image

[Back to top][readme-section-toc]

While this example could directly get the cover information by passing in the cover ID,
it is not often that one would have the ID off-hand, so the most common method would be from a
manga result.

If you want to get all of a manga's cover images, you will need to use the [cover list endpoint](https://api.mangadex.org/swagger.html#/Cover/get-cover)
and use the `manga[]` query parameter.

```rust
// Imports used for downloading the cover to a file.
// They are not used because we're just printing the raw bytes.
// use std::fs::File;
// use std::io::Write;

use reqwest::Url;
use uuid::Uuid;

use mangadex_api::types::RelationshipType;
use mangadex_api::v5::MangaDexClient;
use mangadex_api::CDN_URL;

# async fn run() -> anyhow::Result<()> {
let client = MangaDexClient::default();

let manga_id = Uuid::new_v4();
let manga = client
    .manga()
    .get()
    .manga_id(&manga_id)
    .build()?
    .send()
    .await?;

let cover_id = manga
    .data
    .relationships
    .iter()
    .find(|related| related.type_ == RelationshipType::CoverArt)
    .expect("no cover art found for manga")
    .id;
let cover = client
    .cover()
    .get()
    .cover_id(&cover_id)
    .build()?
    .send()
    .await?;

// This uses the best quality image.
// To use smaller, thumbnail-sized images, append any of the following:
//
// - .512.jpg
// - .256.jpg
//
// For example, "https://uploads.mangadex.org/covers/8f3e1818-a015-491d-bd81-3addc4d7d56a/4113e972-d228-4172-a885-cb30baffff97.jpg.512.jpg"
let cover_url = Url::parse(&format!(
        "{}/covers/{}/{}",
        CDN_URL, manga_id, cover.data.attributes.file_name
    ))
    .unwrap();

let http_client = reqwest::Client::new();

let res = http_client.get(cover_url).send().await?;
// The data should be streamed rather than downloading the data all at once.
let bytes = res.bytes().await?;

// This is where you would download the file but for this example, we're just printing the raw data.
// let mut file = File::create(&filename)?;
// let _ = file.write_all(&bytes);
println!("Chunk: {:?}", bytes);
# Ok(())
# }
```

# Changelog

[Back to top][readme-section-toc]

The changelog can be found [here][changelog].

Changes are added manually to keep the changelog human-readable with summaries of the changes from each version.

# License

[Back to top][readme-section-toc]

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE][license-apache] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT][license-mit] or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

[Back to top][readme-section-toc]

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# Contributing

[Back to top][readme-section-toc]

We welcome contributions from everyone. There are many ways to contribute and the
[CONTRIBUTING.md][contributing] document explains how you can contribute and get started.

[dependency-anyhow-docs]: https://docs.rs/anyhow
[dependency-mangadex-api-types]: https://github.com/tonymushah/mangadex-api/tree/main/mangadex-api-types
[dependency-mangadex-api-schema]: https://github.com/tonymushah/mangadex-api/tree/main/mangadex-api-schema
[dependency-clap-docs]: https://docs.rs/clap
[dependency-fake-docs]: https://docs.rs/fake
[dependency-derive_builder-docs]: https://docs.rs/derive_builder
[dependency-futures-docs]: https://docs.rs/futures
[dependency-reqwest-docs]: https://docs.rs/reqwest
[dependency-serde-docs]: https://docs.rs/serde
[dependency-serde_json-docs]: https://docs.rs/serde_json
[dependency-serde_qs-docs]: https://docs.rs/serde_qs
[dependency-thiserror-docs]: https://docs.rs/thiserror
[dependency-time-docs]: https://docs.rs/time
[dependency-tokio-docs]: https://docs.rs/tokio
[dependency-url-docs]: https://docs.rs/url
[dependency-uuid-docs]: https://docs.rs/uuid
[dependency-wiremock-docs]: https://docs.rs/wiremock

[mangadex-api-url]: https://api.mangadex.org
[mangadex-api-docs-url]: https://api.mangadex.org/swagger.html
[mangadex-homepage]: https://mangadex.org
[reqwest]: https://docs.rs/reqwest
[reqwest-client]: https://docs.rs/reqwest/latest/reqwest/struct.Client.html
[rust-homepage]: https://rust-lang.org

[changelog]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/CHANGELOG.md
[contributing]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/CONTRIBUTING.md
[library-client]: ./v5/struct.MangaDexClient.html
[library-schema-module]: ./v5/schema/index.html
[license-apache]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/LICENSE-APACHE
[license-mit]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/LICENSE-MIT

[readme-section-toc]: #table-of-contents
