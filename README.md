# mangadex-api

[![Rust](https://github.com/tonymushah/mangadex-api/actions/workflows/rust.yml/badge.svg)](https://github.com/tonymushah/mangadex-api/actions/workflows/rust.yml)

## 3.0.0 developpement

A lot of changes will occur in 3.0.0. Please refer to [#27](https://github.com/tonymushah/mangadex-api/issues/27)

## Important

This git repo is just a fork from [gondolyr/mangadex-api](https://gitlab.com/gondolyr/mangadex-api) but the project and crate has been yanked so I will now maintain this crate for [special-eureka](https://github.com/tonymushah/special-eureka) and [eureka-manager](https://github.com/tonymushah/eureka-mmanager)

The `mangadex-api` crate provides a convenient, high-level wrapper
[client][library-client] for the [MangaDex API][mangadex-api-url],
written in [Rust][rust-homepage].

It covers all public endpoints covered by [their documentation][mangadex-api-docs-url].

[Documentation (docs.rs)](https://docs.rs/mangadex_api)

[Documentation (Project `main` branch)](https://gondolyr.gitlab.io/mangadex-api/mangadex_api)

Please note that as MangaDex is still in beta, this SDK will be subject to sudden breaking changes (a lot).

## Disclaimer

`mangadex-api` is not affiliated with [MangaDex][mangadex-homepage].

## Table of Contents

- [mangadex-api](#mangadex-api)
  - [3.0.0 developpement](#300-developpement)
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
  - [Using the old way](#using-the-old-way)
    - [Using the `utils` feature](#using-the-utils-feature)
      - [Via `(filename, Result<bytes>)` vector based](#via-filename-resultbytes-vector-based)
      - [Via `tokio-stream`](#via-tokio-stream)
        - [Without checker](#without-checker)
        - [with checker](#with-checker)
  - [Downloading a manga's main cover image](#downloading-a-mangas-main-cover-image)
    - [Use the legacy way](#use-the-legacy-way)
    - [Using the `utils` feature (recommended)](#using-the-utils-feature-recommended)
      - [via a cover id](#via-a-cover-id)
      - [via a manga id](#via-a-manga-id)
  - [Authentification (via the `oauth` feature)](#authentification-via-the-oauth-feature)
    - [Login](#login)
    - [Resfresh your token](#resfresh-your-token)
  - [Changelog](#changelog)
  - [License](#license)
    - [Contribution](#contribution)
  - [Contributing](#contributing)

## Requirements

[Back to top][readme-section-toc]

- [Rust 1.60+][rust-homepage]

## How to install

[Back to top][readme-section-toc]

Add `mangadex-api` to your dependencies:

```toml
[dependencies]
# ...
# Types and schemas are always required
mangadex-api-types-rust = "0.5"
mangadex-api-schema-rust = "0.5"
mangadex-api = "3.0.0-rc.1"
```

If you are using [`cargo-edit`](https://github.com/killercup/cargo-edit), run

```bash
cargo add mangadex-api
```

## Dependency Justification

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
| [`tokio`][dependency-tokio-docs]                   | Async runtime to handle futures in __(only)__ examples and `utils` feature in chapter reporting, `tokio-mutli-thread`, and `rw-mutli-thread`                                                                      | dev builds + `utils` features |
| [`url`][dependency-url-docs]                       | Convenient `Url` type for validating and containing URLs.                                                                                | always     |
| [`uuid`][dependency-uuid-docs]                     | Convenient `Uuid` type for validating and containing UUIDs for requests and responses. Also used to randomly generate UUIDs for testing. | always     |
| [`wiremock`][dependency-wiremock-docs]             | HTTP mocking to test the [MangaDex API][mangadex-api-url].                                                                               | dev builds |

## Features

[Back to top][readme-section-toc]

All features are not included by default. To enable them, add any of the following to your project's `Cargo.toml` file.

- `multi-thread`

  Enable the `MangaDexClient` to be thread-safe, at the cost of operations being slightly more expensive.

- `legacy-auth` *Deprecated*

  Enable the usage of the `< 5.9.0` login system in the SDK. Please visit the [Mangadex Discord](https://discord.com/invite/mangadex)  for more details

- `legacy-account` *Deprecated*

  Enable the usage of the `< 5.9.0` account management system in the SDK. Please visit the [Mangadex Discord](https://discord.com/invite/mangadex) for more details

- `utils`

  Enable the usage of the `MangaDexClient::download()`. Allows you to download chapters or covers image without tears and long code.

- `tokio-multi-thread`

  Enable the usage of [`tokio::sync::Mutex`](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html), instead of [`futures::lock::Mutex`](https://docs.rs/futures/0.3.29/futures/lock/struct.Mutex.html)

- `rw-mutli-thread`
  
  Enable the usage of [`tokio::sync::RwLock`](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html), instead of [`futures::lock::Mutex`](https://docs.rs/futures/0.3.29/futures/lock/struct.Mutex.html) in the client.
  It can be useful if you want a flexible concurent mutli-thread.

- `oauth` (Enabled by default)
  
  Enable the use of the *brand* new OAuth 2.0 login introduced in MangaDex API 5.9.0.
  
  __Quick Note:__ This `oauth` feature use the [personal-client] approach which means that you need to register a personal client and wait that it'll be validated.
  More details here [here](#authentification-via-the-oauth-feature)

For example, to enable the `multi-thread` feature, add the following to your `Cargo.toml` file:

```toml
mangadex-api = { version = "3.0.0-rc.1", features = ["multi-thread"] }
```

## HTTP Client

[Back to top][readme-section-toc]

The [`mangadex_api::MangaDexClient`][library-client] is asynchronous, using
[`reqwest`][reqwest] as the HTTP client.

## Response Structs

[Back to top][readme-section-toc]

The response structs can be found in the [`mangadex-api-schema-rust` crate][library-schema-module] and contain the fields in a JSON response.

## Getting Started

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
        .get()
        .send()
        .await?;

    println!("{:?}", random_manga);

    Ok(())
}
```

This example demonstates how to fetch the [Mangadex Popular Titles](https://mangadex.org/)

```rust
use mangadex_api::MangaDexClient;
use mangadex_api_schema_rust::v5::RelatedAttributes;
use mangadex_api_types_rust::{
    Language, MangaDexDateTime, MangaSortOrder, OrderDirection, ReferenceExpansionResource,
};
use time::{Duration, OffsetDateTime};
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    // Take the local date and put substract it with 30 days
    let created_at_since = OffsetDateTime::now_utc()
        .checked_sub(Duration::days(30))
        .unwrap();

    let created_at_since = MangaDexDateTime::new(&created_at_since);

    let res = client
        .manga()
        .get()
        // We pick up all manga that has been created during these last 30 days
        .created_at_since(created_at_since)
        // Mangadex Popular Titles is ordered by followedCount descending
        .order(MangaSortOrder::FollowedCount(OrderDirection::Descending))
        // We include the author data
        .include(ReferenceExpansionResource::Author)
        // We include the arstits data
        .include(ReferenceExpansionResource::Artist)
        .send()
        .await?;

    let not_found = String::from("Not found");
    // Just a simple index :3
    let mut index = 1;
    for manga in res.data {
        // Find the English title
        let title = manga
            .attributes
            .title
            .get(&Language::English)
            .unwrap_or(&not_found);
        println!("{index} ~ {title}");
        // Find the author name
        let author = manga
            .find_first_relationships(mangadex_api_types::RelationshipType::Author)
            .and_then(|e| {
                e.attributes.clone().map(|rel| match rel {
                    RelatedAttributes::Author(a) => a.name,
                    _ => not_found.clone(),
                })
            })
            .unwrap_or(not_found.clone());
        println!("\tAuthor: {author}");
        // Find the author name
        let artist = manga
            .find_first_relationships(mangadex_api_types::RelationshipType::Artist)
            .and_then(|e| {
                e.attributes.clone().map(|rel| match rel {
                    RelatedAttributes::Author(a) => a.name,
                    _ => not_found.clone(),
                })
            })
            .unwrap_or(not_found.clone());
        // Print the artist name if it's different of the author
        if artist != author {
            println!("\tArtist: {artist}");
        }
        // We generate the link that goes to the Mangadex page
        let title_link =
            Url::parse("https://mangadex.org/title/")?.join(manga.id.to_string().as_str())?;
        println!("\tLink: {title_link}");
        println!();
        index += 1;
    }
    println!("Done :3");
    Ok(())
}
```

## Using a custom reqwest Client

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

## Searching manga by title

[Back to top][readme-section-toc]

Reference: <https://api.mangadex.org/swagger.html#/Manga/get-search-manga>

```rust
use mangadex_api::v5::MangaDexClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let manga_results = client
        .manga()
        .get()
        .title("full metal")
        .send()
        .await?;

    println!("manga results = {:?}", manga_results);
    Ok(())
}
```

## Searching manga by title with reference expansion

[Back to top][readme-section-toc]

Every fetch will include all relationships but with minimal information such as the relationship type and ID. Reference expansion will include the full JSON object in the results for the types that are added to the request.

In the example below, any associated authors in the list of relationships will provide detailed information such as the author's name, biography, and website in the results.

References:

- <https://api.mangadex.org/docs/01-concepts/reference-expansion/>
- Endpoint: <https://api.mangadex.org/docs/swagger.html#/Manga/get-search-manga>
- Author object: <https://api.mangadex.org/docs/swagger.html#/Author/get-author-id>

```rust
use mangadex_api::v5::schema::RelatedAttributes;
use mangadex_api::v5::MangaDexClient;
// use mangadex_api_types_rust::{ReferenceExpansionResource, RelationshipType};
use mangadex_api_types::{ReferenceExpansionResource, RelationshipType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let manga_results = client
        .manga()
        .get()
        .title("full metal")
        .include(&ReferenceExpansionResource::Author)
        .send()
        .await?;

    println!("manga results = {:?}", manga_results);

    let authors = manga_results.data.iter().filter_map(|manga| {
        manga
            .relationships
            .iter()
            .find(|&rel| rel.type_ == RelationshipType::Author)
    });

    for author in authors {
        if let Some(RelatedAttributes::Author(author_attributes)) = &author.attributes {
            println!("{} - {}", author.id, author_attributes.name);
        }
    }
    Ok(())
}
```

## Downloading chapter pages

[Back to top][readme-section-toc]

Reference: <https://api.mangadex.org/docs/reading-chapter/>

## Using the old way

```rust
// Imports used for downloading the pages to a file.
// They are not used because we're just printing the raw bytes.
// use std::fs::File;
// use std::io::Write;

use uuid::Uuid;

use mangadex_api::v5::MangaDexClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    // Yeah, i'm a [`100 girlfriend`](https://mangadex.org/title/efb4278c-a761-406b-9d69-19603c5e4c8b/the-100-girlfriends-who-really-really-really-really-really-love-you) simp and what! >:)
    let chapter_id = Uuid::parse_str("f2a09509-3c09-4371-a810-ecb99242bd90")?;

    let at_home = client
        .at_home()
        .server()
        .id(chapter_id)
        .get()
        .send()
        .await?;

    let http_client = reqwest::Client::new();

    // Original quality. Use `.data.attributes.data_saver` for smaller, compressed images.
    let page_filenames = &at_home.chapter.data;
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

    Ok(())
}

```

### Using the `utils` feature

#### Via `(filename, Result<bytes>)` vector based

Not recommended if you want to handle each response error

```rust
use anyhow::Result;
use mangadex_api::{utils::download::chapter::DownloadMode, MangaDexClient};
/// used for file exporting
use std::{
    fs::{create_dir_all, File},
    io::Write,
};

/// It's from this manga called [`The Grim Reaper Falls In Love With A Human`](https://mangadex.org/title/be2efc56-1669-4e42-9f27-3bd232bca8ea/the-grim-reaper-falls-in-love-with-a-human)
///
/// [Chapter 1 English](https://mangadex.org/chapter/2b4e39a5-fba0-4055-a176-8b7e19faacdb) by [`Kredim`](https://mangadex.org/group/0b870e54-c75f-4d2e-8068-c40f939135fd/kredim)
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = "your-output-dir";
    let client = MangaDexClient::default();
    let chapter_id = uuid::Uuid::parse_str("32b229f6-e9bf-41a0-9694-63c11191704c")?;
    let chapter_files = client
        // We use the download builder
        .download()
        // Chapter id (accept uuid::Uuid)
        .chapter(chapter_id)
        // You also use `DownloadMode::Normal` if you want some the original quality
        //
        // Default : Normal
        .mode(DownloadMode::DataSaver)
        // Enable the [`The MangaDex@Home report`](https://api.mangadex.org/docs/retrieving-chapter/#the-mangadexhome-report-endpoint) if true
        //
        // Default : false
        .report(true)
        // Something that i don`t really know about
        //
        // More details at : https://api.mangadex.org/docs/retrieving-chapter/#basics
        .force_port_443(false)
        .build()?
        .download_element_vec()
        .await?;
    create_dir_all(format!("{}{}", output_dir, chapter_id))?;
    for (filename, bytes_) in chapter_files {
        if let Ok(bytes) = bytes_ {
            let mut file: File =
                File::create(format!("{}{}/{}", output_dir, chapter_id, filename))?;
            file.write_all(&bytes)?;
        } else if let Err(e) = bytes_ {
            eprintln!("{}", e);
        }
    }
    Ok(())
}
```

#### Via `tokio-stream`

With [`tokio-stream`](https://docs.rs/tokio-stream/), you can handle each response result

##### Without checker

```rust
use anyhow::Result;
use mangadex_api::{utils::download::chapter::DownloadMode, MangaDexClient};
use std::{
    fs::{create_dir_all, File},
    io::Write,
};
use tokio::pin;
use tokio_stream::StreamExt;

/// It's from this manga called [`Keiken Zumi na Kimi to, Keiken Zero na Ore ga, Otsukiai Suru Hanashi`](https://mangadex.org/title/1c8f0358-d663-4d60-8590-b5e82890a1e3/keiken-zumi-na-kimi-to-keiken-zero-na-ore-ga-otsukiai-suru-hanashi)
///
/// [Chapter 13 English](https://mangadex.org/chapter/250f091f-4166-4831-9f45-89ff54bf433b) by [`Galaxy Degen Scans`](https://mangadex.org/group/ab24085f-b16c-4029-8c05-38fe16592a85/galaxy-degen-scans)
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = "./test-outputs";
    let client = MangaDexClient::default();
    let chapter_id = uuid::Uuid::parse_str("250f091f-4166-4831-9f45-89ff54bf433b")?;
    create_dir_all(format!("{}/{}", output_dir, chapter_id))?;
    let download = client
        // We use the download builder
        .download()
        // Chapter id (accept uuid::Uuid)
        .chapter(chapter_id)
        // You also use `DownloadMode::Normal` if you want some the original quality
        //
        // Default : Normal
        .mode(DownloadMode::DataSaver)
        // Enable the [`The MangaDex@Home report`](https://api.mangadex.org/docs/04-chapter/retrieving-chapter/) if true
        //
        // Default : false
        .report(true)
        // Something that i don`t really know about
        //
        // More details at : https://api.mangadex.org/docs/04-chapter/retrieving-chapter/
        .force_port_443(false)
        .build()?;
    let chapter_files = download.download_stream().await?;
    // `pin!` Required for iteration
    pin!(chapter_files);
    while let Some((data, index, total)) = chapter_files.next().await {
        let (filename, bytes_) = data;
        // Prin the progression in the standart output
        println!("{index} / {total} : {filename} ");
        if let Ok(bytes) = bytes_ {
            let mut file: File =
                File::create(format!("{}/{}/{}", output_dir, chapter_id, filename))?;
            file.write_all(&bytes)?;
            println!("downloaded");
        } else if let Err(e) = bytes_ {
            eprintln!("{e}");
        }
    }
    Ok(())
}
```

##### with checker

The checker is a function called after the response fetching but before retreiving the byte content.
Example :

```rust
    /// Some code here
    let download = client
        .download()
        .chapter(chapter_id)
        .mode(DownloadMode::DataSaver)
        .report(true)
        .build()?;
    let chapter_files = download
        .download_stream_with_checker(move |filename, response| {
            /// if this function return `true`, the current response will be skipped
            true
        })
        .await?;
    /// Some code here too
```

Real example :

The checker will check return `true` if a file with the response content length has been created

```rust
use anyhow::Result;
use mangadex_api::{utils::download::chapter::DownloadMode, MangaDexClient};
use std::{
    fs::{create_dir_all, File},
    io::Write,
};
use tokio::pin;
use tokio_stream::StreamExt;

/// It's from this manga called [`Keiken Zumi na Kimi to, Keiken Zero na Ore ga, Otsukiai Suru Hanashi`](https://mangadex.org/title/1c8f0358-d663-4d60-8590-b5e82890a1e3/keiken-zumi-na-kimi-to-keiken-zero-na-ore-ga-otsukiai-suru-hanashi)
///
/// [Chapter 13 English](https://mangadex.org/chapter/250f091f-4166-4831-9f45-89ff54bf433b) by [`Galaxy Degen Scans`](https://mangadex.org/group/ab24085f-b16c-4029-8c05-38fe16592a85/galaxy-degen-scans)
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = "./test-outputs";
    let client = MangaDexClient::default();
    let chapter_id = uuid::Uuid::parse_str("250f091f-4166-4831-9f45-89ff54bf433b")?;
    create_dir_all(format!("{}/{}", output_dir, chapter_id))?;
    let download = client
        // We use the download builder
        .download()
        // Chapter id (accept uuid::Uuid)
        .chapter(chapter_id)
        // You also use `DownloadMode::Normal` if you want some the original quality
        //
        // Default : Normal
        .mode(DownloadMode::DataSaver)
        // Enable the [`The MangaDex@Home report`](https://api.mangadex.org/docs/04-chapter/retrieving-chapter/) if true
        //
        // Default : false
        .report(true)
        // Something that i don`t really know about
        //
        // More details at : https://api.mangadex.org/docs/04-chapter/retrieving-chapter/
        .force_port_443(false)
        .build()?;
    let chapter_files = download
        .download_stream_with_checker(move |filename, response| {
            let is_skip: bool = {
                // Get the response content length
                let content_length = match response.content_length() {
                    None => return false,
                    Some(d) => d,
                };
                // open the chapter image file
                File::open(format!(
                    "{}/{}/{}",
                    output_dir,
                    chapter_id,
                    filename.filename.clone()
                ))
                .map(|pre_file| {
                    pre_file
                        .metadata()
                        .map(|metadata| metadata.len() == content_length)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
            };
            is_skip
        })
        .await?;
    // `pin!` Required for iteration
    pin!(chapter_files);
    while let Some((data, index, total)) = chapter_files.next().await {
        let (filename, bytes_) = data;
        // Prin the progression in the standart output
        println!("{index} / {total} : {filename} ");
        if let Ok(bytes) = bytes_ {
            let mut file: File =
                File::create(format!("{}/{}/{}", output_dir, chapter_id, filename))?;
            file.write_all(&bytes)?;
            println!("downloaded");
        } else if let Err(e) = bytes_ {
            eprintln!("{e}");
        }
    }
    Ok(())
}
```

## Downloading a manga's main cover image

[Back to top][readme-section-toc]

### Use the legacy way

While this example could directly get the cover information by passing in the cover ID,
it is not often that one would have the ID off-hand, so the most common method would be from a
manga result.

If you want to get all of a manga's cover images, you will need to use the [cover list endpoint](https://api.mangadex.org/docs/swagger.html#/Cover/get-cover)
and use the `manga[]` query parameter.

```rust
// Imports used for downloading the cover to a file.
// They are not used because we're just printing the raw bytes.
// use std::fs::File;
// use std::io::Write;

use reqwest::Url;
use uuid::Uuid;

use mangadex_api::v5::MangaDexClient;
use mangadex_api::CDN_URL;
// use mangadex_api_types_rust::RelationshipType;
use mangadex_api_types::RelationshipType;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = MangaDexClient::default();

    let manga_id = Uuid::new_v4();
    let manga = client.manga().id(manga_id).get().send().await?;

    let cover_id = manga
        .data
        .relationships
        .iter()
        .find(|related| related.type_ == RelationshipType::CoverArt)
        .expect("no cover art found for manga")
        .id;
    let cover = client.cover().cover_id(cover_id).get().send().await?;

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
    Ok(())
}
```

### Using the `utils` feature (recommended)

#### via a cover id

```rust
    use anyhow::Result;
    use uuid::Uuid;
    use crate::MangaDexClient;
    use std::{io::Write, fs::File};

    /// Download the volume 2 cover of [Lycoris Recoil](https://mangadex.org/title/9c21fbcd-e22e-4e6d-8258-7d580df9fc45/lycoris-recoil)
    #[tokio::main]
    async fn main() -> Result<()>{
        let cover_id : Uuid = Uuid::parse_str("0bc12ff4-3cec-4244-8582-965b8be496ea")?;
        let client : MangaDexClient = MangaDexClient::default();
        let (filename, bytes) = client.download().cover().build()?.via_cover_id(cover_id).await?;
        let mut file = File::create(format!("{}/{}", "your-output-dir", filename))?;
        file.write_all(&bytes)?;
        Ok(())
    }
```

#### via a manga id

```rust
use anyhow::Result;
use mangadex_api::MangaDexClient;
use std::{fs::File, io::Write};
use uuid::Uuid;

/// Download the [Kimi tte Watashi no Koto Suki Nandesho?](https://mangadex.org/title/f75c2845-0241-4e69-87c7-b93575b532dd/kimi-tte-watashi-no-koto-suki-nandesho) cover
///
/// For test... of course :3
#[tokio::main]
async fn main() -> Result<()> {
    let output_dir = String::from("test-outputs");
    let manga_id: Uuid = Uuid::parse_str("f75c2845-0241-4e69-87c7-b93575b532dd")?;
    let client: MangaDexClient = MangaDexClient::default();
    let (filename, bytes) = client
        .download()
        .cover()
        // you can use
        //
        // ```rust
        // .quality(CoverQuality::Size512)
        // ``` for 512
        // or
        // ```rust
        // .quality(CoverQuality::Size256)
        // ``` for 256
        .build()?
        .via_manga_id(manga_id)
        .await?;
    let bytes = bytes?;
    let mut file = File::create(format!("{}/{}", output_dir, filename))?;
    file.write_all(&bytes)?;
    println!("donwloaded :3");
    Ok(())
}
```

## Authentification (via the `oauth` feature)

Before ~~paste copying~~ *uhm*,... touching the example code below, I recommend that you read the [Mangadex Authentification Section](https://api.mangadex.org/docs/02-authentication/)

First, register a personal client at [Mangadex Profile Settings][mangadex-settings], and wait until it's approved by staff. It can take 2 or 3 three days for now so just wait :>

After a long time, you can now `login` via the `oauth` feature.

### Login

```rust
use mangadex_api::MangaDexClient;
use mangadex_api_schema::v5::oauth::ClientInfo;
use mangadex_api_types::{Password, Username};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = MangaDexClient::default();
    client
        .set_client_info(&ClientInfo {
            client_id: String::from("<SET YOUR CLIENT ID HERE>"),
            client_secret: String::from("<SET YOUR CLIENT INFO HERE>"),
        })
        .await?;
    let response = client
        .oauth()
        .login()
        .username(Username::parse("<YOUR USERNAME HERE>")?)
        .password(Password::parse("<YOUR PASSWORD HERE>")?)
        .send()
        .await?;
    /*
       println!("Access Token: {}", response.access_token);
    */
    println!("Expires in {} minutes", response.expires_in / 60);
    Ok(())
}
```

### Resfresh your token

You just call `mangadex_api::MangaDexClient::oauth().refresh()`

```rust
    ...
    client
        .oauth()
        .refresh()
        .send()
        .await?;
    ...
```

Example:

```rust
use mangadex_api::MangaDexClient;
// use mangadex_api_schema_rust::v5::oauth::ClientInfo;
// use mangadex_api_types_rust::{Password, Username};
use mangadex_api_schema::v5::oauth::ClientInfo;
use mangadex_api_types::{Password, Username};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = MangaDexClient::default();

    // Register your client info
    client
        .set_client_info(&ClientInfo {
            client_id: String::from("<SET YOUR CLIENT ID HERE>"),
            client_secret: String::from("<SET YOUR CLIENT INFO HERE>"),
        })
        .await?;

    // Login to your account
    let response = client
        .oauth()
        .login()
        .username(Username::parse("<YOUR USERNAME HERE>")?)
        .password(Password::parse("<YOUR PASSWORD HERE>")?)
        .send()
        .await?;
    /*
       println!("Access Token: {}", response.access_token);
    */
    println!("Expires in {} minutes", response.expires_in / 60);
    // Wait until the token expires

    sleep(Duration::from_secs(<u64 as TryFrom<usize>>::try_from(
        response.expires_in,
    )?))
    .await;

    // Refresh the session token
    let response = client.oauth().refresh().send().await?;
    /*
       println!("Access Token: {}", response.access_token);
    */
    println!("Expires in {} minutes", response.expires_in / 60);
    Ok(())
}
```

## Changelog

[Back to top][readme-section-toc]

The changelog can be found [here][changelog].

Changes are added manually to keep the changelog human-readable with summaries of the changes from each version.

## License

[Back to top][readme-section-toc]

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE][license-apache] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT][license-mit] or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

[Back to top][readme-section-toc]

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Contributing

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
[library-schema-module]: https://crates.io/crates/mangadex-api-schema-rust
[license-apache]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/LICENSE-APACHE
[license-mit]: https://gitlab.com/gondolyr/mangadex-api/-/blob/main/LICENSE-MIT

[readme-section-toc]: #table-of-contents
[personal-client]: https://api.mangadex.org/docs/02-authentication/personal-clients/
[mangadex-settings]: https://mangadex.org/settings
