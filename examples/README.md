# Table of Contents

## Fetch Manga chapter pages

[Featch manga chapter pages](#fetch-chapter-pages)

## Fetch Manga Covers With a Manga ID

[Fetch manga covers](#fetch-manga-covers)

## Latest Manga Update

[Get the latest manga updates](#latest-updates)

## Log into MangaDex

[Log in](#log-in)

## Map Legacy IDs

[Map legacy IDs to get the new UUIDs](#map-legacy-ids)

## v5 Demo

This example features many of the fetch endpoints and returns the response body.

* [Search a list of manga](#search-manga)
* [View a Single Manga](#view-a-single-manga)
* [Get Recent Chapters for a Manga](#get-recent-chapters-for-a-manga)
* [Get a random Manga](#get-a-random-manga)
* [Get a MD@Home node URL](#get-a-mdhome-node-url)
* [Search authors](#search-author)
* [Search a list of chapters](#search-chapters)
* [View a Single Chapter](#view-a-single-chapter)
* [Search a list of covers](#search-covers)
* [View a Single Cover](#view-a-single-cover)
* [Get Manga volumes and chapters](#get-manga-volumes-and-chapters)
* [Search a list of scanlation groups](#search-scanlation-groups)
* [View a Single Scanlation Group](#view-a-single-scanlation-group)

# Fetch Chapter Pages

[Back to top](#table-of-contents)

This example will fetch chapter pages given a chapter ID.

## Usage

```
cargo run --example download_chapter -- [--data-saver] [-o output] <chapterid>
```

### Options

- `--data-saver`
    Fetch compressed images, which have smaller filesizes. If this is not used, the original uploaded files will be fetched.

- `-o` | `--download`
    Specify the output local directory to save the pages to. If this option is not used, a debug output will be presented instead.

## Examples

Fetch the given chapter's pages and output the responses for each page.

```
cargo run --example download_chapter -- c84f0bdd-0936-4fc3-8a7d-9b24303df33e
```

Fetch and download the given chapter's pages to the current directory.

```
cargo run --example download_chapter -- --download ./ c84f0bdd-0936-4fc3-8a7d-9b24303df33e
```

# Fetch Manga Covers

[Back to top](#table-of-contents)

This example will fetch manga cover art data.

A `MANGA` is a UUID.

## Usage

```
manga_covers [OPTION] [MANGA...]
```

### Options

-h, --help     Output a usage message and exit.
-o, --download Specify the directory to save the images to.

## Examples

This example will get the cover art data for the official test manga.

```
manga_covers f9c33607-9180-4ba6-b85c-e4b5faee7192
```

This will download the manga covers to the local filesystem at the specified directory.

```
manga_covers --download ./ f9c33607-9180-4ba6-b85c-e4b5faee7192
```

---

# Latest Updates

[Back to top](#table-of-contents)

This example will fetch manga with recently published chapters for the specified languages.
By default, this will only fetch English-translated manga.

`latest_updates` returns manga with chapter translations matching each ISO 639-1 2-letter `LANGUAGE`.
If no `LANGUAGE` is provided, it defaults to "en", English.

## Usage

```
latest_updates [OPTION] [LANGUAGE...]
```

## Options

```
-h, --help  Output a usage message and exit.
-p, --page  Specify the page of results. Default is 1.
-l, --limit Specify the maximum number of results to return. Default is 10.
```

## Examples

This example will return up to 20 manga with newly published English and Japanese chapters.
```
latest_updates --limit 20 en ja
```

---

# Log In

This will log in with the credentials provided upon compilation.

You must edit the [`login.rs`](login.rs) file with your credentials in the login builder pattern to use this.

## Usage

```
login
```

---

# Map Legacy IDs

This example will get mappings of legacy IDs to the new UUIDs.

# Usage

```
map_legacy_ids [OPTION] [ID...]
```

## Options

-h, --help  Output a usage message and exit.
-t, --type  Specify the type of IDs that should be mapped.
            Available options are:
                - chapter
                - group
                - manga
                - tag

# Examples

This example will return up the new UUIDs for the legacy manga IDs 18803 and 1001.

```
map_legacy_ids -t manga 18803 1001
```

---

# Search Manga

[Back to top](#table-of-contents)

Search for manga with matching titles.

## Usage

```
cargo run --example v5_demo -- --manga-search "<title>"
```

Providing an empty string for the value will return the most recently updated manga.

# View a Single Manga

[Back to top](#table-of-contents)

View details for a single manga.

## Usage

```
cargo run --example v5_demo -- --manga-view "<manga_id>"
```

The `manga_id` must be a UUID v4.

# Get Recent Chapters for a Manga

[Back to top](#table-of-contents)

Get the latest chapters for a manga.

## Usage

```
cargo run --example v5_demo -- --manga-feed "<manga_id>"
```

The `manga_id` must be a UUID v4.

# Get a random Manga

[Back to top](#table-of-contents)

Get a random manga, chosen by MangaDex.

## Usage

```
cargo run --example v5_demo -- --manga-random
```

# Get a MD@Home node URL

[Back to top](#table-of-contents)

Get a MangaDex@Home node URL.

This can be used to [fetch chapter pages](https://api.mangadex.org/docs/reading-chapter/#retrieving-pages-from-the-mangadexhome-network).

## Usage

```
cargo run --example v5_demo -- --node "<chapter_id>"
```

The `chapter_id` must be a UUID v4.

# Search Author

[Back to top](#table-of-contents)

Search for authors matching the provided name.

## Usage

```
cargo run --example v5_demo -- --author-search "<name>"
```

Providing an empty string for the value will return the most recently updated authors.

# Search Chapters

[Back to top](#table-of-contents)

Search for chapters with the corresponding manga ID (UUID).

## Usage

```
cargo run --example v5_demo -- --chapter-search "<manga_id>"
```

The `manga_id` must be a UUID v4.

# View a Single Chapter

[Back to top](#table-of-contents)

View details for a single chapter.

## Usage

```
cargo run --example v5_demo -- --chapter-view "<chapter_id>"
```

The `chapter_id` must be a UUID v4.

# Get a manga UUID from a legacy numerical ID

[Back to top](#table-of-contents)

Get a manga UUID from a legacy numerical ID.

The mapping endpoint can also map group, chapter, and tag IDs,
however, for this example, only one is supported.

## Usage

```
cargo run --example v5_demo -- --legacy-id-mapping "<manga_id>"
```

# Search Covers

[Back to top](#table-of-contents)

Search for covers with the corresponding manga ID (UUID).

## Usage

```
cargo run --example v5_demo -- --cover-search "<manga_id>"
```

The `manga_id` must be a UUID v4.

# View a Single Cover

[Back to top](#table-of-contents)

View details for a single cover.

## Usage

```
cargo run --example v5_demo -- --cover-view "<cover_id>"
```

The `cover_id` must be a UUID v4.

# Get Manga volumes and chapters

[Back to top](#table-of-contents)

Get the volume numbers and associated chapter numbers for a given manga.

## Usage

```
cargo run --example v5_demo -- --manga-aggregate "<manga_id>"
```

The `manga_id` must be a UUID v4.

# Search Scanlation Groups

[Back to top](#table-of-contents)

Search for scanlation groups matching the provided name.

## Usage

```
cargo run --example v5_demo -- --group-search "<name>"
```

Providing an empty string for the value will return 10 groups.

# View a Single Scanlation Group

[Back to top](#table-of-contents)

View details for a single scanlation group.

## Usage

```
cargo run --example v5_demo -- --group-view "<group_id>"
```

The `group_id` must be a UUID v4.
