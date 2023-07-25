# 2.0.0-rc.1

## Breaking

- ([308792d]) Changed the [manga aggregate endpoint][docs-manga-aggregate-endpoint] response struct ([mangadex-api struct](https://gitlab.com/gondolyr/mangadex-api/-/blob/c50f5361/src/v5/schema/manga_aggregate.rs)) to use [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) instead of [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) to provide an ordered collection of volumes and chapters. This allows users to iterate through the map and get them in order such as "1, 2, 3..." without requiring them to perform the sorting manually.
- ([53a7d7b]) Remove the [`chrono`][crates-chrono] crate as a feature. The [`time`][crates-time] crate is now used by default. This is still being experimented with and [`chrono`][crates-chrono] may return if there's demand. The primary reasons for this removal are because of the extra maintenance it takes to support [`chrono`][crates-chrono] and [`time`][crates-time] together and because of the security advisory [`chrono`][crates-chrono] has. [`chrono`][crates-chrono] has addressed [RUSTSEC-2020-0071] and [RUSTSEC-2020-0159] and is not actually calling `localtime_r` in [`time`][crates-time] ([source](https://github.com/chronotope/chrono/issues/602#issuecomment-940445390)). [`chrono`][crates-chrono] plans on removing its dependency on [`time`][crates-time] in their next semver-compatible version ([source](https://github.com/chronotope/chrono/issues/602#issuecomment-1075915577)).
- Changed the response struct for the is user following an object endpoints listed below ([Struct Code](https://gitlab.com/gondolyr/mangadex-api/-/blob/ae8c150f/mangadex-api-schema/src/v5/is_following_response.rs)):
    - ([d07082f]) Change the response struct for the [is user following a scanlation group endpoint][docs-follows-user-following-scanlation-group-endpoint]. MangaDex uses 404 with `{"result": "ok"}` in the response body to indicate that the request was successful but the user is not following the object.
    - ([b2f1daa]) Change the response struct for the [is user following a manga endpoint][docs-follows-user-following-manga-endpoint]. MangaDex uses 404 with `{"result": "ok"}` in the response body to indicate that the request was successful but the user is not following the object.
    - ([8748fb0]) Change the response struct for the [is user following a user endpoint][docs-follows-user-following-user-endpoint]. MangaDex uses 404 with `{"result": "ok"}` in the response body to indicate that the request was successful but the user is not following the object.

## Endpoints

### Account

- (5.5.7, [447a729]) Add the [check if username is available endpoint][docs-account-username-available-endpoint].

### CustomList

- (5.5.8, [4040c9e]) Add the [follow a custom list endpoint][docs-customlist-follow-custom-list-endpoint].
- (5.5.8, [5ffca10]) Add the [unfollow a custom list endpoint][docs-customlist-unfollow-custom-list-endpoint].

### Follows

- (5.5.8 [2de6365]) Add the [get user followed custom lists endpoint][docs-follows-user-following-custom-lists-endpoint].
- (5.5.8, [e5ef407]) Add the [is user following a custom list endpoint][docs-follows-is-following-custom-list-endpoint].

### Manga

- (5.5.3, [bc11fe0]) Add `availableTranslatedLanguages` field to the [manga endpoints][docs-manga-endpoint-section].
    - [Manga list (response)][docs-manga-list-endpoint]
    - [Create Manga (request and response)][docs-manga-create-endpoint]
    - [View Manga (response)][docs-manga-view-endpoint]
    - [Update Manga (request and response)][docs-manga-update-endpoint]
    - [Manga feed (request)][docs-manga-feed-endpoint]
    - [Get a random Manga (response)][docs-manga-random-endpoint]
    - [Get a specific Manga draft (response)][docs-manga-get-draft-endpoint]
    - [Submit a Manga draft (response)][docs-manga-submit-draft-endpoint]
    - [Get a list of Manga drafts (response)][docs-manga-list-drafts-endpoint]
- (5.5.6, [9798880]) Add the `contentRating` query parameter to the [random Manga endpoint][docs-manga-random-endpoint].

### Settings

- (5.5.7, [c4b803f]) Add the `updatedAt` query parameter to the [get user settings endpoint][docs-settings-get-user-endpoint].
- (5.5.7, [babf6e5]) Add the `settings` and `updatedAt` request body fields to the [create user settings endpoint][docs-settings-create-update-user-endpoint].

## Refactor

- ([07048a8]) Move the `types` module into its own workspace. This should hopefully improve compile times just a bit but most of all, improve the project file structure.
- ([6de3bc5]) Move the `schema` module into its own workspace. This should hopefully improve compile times just a bit but most of all, improve the project file structure.

## Fixes

- ([6e53ac9]) Fix the 2-letter language code for Romanian.
- ([b0fd3b3]) Change the start upload chapter session HTTP test to use `assert!` instead of `assert_eq!` for one of the tests.

## Documentation

- (5.5.7, [1253589]) Update the MangaUpdates URL regex for scanlation groups.
- ([2beeae0]) Change the MangaDex API docs URL to link to the Swagger page. MangaDex had changed what is displayed on their main API landing page at [https://api.mangadex.org][docs-api-url].
- ([571178a]) Remove extra semicolon in endpoint doctests.
- ([5af916f]) Add "Dependency Justification" section to the [README][local-readme]. This section is taking the advice from [Reddit to add this section to projects](https://teddit.net/r/rust/comments/u5cjmu/dependency_justification_section_may_need_more/). [Crossterm has this in their README](https://github.com/crossterm-rs/crossterm#dependency-justification) and the format is taken from it.

## Other

- ([eb27b5e], [60a544f]) Update `anyhow`, `clap`, `derive_builder`, `futures`, `serde`, `serde_json`, `serde_qs`, `time`, `tokio`, `uuid`, `wiremock` dependencies.
- ([7f01935]) Remove doctests from the GitLab CI/CD pipeline to prevent sending requests to MangaDex's servers.

[07048a8]: https://gitlab.com/gondolyr/mangadex-api/-/commit/07048a8
[1253589]: https://gitlab.com/gondolyr/mangadex-api/-/commit/1253589
[2beeae0]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2beeae0
[2de6365]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2de6365
[308792d]: https://gitlab.com/gondolyr/mangadex-api/-/commit/308792d
[4040c9e]: https://gitlab.com/gondolyr/mangadex-api/-/commit/4040c9e
[447a729]: https://gitlab.com/gondolyr/mangadex-api/-/commit/447a729
[53a7d7b]: https://gitlab.com/gondolyr/mangadex-api/-/commit/53a7d7b
[571178a]: https://gitlab.com/gondolyr/mangadex-api/-/commit/571178a
[5af916f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/5af916f
[5ffca10]: https://gitlab.com/gondolyr/mangadex-api/-/commit/5ffca10
[60a544f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/60a544f
[6de3bc5]: https://gitlab.com/gondolyr/mangadex-api/-/commit/6de3bc5
[6e53ac9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/6e53ac9
[7f01935]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7f01935
[8748fb0]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8748fb0
[9798880]: https://gitlab.com/gondolyr/mangadex-api/-/commit/9798880
[b0fd3b3]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b0fd3b3
[b2f1daa]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b2f1daa
[babf6e5]: https://gitlab.com/gondolyr/mangadex-api/-/commit/babf6e5
[bc11fe0]: https://gitlab.com/gondolyr/mangadex-api/-/commit/bc11fe0
[c4b803f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c4b803f
[d07082f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/d07082f
[e5ef407]: https://gitlab.com/gondolyr/mangadex-api/-/commit/e5ef407
[eb27b5e]: https://gitlab.com/gondolyr/mangadex-api/-/commit/eb27b5e

# Version 1.3.0 (2022-02-04)

# Documentation

- (5.4.12, [28dd4fc]) Add descriptions to the [`MangaRelation` enum][docs-main-staticdata-manga-related-enum]. These descriptions are [provided by MangaDex][docs-main-staticdata-manga-related-enum].

# Endpoints

## Chapter

- (5.4.13, [3204250]) Add `readableAt` sort order variant to the [chapter list endpoint][docs-chapter-list-endpoint].
- (5.4.13, [0b0936b]) Add `readableAt` response field to the following endpoints that return chapter data.
    - [Chapter list][docs-chapter-list-endpoint]
    - [Get Chapter][docs-chapter-view-endpoint]
    - [Update Chapter][docs-chapter-update-endpoint]
    - [Get logged User followed Manga feed (Chapter list)][docs-feed-user-followed-manga-endpoint]
    - [Manga feed][docs-manga-feed-endpoint]

## Cover

- (5.4.12, [5d51c26], [e550175]) Add `locale` field to the [cover endpoints][docs-cover-section].
    - [CoverArt list (request and response)][docs-cover-list-endpoint]
    - [Upload cover (request and response)][docs-cover-upload-endpoint]
    - [Get cover (response)][docs-cover-view-endpoint]
    - [Edit cover (request and response)][docs-cover-edit-endpoint]

## Manga

- (5.4.12, [b09763f]) Add `chapterNumbersResetOnNewVolume` field to the [manga endpoints][docs-manga-section].
    - [Manga list (response)][docs-manga-list-endpoint]
    - [Create Manga (request and response)][docs-manga-create-endpoint]
    - [View Manga (response)][docs-manga-view-endpoint]
    - [Update Manga (request and response)][docs-manga-update-endpoint]
    - [Manga feed (request)][docs-manga-feed-endpoint]
    - [Get a random Manga (response)][docs-manga-random-endpoint]
    - [Get a specific Manga draft (response)][docs-manga-get-draft-endpoint]
    - [Submit a Manga draft (response)][docs-manga-submit-draft-endpoint]
    - [Get a list of Manga drafts (response)][docs-manga-list-drafts-endpoint]
- (5.4.13, [3204250]) Add `readableAt` sort order variant to the endpoints that return a manga feed.
    - [Get logged User followed Manga feed (Chapter list)][docs-feed-user-followed-manga-endpoint]
    - [CustomList Manga feed][docs-customlist-manga-feed-endpoint]
    - [Manga feed][docs-manga-feed-endpoint]

## Report

- (5.4.12 / 5.4.13, [108e44f], [0ea0c13]) Add [endpoint to get a list of user submitted reports][docs-report-list-endpoint].

## Scanlation Group

- (5.4.11, [8b55f9c]) Add `mangaUpdates` field to the [Scanlation Group endpoints][docs-scanlationgroup-section].
    - [List groups (response)][docs-scanlationgroup-list-endpoint]
    - [Create group (request and response)][docs-scanlationgroup-create-endpoint]
    - [View group (response)][docs-scanlationgroup-view-endpoint]
    - [Update group (request and response)][docs-scanlationgroup-update-endpoint]

[0b0936b]: https://gitlab.com/gondolyr/mangadex-api/-/commit/0b0936b
[0ea0c13]: https://gitlab.com/gondolyr/mangadex-api/-/commit/0ea0c13
[108e44f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/108e44f
[28dd4fc]: https://gitlab.com/gondolyr/mangadex-api/-/commit/28dd4fc
[3204250]: https://gitlab.com/gondolyr/mangadex-api/-/commit/3204250
[5d51c26]: https://gitlab.com/gondolyr/mangadex-api/-/commit/5d51c26
[8b55f9c]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8b55f9c
[b09763f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b09763f
[e550175]: https://gitlab.com/gondolyr/mangadex-api/-/commit/e550175

# Version 1.2.1 (2022-01-14)

## Deprecate

- ([8a19e8e]) Deprecate the `user` query parameter for the [list manga drafts endpoint][docs-manga-list-drafts-endpoint]. This field removed in MangaDex 5.4.9.

## Documentation

- ([c666443]) Add note about `follows` field not matching the API documentation compared with the actual response.

## Endpoints

### Settings

The [Settings API documentation][docs-settings-section] lacks a lot of details about how to make requests to the endpoints so only the structure has been implemented.
The interface to it is private until more details are given.

- ([5bcdb6c]) Add boilerplate code for the [Settings endpoints][docs-settings-section].
- ([a51b19a]) Add boilerplate code for the [create Settings template endpoint][docs-settings-create-template-endpoint].
- ([f8812c7]) Add boilerplate code for the [get Settings template by version ID endpoint][docs-settings-get-template-endpoint].
- ([221b081]) Add boilerplate code for the [create or update a user's Settings endpoint][docs-settings-create-update-user-endpoint].
- ([8047356]) Add boilerplate code for the [get latest Settings template endpoint][docs-settings-get-latest-template-endpoint].
- ([df0c835]) Add boilerplate code for the [get a user's Settings endpoint][docs-settings-get-user-endpoint].

## Internal

- ([b2eb88d]) Pin all dependencies. After seeing the damage the NPM libraries `colors` and `faker` had done due to malicious intent, for the safety of this library's users, all dependencies will now specify an exact version to use.

[221b081]: https://gitlab.com/gondolyr/mangadex-api/-/commit/221b081
[5bcdb6c]: https://gitlab.com/gondolyr/mangadex-api/-/commit/5bcdb6c
[8047356]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8047356
[8a19e8e]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8a19e8e
[a51b19a]: https://gitlab.com/gondolyr/mangadex-api/-/commit/a51b19a
[b2eb88d]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b2eb88d
[c666443]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c666443
[df0c835]: https://gitlab.com/gondolyr/mangadex-api/-/commit/df0c835
[f8812c7]: https://gitlab.com/gondolyr/mangadex-api/-/commit/f8812c7

# Version 1.2.0 (2022-01-09)

## Documentation

- ([dd3403a]) Add description to the cover endpoint that IDs ([Upload][docs-cover-upload-endpoint], [Get][docs-cover-get-endpoint], [Edit][docs-cover-edit-endpoint], [Delete][docs-cover-delete-endpoint]) may be Manga for cover IDs.
- ([87500f7]) Remove reference to deprecated chapter response fields.

## Examples

- ([99e62b1]) Change `download_chapter` output to match Tachiyomi's folder and filename format.

## Features

- ([1d799aa]) Add Weibo and Naver fields to the [author response body][docs-author-view-endpoint].
- ([15aa4c2]) Add Weibo and Naver fields to the [update author request body][docs-author-update-endpoint].
- ([1e6ceb9]) Add Weibo and Naver fields to the [create author request body][docs-author-create-endpoint].
- ([54b9534]) Add [create or update Manga rating endpoint][docs-rating-create-update-manga-endpoint].
- ([7ed0c82]) Add [delete Manga rating endpoint][docs-rating-delete-manga-endpoint].
- ([4dfc671]) Add [find Manga statistics endpoint][docs-statistics-get-manga-endpoint].
- ([7cf7fc1]) Add [get your Manga ratings endpoint][docs-rating-get-manga-endpoint].
- ([93fe174]) Add `includes` query parameter to the [Manga relation list endpoint][docs-manga-list-relation-endpoint].
- ([f1d3145]) Add `excludedGroups` query parameter to the [list chapters endpoint][docs-chapter-list-endpoint].
- ([cc30519]) Add `excludedUploaders` query parameter to the [list chapters endpoint][docs-chapter-list-endpoint].
- ([d557e77]) Add `excludedGroups` query parameter to the [user followed Manga feed endpoint][docs-feed-user-followed-manga-endpoint].
- ([53a140d]) Add `excludedUploaders` query parameter to the [user followed Manga feed endpoint][docs-feed-user-followed-manga-endpoint].
- ([1b038cf]) Add `excludedGroups` query parameter to the [custom list Manga feed endpoint][docs-customlist-manga-feed-endpoint].
- ([41c8565]) Add `excludedUploaders` query parameter to the [custom list Manga Feed endpoint][docs-customlist-manga-feed-endpoint].
- ([c3ec172]) Add `excludedGroups` query parameter to the [Manga feed endpoint][docs-manga-feed-endpoint].
- ([2e0e8c2]) Add `excludedUploaders` query parameter to the [Manga feed endpoint][docs-manga-feed-endpoint].
- ([22c6c13]) Add `pages` field to the chapter response body ([List][docs-chapter-list-endpoint], [Get][docs-chapter-view-endpoint]). This provides information about how many pages the chapter has.
- ([99c0dd8]) Add the [get Manga statistics endpoint][docs-statistics-get-manga-endpoint].
- ([af2f4ac]) Add `AlternateVersion` variant to the `MangaRelation` enum.

## Internal

- ([6540689]) Sort endpoint functions alphabetically for `MangaDexClient`. This makes it easier to scroll to find functions without searching.

## Refactor

- ([55680ec]) Allow multiple uploaders to be set for the [list chapter endpoint][docs-chapter-list-endpoint].
- ([bf43974]) Replace `StructOpt` with `clap` dependency. [`clap` 3.0](https://epage.github.io/blog/2021/12/clap3/) integrates `StructOpt` functionality into the library and using `clap` directly will ensure that the latest version of `clap` and its dependencies will be used, improving stability and security.

[15aa4c2]: https://gitlab.com/gondolyr/mangadex-api/-/commit/15aa4c2
[1b038cf]: https://gitlab.com/gondolyr/mangadex-api/-/commit/1b038cf
[1d799aa]: https://gitlab.com/gondolyr/mangadex-api/-/commit/1d799aa
[1e6ceb9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/1e6ceb9
[22c6c13]: https://gitlab.com/gondolyr/mangadex-api/-/commit/22c6c13
[2e0e8c2]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2e0e8c2
[41c8565]: https://gitlab.com/gondolyr/mangadex-api/-/commit/41c8565
[4dfc671]: https://gitlab.com/gondolyr/mangadex-api/-/commit/4dfc671
[53a140d]: https://gitlab.com/gondolyr/mangadex-api/-/commit/53a140d
[54b9534]: https://gitlab.com/gondolyr/mangadex-api/-/commit/54b9534
[55680ec]: https://gitlab.com/gondolyr/mangadex-api/-/commit/55680ec
[6540689]: https://gitlab.com/gondolyr/mangadex-api/-/commit/6540689
[7cf7fc1]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7cf7fc1
[7ed0c82]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7ed0c82
[87500f7]: https://gitlab.com/gondolyr/mangadex-api/-/commit/87500f7
[93fe174]: https://gitlab.com/gondolyr/mangadex-api/-/commit/93fe174
[99c0dd8]: https://gitlab.com/gondolyr/mangadex-api/-/commit/99c0dd8
[99e62b1]: https://gitlab.com/gondolyr/mangadex-api/-/commit/99e62b1
[af2f4ac]: https://gitlab.com/gondolyr/mangadex-api/-/commit/af2f4ac
[bf43974]: https://gitlab.com/gondolyr/mangadex-api/-/commit/bf43974
[c3ec172]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c3ec172
[cc30519]: https://gitlab.com/gondolyr/mangadex-api/-/commit/cc30519
[d557e77]: https://gitlab.com/gondolyr/mangadex-api/-/commit/d557e77
[dd3403a]: https://gitlab.com/gondolyr/mangadex-api/-/commit/dd3403a
[f1d3145]: https://gitlab.com/gondolyr/mangadex-api/-/commit/f1d3145

# Version 1.1.0 (2021-12-24)

This version changes how chapter pages are fetched through the API. Instead of making a request to the chapter endpoint and the at-home server endpoint, only a request to the at-home server endpoint is required.

## Features

- ([7d3c50b0]) Add chapter page data to the [at-home server endpoint (`GET /at-home/server/{chapterId}`)][docs-athome-get-chapter-server-endpoint].

## Documentation

- ([3fc0cfd6]) Fix project name to make it consistent in casing in [CONTRIBUTING.md][local-contributing].
- ([fb26867a]) Add [conventional commits](https://conventionalcommits.org) style to [CONTRIBUTING.md][local-contributing].
- ([582bef8e]) Change duration description from PHP DateInterval to [ISO 8601 Duration](https://en.wikipedia.org/wiki/ISO_8601#Durations).

## Refactor

- ([67c82266]) Remove the chapter page data response fields, `hash`, `data`, and `dataSaver`, from all endpoints that return chapter data.
    - [Get chapter `GET /chapter/{id}`][docs-chapter-view-endpoint]
    - [Chapter list `GET /chapter`][docs-chapter-list-endpoint]
    - [Update chapter `PUT /chapter/{id}`][docs-chapter-update-endpoint]
    - [Commit the upload session `POST /upload/{uploadSessionId}/commit`][docs-upload-commit-endpoint]
    - [Get logged User followed Manga feed (Chapter list)][docs-feed-user-followed-manga-endpoint]

[3fc0cfd6]: https://gitlab.com/gondolyr/mangadex-api/-/commit/3fc0cfd6
[582bef8e]: https://gitlab.com/gondolyr/mangadex-api/-/commit/582bef8e
[67c82266]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7176aa18
[7d3c50b0]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7d3c50b0
[fb26867a]: https://gitlab.com/gondolyr/mangadex-api/-/commit/fb26867a

# Version 1.0.0 (2021-12-14)

This version stabilizes the `mangadex-api` library API. Edge-case responses are now handled gracefully so that the library does not panic. A number of modules have been moved around to (hopefully) be more logical and consistent.

I have no forseeable plans to introduce new breaking changes to the structure of the library; however, if the MangaDex API introduces them, it will be unavoidable in order to maintain compatibility with it.

## Added

- ([71921998]) Implemented the [custom list Manga feed][docs-customlist-manga-feed-endpoint] alias in the `v5::feed` module. This alias calls the `v5::custom_list::manga_feed()` function.
- ([3f32772c]) Added the `biography` field to the author [create][docs-author-create-endpoint] and [update][docs-author-update-endpoint] endpoints. This was added in MangaDex 5.3.13.
- ([26521d3f], [cf494478]) Added an example to download chapter pages. This should give users an idea how they can [read chapters through the API][docs-main-reading-a-chapter].
- ([5eb57760]) Added the `group` query parameter to the [Manga list endpoint][docs-manga-list-endpoint]. This was added in Mangadex 5.3.14.
- ([7e96fca2]) Added the `others` response field to the [Manga aggregate endpoint][docs-manga-aggregate-endpoint]. This was added in MangaDex 5.3.14.
- ([2cecb76f]) Added the `order` query parameter to sort the results for the [scanlation group list endpoint][docs-scanlationgroup-list-endpoint]. This was added in MangaDex 5.3.14.
- ([44915d6f]) Added `Author` variant to the `ReportCategory` enum. This was added in MangaDex 5.3.14.
- ([6b570a4b]) Added the `inactive` and `publishDelay` response fields to the scanlation group response bodies. This was added in MangaDex 5.3.14.
- ([df397bbd]) Added the `inactive` and `publishDelay` fields to the [create scanlation group endpoint][docs-scanlationgroup-create-endpoint]. This was added in MangaDex 5.3.14.
- ([1a155865]) Added the `inactive` and `publishDelay` fields to the [scanlation group update endpoint][docs-scanlationgroup-update-endpoint]. This was added in MangaDex 5.3.14.
- ([cf5dddc2]) Added the `context` field to the error response struct. This field is only present where Captcha is required such as when the client is rate-limited.
- ([e713508f]) Added the Nepali language.

## Fixed

- ([e8d8dfc5]) Fields that expect localized string objects (Map) can now deserialize array values and default to an empty `HashMap`.
- ([cfded268]) The `description` field for Manga responses can now handle when the API returns an empty array. For example, as of 2021-12-11, Manga ID [`fe38c4c2-8cea-4f91-9e80-b368172bdd5c`](https://api.mangadex.org/manga/fe38c4c2-8cea-4f91-9e80-b368172bdd5c) returns an empty array for the `description` field.
- ([c50f5361]) The library can now deserialize the `volumes` and `chapters` fields from the [manga aggregate endpoint][docs-manga-aggregate-endpoint] when an array is returned. For example, as of 2021-12-11, Manga ID [`d773c8be-8e82-4ff1-a4e9-46171395319b`](https://api.mangadex.org/manga/d773c8be-8e82-4ff1-a4e9-46171395319b/aggregate) returns an array for the `chapters` field and Manga ID [`14e157c3-7856-4f86-a2d2-8faa86660744`](https://api.mangadex.org/manga/14e157c3-7856-4f86-a2d2-8faa86660744/aggregate) returns an array for the `volumes` field.
- ([4e99bc52]) Changed the `focusedLanguages` response field name from `focused_language` to `focused_languages` for the scanlation group response. Because this is an optional field, the library would always return `None` even though the field was being returned. The API defines the field as `focusedLanguage` even though the API actually returns `focusedLanguages`.
- The following endpoints can now set nullable fields as `null` (`None`):
    - Author
        - ([7927ef48]) [Create author][docs-author-create-endpoint]
        - ([a9c1abfd]) [Update author][docs-author-update-endpoint]
    - Chapter
        - ([71d50445]) [Update chapter][docs-chapter-update-endpoint]
    - Cover
        - ([7b847776]) [Edit cover][docs-cover-edit-endpoint]
    - Manga
        - ([e0b67a64]) [Create Manga][docs-manga-create-endpoint]
        - ([752f89f1]) [Update Manga][docs-manga-update-endpoint]
    - Scanlation Group
        - ([405883a4]) [Create scanlation group][docs-scanlationgroup-create-endpoint]
        - ([8a8216f3]) [Update scanlation group][docs-scanlationgroup-update-endpoint]
- ([05fd0b1b]) Removed the unused `leader` and `members` scanlation group response fields. These fields were removed at some point and moved to the `relationships` field.
- ([e5fdb761]) The Manga response struct's `links` field can now handle instances when the API returns an empty array for the field. When an array is returned, the library will set the field to `None`.

## Changed

- ([4e5e01d9]) **BREAKING** Moved the `Username` and `Password` tuple structs to the `types` module. These structs acted as types instead of schema types (which are meant for response bodies). Use the `mangadex_api::mangadex_types::Username` and `mangadex_api::mangadex_types::Password` paths from now on.
- ([abeaca0b]) **BREAKING** Changed the `includes[]` single add method from `add_relationship()` to `include()` to make it consistent with the query parameter name. Because the API already returns all relationships in the results, having the function called `add_relationship()` was misleading, even if the relationships not specified were just the IDs.
- ([10a030ad]) **BREAKING** Moved the `mangadex_api::v5::error` module to `mangadex_api::error`. The `Error` and `Result` types are probably not specific to just the v5 API so for organization, it makes more sense for it to be placed at the crate level. While the `Result` type hasn't changed for users, the `Error` type has now been restricted to the `error` module so users will need to explicitly import it as `use mangadex_api::error::Error`. I don't feel that the `Error` type needs to be re-exported in the root level but should that change, it will be easy to add it back in as a non-breaking change.
- ([60d8e133]) **BREAKING** Separated the reference expansion enum to its own enum. The `RelationshipType` enum represents all response types that MangaDex could return while the new `ReferenceExpansionResource` is meant for adding types to the `includes[]` reference expansion query parameter for requests. Use `mangadex_api::mangadex_types::ReferenceExpansionResource` for the `includes` request field.
- ([b6f3ca1c]) **BREAKING** Use `url::Url` struct for `MangaLinks` fields with URLs. This should provide additional conveniences when handling URLs.
- ([f6fb8d19]) **BREAKING** String request fields will now require a string reference (`&str`) to use.
- ([0dadf437]) Added the `non_exhaustive` attribute to various types and structs. This should help maintain a certain level of backwards-compatibility when additional variants and fields are added.
    - Reference: <https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute>
- ([7f12a52a]) Changed how the library examples import the `MangaDexClient` to explicitly import it from the `v5` module as `use mangadex_api::v5::MangaDexClient` even though it still exists at the `mangadex_api::MangaDexClient` path. The reason for explicitly using the `v5` path is so that if a new API version were introduced from MangaDex and they maintain v5 and the new one, this would prevent this library from introducing a breaking change to users as long as they use the `v5` path. The root-level `MangaDexClient` will always use the latest API version.
- ([1d15e3eb]) Removed the authentication requirement for the refresh token endpoint. Users have an option of providing their own refresh token if they wish, which will override any stored tokens, otherwise, it will default to the refresh token stored in the `HttpClient`, if one exists. If neither are available, an error is returned.
- ([3ccb5d3c]) The library now handles HTTP 5xx errors separately. Previously, server errors would be lumped together with the `RequestError` and only provide a generic request failed error. Having a separate error will empower users to be able to handle errors that are not the fault of this library or their request appropriately in their applications.
- ([858a27e6]) Added additional context to the `Error` type so that when it is converted to a string, it has additional debug info.

## Internal

- ([f969abac]) Changed the `NoData` struct's docblock code to `text` so that it is not run when running the library's tests. The code is meant as reference for internal developers but not meant to be functional so having it execute would cause problems.
- ([537fe008]) Consolidated the `mangadex_api::v5::schema::common` module into `mangadex_api::v5::schema`. This removes the redundant structure where common definitions were defined in both the `mangadex_api::v5::schema` and `mangadex_api::v5::schema::common` modules. Having all of the common definitions in the parent module should reduce the ambiguity.
- ([04ef66ad]) Moved the `mangadex_api::common` module to `lib.rs`. This removes the redundant structure where common definitions were defined in both `mangadex_api::common` and `lib.rs`.
- ([60e071c7]) Added an example on how to use reference expansion in the README. The concept is a bit confusion from the API documenation and even moreso from this library. Having at least one example should reduce the vagueness of what it is and how to use it.
- ([07b2a036]) Updated [`tokio`][crates-tokio] to `1.14`.
- ([9f112f60]) Endpoint builders' `new()` constructor is now only visible the the library and `rustdoc` will no longer generate documentation for the method even with the `--document-private-items`. Individual endpoint `http_client` fields are only available to the library now and `rustdoc` will also not generate documentation for the field.
- ([cc6b123f]) Removed the public re-export of the `Result` type in the `v5` module. This wasn't being used and is already available from the crate root-level.

[04ef66ad]: https://gitlab.com/gondolyr/mangadex-api/-/commit/04ef66ad
[05fd0b1b]: https://gitlab.com/gondolyr/mangadex-api/-/commit/05fd0b1b
[07b2a036]: https://gitlab.com/gondolyr/mangadex-api/-/commit/07b2a036
[0dadf437]: https://gitlab.com/gondolyr/mangadex-api/-/commit/0dadf437
[10a030ad]: https://gitlab.com/gondolyr/mangadex-api/-/commit/10a030ad
[1d15e3eb]: https://gitlab.com/gondolyr/mangadex-api/-/commit/1d15e3eb
[26521d3f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/26521d3f
[2cecb76f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2cecb76f
[3ccb5d3c]: https://gitlab.com/gondolyr/mangadex-api/-/commit/3ccb5d3c
[3f32772c]: https://gitlab.com/gondolyr/mangadex-api/-/commit/3f32772c
[405883a4]: https://gitlab.com/gondolyr/mangadex-api/-/commit/405883a4
[44915d6f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/44915d6f
[4e5e01d9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/4e5e01d9
[4e99bc52]: https://gitlab.com/gondolyr/mangadex-api/-/commit/4e99bc52
[537fe008]: https://gitlab.com/gondolyr/mangadex-api/-/commit/537fe008
[5eb57760]: https://gitlab.com/gondolyr/mangadex-api/-/commit/5eb57760
[60d8e133]: https://gitlab.com/gondolyr/mangadex-api/-/commit/60d8e133
[60e071c7]: https://gitlab.com/gondolyr/mangadex-api/-/commit/60e071c7
[6b570a4b]: https://gitlab.com/gondolyr/mangadex-api/-/commit/6b570a4b
[71921998]: https://gitlab.com/gondolyr/mangadex-api/-/commit/71921998
[71d50445]: https://gitlab.com/gondolyr/mangadex-api/-/commit/71d50445
[7927ef48]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7927ef48
[7b847776]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7b847776
[7e96fca2]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7e96fca2
[7f12a52a]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7f12a52a
[752f89f1]: https://gitlab.com/gondolyr/mangadex-api/-/commit/752f89f1
[858a27e6]: https://gitlab.com/gondolyr/mangadex-api/-/commit/858a27e6
[8a8216f3]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8a8216f3
[9f112f60]: https://gitlab.com/gondolyr/mangadex-api/-/commit/9f112f60
[a9c1abfd]: https://gitlab.com/gondolyr/mangadex-api/-/commit/a9c1abfd
[abeaca0b]: https://gitlab.com/gondolyr/mangadex-api/-/commit/abeaca0b
[b6f3ca1c]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b6f3ca1c
[c50f5361]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c50f5361
[cc6b123f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/cc6b123f
[cf494478]: https://gitlab.com/gondolyr/mangadex-api/-/commit/cf494478
[cf5dddc2]: https://gitlab.com/gondolyr/mangadex-api/-/commit/cf5dddc2
[cfded268]: https://gitlab.com/gondolyr/mangadex-api/-/commit/cfded268
[df397bbd]: https://gitlab.com/gondolyr/mangadex-api/-/commit/df397bbd
[e0b67a64]: https://gitlab.com/gondolyr/mangadex-api/-/commit/e0b67a64
[e5fdb761]: https://gitlab.com/gondolyr/mangadex-api/-/commit/e5fdb761
[e713508f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/e713508f
[e8d8dfc5]: https://gitlab.com/gondolyr/mangadex-api/-/commit/e8d8dfc5
[f6fb8d19]: https://gitlab.com/gondolyr/mangadex-api/-/commit/f6fb8d19
[f969abac]: https://gitlab.com/gondolyr/mangadex-api/-/commit/f969abac

# Version 1.0.0-alpha.9 (2021-11-13)

This version addresses an issue searching for manga where the `hasAvailableChapters` query parameter was always being included in the request, defaulting to `false`.

## Fixed

- ([88f4667]) Fixed an issue searching for Manga where the `hasAvailableChapters` query parameter was always included in every request, whether or not it was specified. It would default to `false` and thus filter the results.

[88f4667]: https://gitlab.com/gondolyr/mangadex-api/-/commit/88f4667

# Version 1.0.0-alpha.8 (2021-11-12)

This version addresses [issue #10](https://gitlab.com/gondolyr/mangadex-api/-/issues/10) where users are unable to access [reference expanded][docs-main-reference-expansion] attributes.

## Changed

- ([a40c140]) **BREAKING** Renamed `mod_notes` field to `primary_cover` and adjusted its type from `String` to `Uuid`. This change was introduced in MangaDex 5.3.12.

## Fixed

- ([e0b5ec6]) Fixed reference expanded types from being inaccessible by making the enum `RelatedAttributes` public. This change is a quick fix to resolve [GitLab issue #10](https://gitlab.com/gondolyr/mangadex-api/-/issues/10) but the API may change to support a solution with less boilerplate code in the future.

## Internal

- ([b63135a]) Fixed an issue with how the documentation is generated on docs.rs where it would attempt to build the documentation with all features enabled. This is not possible due to `chrono` and `time` being incompatible with each other. For now, just the default list of features (currently none) will be built.

[a40c140]: https://gitlab.com/gondolyr/mangadex-api/-/commit/a40c140
[b63135a]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b63135a
[e0b5ec6]: https://gitlab.com/gondolyr/mangadex-api/-/commit/e0b5ec6

# Version 1.0.0-alpha.7 (2021-11-09)

Major changes include the transition to the Rust 2021 edition and a move away from `chrono` while also providing `time` support.
`chrono` is still part of this library, however, is no longer included by default and must be enabled as a feature.

## Added

- ([7ca3b80]) Added `twitter` field to scanlation group response attributes.
- ([12b983b]) Added `twitter` field to the create scanlation group endpoint.
- ([d0d42ed]) Added `twitter` field to the update scanlation group endpoint.
- ([4dc82a2]) Added `time` as an optional feature as an alternative to `chrono` in lieu of `chrono`'s disclosed security advisories.
- ([64da46a]) Added `source` field to the upload session file response attributes.
- ([a8c8a78]) Added the Start Edit Chapter Session endpoint. This allows users to edit existing chapters such as uploading updated page images.
- ([93e7558]) Added the `hasAvailableChapters` query parameter to the Manga list endpoint.
- ([92a4313]) Added new sort order fields for the manga feed endpoints. This change adds `createdAt`, `updatedAt`, and `publishAt` as available options.
- ([cead1f2]) Added `description` field to the upload cover endpoint.

## Changed

- ([228dec9]) **BREAKING** Made `chrono` an optional feature due to the recent security advisories that have been disclosed. `chrono` uses an outdated version of `time` (0.1 at the time of writing), [which has a security notice](https://rustsec.org/advisories/RUSTSEC-2020-0071). This information is largely taken from https://passcod.name/technical/no-time-for-chrono.html.

  See the following for more information:

  RustSec:
    - [RUSTSEC-2020-0071](https://rustsec.org/advisories/RUSTSEC-2020-0071) on Time 0.1 and 0.2.7–0.2.22
    - [RUSTSEC-2020-0159](https://rustsec.org/advisories/RUSTSEC-2020-0159) on Chrono

  Mitre:
    - [CVE-2020-26235](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-26235)

  GitHub:
    - [GHSA-wcg3-cvx6-7396](https://github.com/time-rs/time/security/advisories/GHSA-wcg3-cvx6-7396)

  Chrono:
    - https://github.com/chronotope/chrono/issues/499

- ([71e26b3]) **BREAKING** Changed the default and `chrono` features to use the new `MangaDexDateTime` newtype struct. When building requests, the datetime fields should not need any special construction and users should be able to pass in the usual `DateTime` (`chrono`), `OffsetDateTime` (`time`), or `String` (default) structs and the `into()` method should be able to convert them to a `MangaDexDateTime` struct automatically. This is different from the `Username` and `Password` structs because they have built-in validation that will prevent those requests from being sent to MangaDex if they don't meet the requirements; the `MangaDexDateTime` struct is just a wrapper around the feature's respective type.

## Fixed

- [8177388] Fixed the `upload` feature so that it compiles.

## Internal

- ([dbd1256]) Update the project to use the Rust 2021 edition.
- ([7392281]) Temporarily remove `cargo-audit` from the CI/CD pipeline because of `chrono`'s security advisories. This library will continue to support `chrono` integration for the forseeable future but will not include it as a default feature.
- ([f92398f]) Consolidated the `language` module to use a macro to improve maintainability. This makes it so that new languages that are added only need to be added in a single location instead of having to copy the variant multiple times across the various function implementations.

[12b983b]: https://gitlab.com/gondolyr/mangadex-api/-/commit/12b983b
[228dec9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/228dec9
[4dc82a2]: https://gitlab.com/gondolyr/mangadex-api/-/commit/4dc82a2
[64da46a]: https://gitlab.com/gondolyr/mangadex-api/-/commit/64da46a
[71e26b3]: https://gitlab.com/gondolyr/mangadex-api/-/commit/71e26b3
[7392281]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7392281
[7ca3b80]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7ca3b80
[8177388]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8177388
[92a4313]: https://gitlab.com/gondolyr/mangadex-api/-/commit/92a4313
[93e7558]: https://gitlab.com/gondolyr/mangadex-api/-/commit/93e7558
[a8c8a78]: https://gitlab.com/gondolyr/mangadex-api/-/commit/a8c8a78
[cead1f2]: https://gitlab.com/gondolyr/mangadex-api/-/commit/cead1f2
[d0d42ed]: https://gitlab.com/gondolyr/mangadex-api/-/commit/d0d42ed
[dbd1256]: https://gitlab.com/gondolyr/mangadex-api/-/commit/dbd1256
[f92398f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/f92398f

# Version 1.0.0-alpha.6 (2021-10-12)

All the new endpoint additions were added in MangaDex 5.3.4.

## Added

- ([d101a343]) Added `MangaState` enum to hold `state` field values. The new `state` field describes the staff approval status for a manga. When a manga is created, it requires staff approval for it to appear to the public.
- ([cca78131], [3476da18]) Added the get a specific Manga Draft endpoint (`GET /manga/draft/{id}`). This allows users to fetch a specific manga that isn't published.
- ([80929934], [3476da18]) Added submit Manga Draft endpoint (`POST /manga/draft/{id}/commit`). This allows users to change the state of a "draft" Manga to "submitted" for staff approval.
- ([039bdd29], [3476da18]) Added search Manga Drafts endpoint (`GET /manga/draft`). This allows users to search for Manga that have not been published.
- ([c8f2b1bd]) Added Manga relation list endpoint (`GET /manga/{id}/relation`). This endpoint fetches the related manga for a specific Manga such as spin-offs and sequels.
- ([bf74e422]) Added create a Manga relation endpoint (`POST /manga/{id}/relation`). This endpoint adds a relationship between Manga such as specifying one is a spin-off of the main story.
- ([c3503f43]) Added delete a Manga relation endpoint (`DELETE /manga/{id}/relation`). This endpoint removes the relationship between Manga.

## Changed

- ([fdcc8b89]) [**BREAKING**] Renamed `MangaRelated` enum to `MangaRelation`.
This enum wasn't really used outside of a single GET endpoint, so I don't expect much use of this directly.
With how the new MangaDex 5.3.4 endpoints are using this enum, it makes
more sense to call it a Manga relation, semantically speaking, because
it is a noun describing the entity, not a verb.
While this is bikeshedding, I feel more comfortable with this naming and prefer to
make this breaking change now rather than later when it is now used for
more than just a field type from a response.

## Internal

- ([a8401d8e]) Removed the ignore attribute from the list HTTP 400 tests. These tests are now run when the project tests are run.
- ([9990ccec]) Removed unused `Serialize`/`Deserialize` traits from `CreateManga` request struct and `NoData` struct.

[039bdd29]: https://gitlab.com/gondolyr/mangadex-api/-/commit/039bdd29
[3476da18]: https://gitlab.com/gondolyr/mangadex-api/-/commit/3476da18
[80929934]: https://gitlab.com/gondolyr/mangadex-api/-/commit/80929934
[9990ccec]: https://gitlab.com/gondolyr/mangadex-api/-/commit/9990ccec
[a8401d8e]: https://gitlab.com/gondolyr/mangadex-api/-/commit/a8401d8e
[bf74e422]: https://gitlab.com/gondolyr/mangadex-api/-/commit/bf74e422
[c3503f43]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c3503f43
[c8f2b1bd]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c8f2b1bd
[cca78131]: https://gitlab.com/gondolyr/mangadex-api/-/commit/cca78131
[d101a343]: https://gitlab.com/gondolyr/mangadex-api/-/commit/d101a343
[fdcc8b89]: https://gitlab.com/gondolyr/mangadex-api/-/commit/fdcc8b89

# Version 1.0.0-alpha.5 (2021-10-06)

## Added

- ([da763d39]) Added the `focusLanguage` query parameter and response body field to the scanlation group list and update endpoint. The field was added in MangaDex 5.3.1.
- ([8cdbb45f]) Added the `altNames` field to the scanlation group response. The field was added in MangaDex 5.3.2.
- ([50e3444b]) Added social media fields to the author response body struct. These fields were added in MangaDex 5.3.2.
- ([48b0f8f3]) Added social media fields to the author create and update endpoint request body structs. These fields were added in MangaDex 5.3.2.
- ([2b520c6e]) Added `related` field to the `Relationship` struct. This field was added in MangaDex 5.3.3 and only appears for Manga entities and Manga relationships.
- ([6a114dd0]) Added the relationship's `attributes` field when [Reference Expansion][docs-main-reference-expansion] is used.

## Changed

- ([28faf433]) Pass the underlying error context to the library's error context message. For example, if there was a `Reqwest` error, the `Reqwest` error message will be passed along with this library's custom error message to provide better transparency into what went wrong.

## Fixed

- ([828db141], [7debff8a] - Thanks @evetsso) Fixed how datetime fields are serialized with requests to match the `YYYY-MM-DDTHH:MM:SS` format the MangaDex API is expectation. The `chrono` library serializes to RFC3339/ISO 8601 by default, which is not the format MangaDex accepts.

## Internal

- ([ec02dce9]) Added a `rustfmt.toml` configuration to the project to ensure Rust code is formatted to a specific standard.
- ([037719ce]) Added a cargo-deny config (`deny.toml`) to ensure dependencies are secure and currently maintained. For now, this isn't used in the CI/CD pipeline due to the `ring` dependency being unmaintained from this library's use of `rustls`; there are also license parsing errors in some dependencies due to the projects' multi-license setup.
- ([9ce6444c]) Added the `target/` directory to the GitLab CI/CD cache to improve the CI/CD performance.
- ([43047625]) Removed the `Serialize` trait from the response structs. This should slightly improve compile-time performance and slightly reduce the binary size, though not in a significant way.

[037719ce]: https://gitlab.com/gondolyr/mangadex-api/-/commit/037719ce
[28faf433]: https://gitlab.com/gondolyr/mangadex-api/-/commit/28faf433
[2b520c6e]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2b520c6e
[43047625]: https://gitlab.com/gondolyr/mangadex-api/-/commit/43047625
[48b0f8f3]: https://gitlab.com/gondolyr/mangadex-api/-/commit/48b0f8f3
[50e3444b]: https://gitlab.com/gondolyr/mangadex-api/-/commit/50e3444b
[6a114dd0]: https://gitlab.com/gondolyr/mangadex-api/-/commit/6a114dd0
[7debff8a]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7debff8a
[828db141]: https://gitlab.com/gondolyr/mangadex-api/-/commit/828db141
[8cdbb45f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8cdbb45f
[9ce6444c]: https://gitlab.com/gondolyr/mangadex-api/-/commit/9ce6444c
[da763d39]: https://gitlab.com/gondolyr/mangadex-api/-/commit/da763d39
[ec02dce9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/ec02dce9

# Version 1.0.0-alpha.4 (2021-09-15)

This release fixes the inconsistent list response body structure that the MangaDex API returned prior to MangaDex 5.2.35.
Many breaking changes have been introduced, primarily to the list endpoints.

## Added

- [Add `includes[]` query parameter to the user followed groups endpoint (`GET /user/follows/group`).][d3b8baff]
- [Add `includes[]` query parameter to the user followed manga endpoint (`GET /user/follows/manga`).][2117471b]
- [Add `followedCount` and `relevance` sort categories for the manga list endpoint.][b3fc3c78]
- [Add the batch mark manga chapters endpoint (`POST /manga/{id}/read`).][fe55aafa]
- [Add the `id` field for the chapter aggregate response struct (`GET /manga/{id}/aggregate`).][b6bbfe12]
- [Add `includeFutureUpdates` query parameter to the chapter list endpoint (`GET /chapter`).][bc119da2]
- [Add `includeFutureUpdates` query parameter to the user followed manga feed endpoint (`GET /user/follows/manga/feed`).][c868ec44]
- [Add `includeFutureUpdates` query parameter to the custom list manga feed endpoint (`GET /list/{id}/feed`).][25b44425]
- [Add `includeFutureUpdates` query parameter to the manga feed endpoint (`GET /manga/{id}/feed`).][3dd833d3]
- [Add `result` field to the response struct for the AtHome server endpoint (`GET /at-home/server/{id}`).][37c994f1]

## Changed

- \[**BREAKING**\] [MangaDex now returns list responses in the same structure as single lookups as of MangaDex 5.2.35. The following endpoints have been affected:][d0a0fa2d]
    - List/search authors (`GET /author`)
    - List/search chapters (`GET /chapter`)
    - List/search covers (`GET /cover`)
    - List/search custom lists (`GET /list`)
    - List/search manga (`GET /manga`)
    - List/search scanlation groups (`GET /group`)
    - List/search users (`GET /user`)
    - Custom list manga feed (`GET /list/{id}/feed`)
    - Legacy ID mapping (`POST /legacy/mapping`)
    - Manga feed (`GET /manga/{id}/feed`)
    - List manga tags (`GET /manga/tag`)
    - List report reasons (`GET /report/reasons/{category}`)
    - List user custom lists (`GET /user/{id}/list`)
    - List my followed groups (`GET /user/follows/group`)
    - List my followed manga (`GET /user/follows/manga`)
    - List my followed manga feed (`GET /user/follows/manga/feed`)
    - List my followed users (`GET /user/follows/user`)
    - List my custom lists (`GET /user/list`)

[2117471b]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2117471b
[25b44425]: https://gitlab.com/gondolyr/mangadex-api/-/commit/25b44425
[37c994f1]: https://gitlab.com/gondolyr/mangadex-api/-/commit/37c994f1
[3dd833d3]: https://gitlab.com/gondolyr/mangadex-api/-/commit/3dd833d3
[b3fc3c78]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b3fc3c78
[b6bbfe12]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b6bbfe12
[bc119da2]: https://gitlab.com/gondolyr/mangadex-api/-/commit/bc119da2
[c868ec44]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c868ec44
[d0a0fa2d]: https://gitlab.com/gondolyr/mangadex-api/-/commit/d0a0fa2d
[d3b8baff]: https://gitlab.com/gondolyr/mangadex-api/-/commit/d3b8baff
[fe55aafa]: https://gitlab.com/gondolyr/mangadex-api/-/commit/fe55aafa

# Version 1.0.0-alpha.3 (2021-09-01)

This version addresses the breaking changes in MangaDex 5.2.24.
The biggest changes are to the create scanlation group request fields and the move of the `relationships` JSON response field from the top-level response to inside the `data` field.

## Added

- [Added the `groups[]` query parameter to the aggregate manga endpoint (Get manga volumes & chapters)][6db1f424]

## Changed

- \[**BREAKING**\] [MangaDex now returns the `relationships` array inside the `data` object. The following endpoints have had their response structs adjusted:][a40aad35]
    - Create an author (`POST /author`)
    - Get an author (`GET /author/{id}`)
    - List/search authors (`GET /author`)
    - Update an authors (`PUT /author/{id}`)
    - Get a chapter (`GET /chapter/{id}`)
    - List/search chapters (`GET /chapter`)
    - Update a chapter (`PUT /chapter/{id}`)
    - Edit a cover (`PUT /cover/{id}`)
    - Get a cover (`GET /cover/{id}`)
    - List/search covers (`GET /cover`)
    - Upload a cover (`POST /cover`)
    - Create a custom list (`POST /list`)
    - Get a custom list (`GET /list/{id}`)
    - Get a custom list manga feed (`GET /list/{id}/feed`)
    - Update a custom list (`PUT /list/{id}`)
    - Get legacy ID mapping (`POST /legacy/mapping`)
    - Create a manga (`POST /manga`)
    - Manga feed (`GET /manga/{id}/feed`)
    - Get a manga (`GET /manga/{id}`)
    - List/search manga (`GET /manga`)
    - List manga tags (`GET /manga/tag`)
    - Get a random manga (`GET /manga/random`)
    - Update a manga (`PUT /manga/{id}`)
    - Create a scanlation group (`POST /group`)
    - Get a scanlation group (`GET /group/{id}`)
    - List/search scanlation groups (`GET /group`)
    - Update a scanlation group (`PUT /group/{id}`)
    - Commit an upload session (`PUT /upload/{id}/commit`)
    - Upload images to an upload session (`POST /upload/{id}`)
    - Get the logged-in user's followed groups (`GET /user/follows/group`)
    - Get the logged-in user's followed manga (`GET /user/follows/manga`)
    - Get the logged-in user's followed manga feed (`GET /user/follows/manga/feed`)
- \[**BREAKING**\] [Updated the create scanlation group request body fields. MangaDex updated the request body fields in 5.2.24.][2f08cde1]

## Internal

- [Rename the `v5_cli` example to `v5_demo`. The use of CLI implied that it did more than simply demonstrate a couple of fetch endpoint and print the response bodies.][c1fbfe68]

[2f08cde1]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2f08cde1
[6db1f424]: https://gitlab.com/gondolyr/mangadex-api/-/commit/6db1f424
[a40aad35]: https://gitlab.com/gondolyr/mangadex-api/-/commit/a40aad35
[c1fbfe68]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c1fbfe68

# Version 1.0.0-alpha.2 (2021-08-31)

This version addresses the changes introduced in MangaDex 5.2.23.
The most notable change in this version of MangaDex is the additional language query parameters such as `originalLanguage[]` and `excludedOriginalLanguage[]`.

## Added

- [The `excludedTranslatedLanguage[]` and `availableTranslatedLanguage[]` query parameters were added to the list manga endpoint.][4279a416]
- [The `originalLanguage[]` and `excludedOriginalLanguage[]` query parameters were added to the list chapter endpoint.][c9f686ea]
- [The `originalLanguage[]` and `excludedOriginalLanguage[]` query parameters were added to the user followed manga feed endpoint.][14ca38a9]
- [The `originalLanguage[]` and `excludedOriginalLanguage[]` query parameters were added to the CustomList manga feed endpoint.][2cc5c7fa]
- [The `originalLanguage[]` and `excludedOriginalLanguage[]` query parameters were added to the manga feed endpoint.][67650978]

## Changed

- \[**BREAKING**\] [The manga response (`MangaAttributes`) `status` field is no longer marked as nullable according to the MangaDex docs. The `Option` wrapper has been removed as a result.][ab72edd0]
- \[**BREAKING**\] [The `originalLanguage`, `status`, and `contentRating` fields for creating a new manga entry are now required by MangaDex.][00ffd568]

## Fixed

- [The `TagSearchMode` enum now serializes into uppercase as the MangaDex API requires it to be.][83b60431]

## Internal

- [The sort order enums now use a macro to define them. All of the enums' variants are tuple structs that use the `OrderDirection` enum so using a macro ensures that every enum is consistent.][2b30cb93]

[00ffd568]: https://gitlab.com/gondolyr/mangadex-api/-/commit/00ffd568
[14ca38a9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/14ca38a9
[2b30cb93]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2b30cb93
[2cc5c7fa]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2cc5c7fa
[4279a416]: https://gitlab.com/gondolyr/mangadex-api/-/commit/4279a416
[67650978]: https://gitlab.com/gondolyr/mangadex-api/-/commit/67650978
[83b60431]: https://gitlab.com/gondolyr/mangadex-api/-/commit/83b60431
[ab72edd0]: https://gitlab.com/gondolyr/mangadex-api/-/commit/ab72edd0
[c9f686ea]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c9f686ea

# Version 1.0.0-alpha.1 (2021-08-30)

The main additions to this version are multi-threaded support and support for the new fields in the query parameters, request bodies, and response bodies added after 5.2.20.
With multi-threaded support, you can use this library in applications that require the use of multiple threads such as GUI applications. Just enable the `multi-thread` feature, and you will be able to make async requests in multiple threads.

## Added

- [Error messages have been modified to use all lowercase and no ending punctuation to respect the idea that "you are not necessarily the start of the sentence" because the returned error may be printed as the cause of some other error.][711421eb]
- [Add support for multi-threaded operations by using `Arc` and `Mutex`.][e329e6f5] [This is hidden behind the `multi-thread` feature and is not enabled by default because it is experimental and also has a very minor performance hit compared to using `Rc` and `RefCell`.][c7a727bf]
- [Add the `contentRating[]` query parameter to the chapter list endpoint.][4a6cd87d]
- [Add the `contentRating[]` and `includes[]` query parameters to the user manga feed endpoint.][897314e9]
- [Add the `contentRating[]` and `includes[]` query parameters to the CustomList manga feed endpoint.][8c2f47ce]
- [Add the `volume` field to the upload cover endpoint request body.][db8b8171]
- [Add the `contentRating[]` query parameter to the manga feed endpoint.][de6521c9]
- [Add `official` and `verified` scanlation group response fields (`ScanlationGroupAttributes`). The `verified` field is not included in the API documentation as of 5.2.22 but is returned in the response body.][8a9483d5]
- [Add the `roles` field to the user response fields (`UserAttributes`). A `UserRole` enum has been started but because not all of the different variants are known, it is currently acting as a placeholder with the current discovered values.][7b21d2c2]
- [Add the `derive_builder::UninitializedFieldError` error as an internal error variant so that `.build()?` can be used if the `mangadex_api` `Error` enum is used in an application as a return type.][b657005e]

## Changed

- [The `MangaStatus` enum `Display` implementation now outputs title-case (e.g. "This is Title Case") for flexible usability. Some applications may want to print out the status from a response and this makes it convenient to do so.][ebfcb5de]

## Documentation

- [The `lib.rs` module docblock has been moved over to the `README.md` file and now imports the `README.md` file so that the library's frontpage documentation is centralized.][f8c9c3ab]

## Internal

- [A type alias for the `HttpClient` reference counter has been added. This makes it easier to maintain the endpoints that use this so that it is centralized to a single location.][26164628]

[26164628]: https://gitlab.com/gondolyr/mangadex-api/-/commit/26164628
[4a6cd87d]: https://gitlab.com/gondolyr/mangadex-api/-/commit/4a6cd87d
[711421eb]: https://gitlab.com/gondolyr/mangadex-api/-/commit/711421eb
[7b21d2c2]: https://gitlab.com/gondolyr/mangadex-api/-/commit/7b21d2c2
[897314e9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/897314e9
[8a9483d5]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8a9483d5
[8c2f47ce]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8c2f47ce
[b657005e]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b657005e
[c7a727bf]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c7a727bf
[db8b8171]: https://gitlab.com/gondolyr/mangadex-api/-/commit/db8b8171
[de6521c9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/de6521c9
[e329e6f5]: https://gitlab.com/gondolyr/mangadex-api/-/commit/e329e6f5
[ebfcb5de]: https://gitlab.com/gondolyr/mangadex-api/-/commit/ebfcb5de
[f8c9c3ab]: https://gitlab.com/gondolyr/mangadex-api/-/commit/f8c9c3ab

# Version 1.0.0-alpha.0 (2021-08-23)

I am pleased to announce the first public release of the rewrite of `mangadex-api`, which has feature parity with the [v5 MangaDex API][docs-api-url]!

The ergonomics of the library are still being adjusted but all of the available public endpoints (as of 5.2.20) are available for use with this library. While the library is in its 1.0.0 alpha state, small adjustments and fixes are expected before a 1.0.0 release.

A major change is the error handling where more descriptive errors are provided.

The "Upload" category is hidden behind the `upload` feature because it is experimental but should be overall functional.

Please report any issues or suggestions to the project at https://gitlab.com/gondolyr/mangadex-api/-/issues.

# Version 0.2.6 (2021-03-21)

## Changed

- [Add basic error handling when the MangaDex servers return an HTTP error code][ee414061]
    - This was discovered when MangaDex required extended maintenance time to resolve issues on their end.

[ee414061]: https://gitlab.com/gondolyr/mangadex-api/-/commit/ee414061c772bace7ed441b66ecb1a9186aac50c

# Version 0.2.5 (2021-03-11)

## Added

- [`Display` trait on the Demographic and PublicationStatus enums][c3744870]

## Changed

- [Blank manga artists are filtered out from the v2 `/manga/{id}` endpoint response so that consumers of this library don't have to do it themselves][0ca81a38]

## Documentation

- [Group `delay` field now has a more accurate description][172937df]
- [Clarified the `md_at_home` field's purpose][2ebd4663]
- [Added descriptions for manga response fields][1633138c]
- [Added a description for the `TagData.group` partial struct field][a7021282]

## Fixed

- [The `GroupChaptersResponse` struct now uses the `ChaptersDataPartial` struct for the `data` field to match the API's response structure][10ae6be7]
- [The `group_chapters()` v2 client builder method now uses the correct `GroupId` type alias for the `id` parameter][43800678]

[c3744870]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c374487066ab2f3d151aa42ec394816876250180
[0ca81a38]: https://gitlab.com/gondolyr/mangadex-api/-/commit/0ca81a3807966846ca807066c8ebb20da22a98dc
[10ae6be7]: https://gitlab.com/gondolyr/mangadex-api/-/commit/10ae6be7b574470a1ddef7b9a30350fecdd7f0a9
[43800678]: https://gitlab.com/gondolyr/mangadex-api/-/commit/43800678508be9a84ae31e09748c43ede29b268b
[172937df]: https://gitlab.com/gondolyr/mangadex-api/-/commit/172937df8d13fa6498c3d00e0f6168e795e28e80
[2ebd4663]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2ebd4663bdb657c360ddb5f471d69ec8a4d6b790
[1633138c]: https://gitlab.com/gondolyr/mangadex-api/-/commit/1633138cbb974d57d3c64f10af3b9267d3d94044
[a7021282]: https://gitlab.com/gondolyr/mangadex-api/-/commit/a7021282112b3a7ca838b2f98ace14bdc2f84c3c

# Version 0.2.4 (2021-03-05)

## Changed

- [Web scrape clients include `Send + Sync` traits][f8008011]

## Documentation

- [Add flags to include all features and document private items when publishing to docs.rs][5a4f7a2f]

[5a4f7a2f]: https://gitlab.com/gondolyr/mangadex-api/-/commit/5a4f7a2f69c434aeeeb76a114ccce58613cf49db
[f8008011]: https://gitlab.com/gondolyr/mangadex-api/-/commit/f80080119bad511b9037f20249480c5d398fc53e

# Version 0.2.3 (2021-01-16)

## Added

- [`thread_id` field to `ChapterPartial` struct][1a6cf334]
- [Added `read` field to the v2 followed-updates endpoint response][409d61df]
- [Added `blockgroups` query parameter to all endpoints that return a list of chapters][c6e7e2db]. Enabling this filters out chapters belonging to groups that the user has blocked.
    - `/v2/group/{id}/chapters`
    - `/v2/manga/{id}/chapters`
    - `/v2/user/{id|me}/chapters`
    - `/v2/user/{id|me}/followed-updates`
- [Added `type` and `hentai` parameters to the v2 followed-manga endpoint][e8689022]. These are the same parameters that the followed-updates endpoint permitted.

## Changed

- [Base API URL now points to `api.mangadex.org`][c2870def]
- [The followed-updates response struct now uses the `title` field instead of `name`][6bdada80]. The `name()` getter has been preserved to maintain compatiblity for those using `name()`.

## Documentation

- [Add `threadId` field to the v2 OpenAPI document][a9284838]

## Fixed

- ["external" chapter status types are able to be parsed][48513b38]
- [The response struct for the v2 manga chapters endpoint (`/v2/manga/{id}/chapters`) has been fixed][71851fa5]. Previously, it was using the wrong struct for the `data` field.

[c2870def]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c2870def1f9e509449e147f7de0d6abcf2274f0d
[6bdada80]: https://gitlab.com/gondolyr/mangadex-api/-/commit/6bdada801b9a9a829c4e46bd15f2872b20c3aa3c
[409d61df]: https://gitlab.com/gondolyr/mangadex-api/-/commit/409d61dfbbdb2657707819af10ef36f38d00e1d1
[48513b38]: https://gitlab.com/gondolyr/mangadex-api/-/commit/48513b385fa55580a683ccb8a48ff45327064860
[c6e7e2db]: https://gitlab.com/gondolyr/mangadex-api/-/commit/c6e7e2db7b5bc2356d5cdb2d8de7f3fca5dc376e
[e8689022]: https://gitlab.com/gondolyr/mangadex-api/-/commit/bd01b61ff69f2a3eb8ea39ab94ea331cce4ff52c
[71851fa5]: https://gitlab.com/gondolyr/mangadex-api/-/commit/71851fa5e27ee97ce8e14aa09bf86fc481d80c81
[1a6cf334]: https://gitlab.com/gondolyr/mangadex-api/-/commit/1a6cf334ef64800af22f7548d12531326b2dbc6c
[a9284838]: https://gitlab.com/gondolyr/mangadex-api/-/commit/a928483880bca18c72b5ca051858c4cd98e752f6

# Version 0.2.2 (2021-01-08)

## Dependencies

- [Update Reqwest to 0.11][b8e82c6]
- [Update Tokio to 1.0][b8e82c6]

[b8e82c6]: https://gitlab.com/gondolyr/mangadex-api/-/commit/b8e82c6d414717dabfa8c6650f6bf52f16c4dc99

# Version 0.2.0 (2020-12-29)

The web scraping search, fetch latest updates, and most popular manga functionality
all return a struct with the manga ID, title, and cover image URL.

## Added

- [Added `SiteTheme` enum for choosing the website theme][fc45db0]
- [Added functionality to fetch the latest manga updates via web scraping][bcfd464]
- [Added search functionality via web scraping][84216cb]
- [Added functionality to fetch the most popular manga (highest Bayesian rating) via web scraping][2d616df]
- [Added a `builder()` method to the `MangaDexV2` struct to allow for API configuration for things such as the user agent][f5874bd]
- [Added `tags()` method to `TagCategory` enum to return the associated tags with the category][0375fe6]

## Changed

- \[BREAKING\] [Changed `Language` default value to `English` because MangaDex appears to default to this language][9804cab]
- \[BREAKING\] [Make "time"/"Chrono" feature on by default to make it easier to work with dates and times][66845e9]
- [Changed enum function arguments to accept generic values for improved flexibility][8f296ef]
- [Make `Language` enum `From` trait implementations case-insensitive][4b522b9]

## Examples

- [Added login example to README][2705240]

## Fixed

- [Fixed `Language::Other` integer value][9804cab]

## Internal Only

- [Implement `FromStr` trait on Language enum][2c4d698]
- [Removed patch versions from Cargo.toml because the latest patch fixes should not include any breaking
  changes and should instead bring the latest fixes][2891a55]

[2c4d698]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2c4d69860f729579bcffbfaacdd11c3bf0d79421
[fc45db0]: https://gitlab.com/gondolyr/mangadex-api/-/commit/fc45db08c500cec8a665351a4df3dd90e80f664a
[9804cab]: https://gitlab.com/gondolyr/mangadex-api/-/commit/9804cab889284de1a463bedf8fd6e33fa945930d
[bcfd464]: https://gitlab.com/gondolyr/mangadex-api/-/commit/bcfd4641c9ad74b88c7c2783fdb2af6e1776f4d6
[84216cb]: https://gitlab.com/gondolyr/mangadex-api/-/commit/84216cbe3a930bdf6db1f271c1dd0749d42a0b9a
[8f296ef]: https://gitlab.com/gondolyr/mangadex-api/-/commit/8f296efaa8a0583bf9722229de773bd8abfb468b
[66845e9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/66845e9dc3fffed4289353bcf7e68f002f65343b
[2d616df]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2d616dfdab870eacb9e1b348b2c6cf1da129c0b6
[f5874bd]: https://gitlab.com/gondolyr/mangadex-api/-/commit/f5874bd6e5fb826e99beec7cd56a89b86c917678
[4b522b9]: https://gitlab.com/gondolyr/mangadex-api/-/commit/4b522b9adf8d8609e8f64a3943a49db484db882d
[2891a55]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2891a55a1991916b25d1c7118fe6c038216fa0ec
[0375fe6]: https://gitlab.com/gondolyr/mangadex-api/-/commit/0375fe6fc46343ff4a07a38b1053c6aeb9d3819c
[2705240]: https://gitlab.com/gondolyr/mangadex-api/-/commit/27052400efdc3feecb6c3ce56c5505764cbf01d5

# Version 0.1.1 (2020-12-11)

## Improvements

- [Added `Copy` trait to internal MangaDex types.][ac533c0a]
- [Added `Hash` and `Serialize` to internal MangaDex types.][135f2c69]

## Documentation

- [Added GitLab pages job to publish Cargo-generated docs onto project website.][4d9065e1]
- [Fixed README example and v2 client docblocks return types.][f2197738]
- [Added Contributing guide.][21da4ca6]

## Examples

- [Added manga cover image downloader.][e9980599]
- [Added chapter downloader.][2f04b936]

## Internal Only

- [Moved `TagCategory` out of `tag` module to `tag_category`.][883b0e15]
- [Removed `getset` crate and implemented own getters.][da206ffc]

[4d9065e1]: https://gitlab.com/gondolyr/mangadex-api/-/commit/4d9065e180c87f1bd4be781171714b6d52e2f9ed
[f2197738]: https://gitlab.com/gondolyr/mangadex-api/-/commit/f219773828538783267eed7cf0534724735566c9
[21da4ca6]: https://gitlab.com/gondolyr/mangadex-api/-/commit/21da4ca6c12f282b086a835760e9616174242b67
[e9980599]: https://gitlab.com/gondolyr/mangadex-api/-/commit/e9980599c50073d621edc31e85cd6a6c89764788
[ac533c0a]: https://gitlab.com/gondolyr/mangadex-api/-/commit/ac533c0a05183d4e678387dd148551a4bc575fdf
[883b0e15]: https://gitlab.com/gondolyr/mangadex-api/-/commit/883b0e158bcc8835be075c10803c42362c5f3fe1
[135f2c69]: https://gitlab.com/gondolyr/mangadex-api/-/commit/135f2c69b39ea304ed0e29e3b0f301c668d08c6d
[2f04b936]: https://gitlab.com/gondolyr/mangadex-api/-/commit/2f04b93667390b28729d910759f206670819780b
[da206ffc]: https://gitlab.com/gondolyr/mangadex-api/-/commit/da206ffca29657173dd1f28e2a475468657fc9b2

# Version 0.1.0 (2020-12-06)

## Features

- [All (most) MangaDex v2 endpoints implemented as of 2020-12-06.][0.1.0-endpoints]
  - API index page is not implemented. At this time, there is no intention to implement this unless there is strong demand.
- [Can log in to access restricted endpoints.][d8f794f8]
- [Chrono is a feature that will transmute various date/time fields as Chrono types][9d71c29b]
- [Example CLI tool for calling all the implemented v2 endpoints.][6c1f024c]
    - This does not have a lot of customization as it is designed to demonstrate basic usage.
- [Many MangaDex types have been internalized to Rust enums and structs for ease-of-use.][0.1.0-types]

[9d71c29b]: https://gitlab.com/gondolyr/mangadex-api/-/commit/9d71c29b3ca41201fe89b3fd351cc8792a522522
[6c1f024c]: https://gitlab.com/gondolyr/mangadex-api/-/commit/6c1f024cde9cd396e0fd0b260b0f82c455eccd40
[d8f794f8]: https://gitlab.com/gondolyr/mangadex-api/-/commit/d8f794f860f8b93c306fd52e7ba61d013c88bc1f
[0.1.0-endpoints]: https://gitlab.com/gondolyr/mangadex-api/-/tree/0.1.0/src/v2/builder
[0.1.0-types]: https://gitlab.com/gondolyr/mangadex-api/-/tree/0.1.0/src/types

[crates-chrono]: https://crates.io/crates/chrono
[crates-time]: https://crates.io/crates/time
[crates-tokio]: https://crates.io/crates/tokio

[docs-api-docs-url]: https://api.mangadex.org/swagger.html
[docs-api-url]: https://api.mangadex.org

[docs-account-username-available-endpoint]: https://api.mangadex.org/swagger.html#/Account/get-account-available

[docs-athome-get-chapter-server-endpoint]: https://api.mangadex.org/swagger.html#/AtHome/get-at-home-server-chapterId

[docs-author-create-endpoint]: https://api.mangadex.org/swagger.html#/Author/post-author
[docs-author-update-endpoint]: https://api.mangadex.org/swagger.html#/Author/put-author-id
[docs-author-view-endpoint]: https://api.mangadex.org/swagger.html#/Author/get-author-id

[docs-chapter-list-endpoint]: https://api.mangadex.org/swagger.html#/Chapter/get-chapter
[docs-chapter-update-endpoint]: https://api.mangadex.org/swagger.html#/Chapter/put-chapter-id
[docs-chapter-view-endpoint]: https://api.mangadex.org/swagger.html#/Chapter/get-chapter-id

[docs-cover-section]: https://api.mangadex.org/swagger.html#/Cover
[docs-cover-delete-endpoint]: https://api.mangadex.org/swagger.html#/Cover/delete-cover
[docs-cover-edit-endpoint]: https://api.mangadex.org/swagger.html#/Cover/edit-cover
[docs-cover-get-endpoint]: https://api.mangadex.org/swagger.html#/Cover/get-cover-id
[docs-cover-list-endpoint]: https://api.mangadex.org/swagger.html#/Cover/get-cover
[docs-cover-upload-endpoint]: https://api.mangadex.org/swagger.html#/Cover/upload-cover
[docs-cover-view-endpoint]: https://api.mangadex.org/swagger.html#/Cover/get-cover-id

[docs-customlist-follow-custom-list-endpoint]: https://api.mangadex.org/swagger.html#/CustomList/follow-list-id
[docs-customlist-unfollow-custom-list-endpoint]: https://api.mangadex.org/swagger.html#/CustomList/unfollow-list-id
[docs-customlist-manga-feed-endpoint]: https://api.mangadex.org/swagger.html#/CustomList/get-list-id-feed

[docs-feed-user-followed-manga-endpoint]: https://api.mangadex.org/swagger.html#/Feed/get-user-follows-manga-feed

[docs-follows-is-following-custom-list-endpoint]: https://api.mangadex.org/swagger.html#/Follows/get-user-follows-list-id
[docs-follows-user-following-custom-lists-endpoint]: https://api.mangadex.org/swagger.html#/Follows/get-user-follows-list
[docs-follows-user-following-manga-endpoint]: https://api.mangadex.org/swagger.html#/Follows/get-user-follows-manga-id
[docs-follows-user-following-scanlation-group-endpoint]: https://api.mangadex.org/swagger.html#/Follows/get-user-follows-group-id
[docs-follows-user-following-user-endpoint]: https://api.mangadex.org/swagger.html#/Follows/get-user-follows-user-id

[docs-main-reading-a-chapter]: https://api.mangadex.org/docs/reading-chapter/
[docs-main-reference-expansion]: https://api.mangadex.org/docs/reference-expansion/
[docs-main-staticdata-manga-related-enum]: https://api.mangadex.org/docs/static-data/#manga-related-enum

[docs-manga-endpoint-section]: https://api.mangadex.org/swagger.html#/Manga
[docs-manga-aggregate-endpoint]: https://api.mangadex.org/swagger.html#/Manga/get_manga__id__aggregate
[docs-manga-create-endpoint]: https://api.mangadex.org/swagger.html#/Manga/post-manga
[docs-manga-feed-endpoint]: https://api.mangadex.org/swagger.html#/Manga/get-manga-id-feed
[docs-manga-get-draft-endpoint]: https://api.mangadex.org/swagger.html#/Manga/get-manga-id-draft
[docs-manga-list-drafts-endpoint]: https://api.mangadex.org/swagger.html#/Manga/get-manga-drafts
[docs-manga-list-endpoint]: https://api.mangadex.org/swagger.html#/Manga/get-search-manga
[docs-manga-list-relation-endpoint]: https://api.mangadex.org/swagger.html#/Manga/get-manga-relation
[docs-manga-random-endpoint]: https://api.mangadex.org/swagger.html#/Manga/get-manga-random
[docs-manga-submit-draft-endpoint]: https://api.mangadex.org/swagger.html#/Manga/commit-manga-draft
[docs-manga-update-endpoint]: https://api.mangadex.org/swagger.html#/Manga/put-manga-id
[docs-manga-view-endpoint]: https://api.mangadex.org/swagger.html#/Manga/get-manga-id

[docs-rating-create-update-manga-endpoint]: https://api.mangadex.org/swagger.html#/Rating/post-rating-manga-id
[docs-rating-delete-manga-endpoint]: https://api.mangadex.org/swagger.html#/Rating/delete-rating-manga-id
[docs-rating-get-manga-endpoint]: https://api.mangadex.org/swagger.html#Rating/get-rating

[docs-report-list-endpoint]: https://api.mangadex.org/swagger.html#/Report/get-reports

[docs-scanlationgroup-section]: https://api.mangadex.org/swagger.html#/ScanlationGroup
[docs-scanlationgroup-create-endpoint]: https://api.mangadex.org/swagger.html#/ScanlationGroup/post-group
[docs-scanlationgroup-list-endpoint]: https://api.mangadex.org/swagger.html#/ScanlationGroup/get-search-group
[docs-scanlationgroup-update-endpoint]: https://api.mangadex.org/swagger.html#/ScanlationGroup/put-group-id
[docs-scanlationgroup-view-endpoint]: https://api.mangadex.org/swagger.html#/ScanlationGroup/get-group-id

[docs-settings-section]: https://api.mangadex.org/swagger.html#/Settings
[docs-settings-create-template-endpoint]: https://api.mangadex.org/swagger.html#/Settings/post-settings-template
[docs-settings-create-update-user-endpoint]: https://api.mangadex.org/swagger.html#/Settings/post-settings
[docs-settings-get-latest-template-endpoint]: https://api.mangadex.org/swagger.html#/Settings/get-settings-template
[docs-settings-get-template-endpoint]: https://api.mangadex.org/swagger.html#/Settings/get-settings-template-version
[docs-settings-get-user-endpoint]: https://api.mangadex.org/swagger.html#/Settings/get-settings

[docs-statistics-get-manga-endpoint]: https://api.mangadex.org/swagger.html#/Statistics/get-statistics-manga

[docs-upload-commit-endpoint]: https://api.mangadex.org/swagger.html#/Upload/commit-upload-session

[local-contributing]: CONTRIBUTING.md
[local-readme]: README.md

[RUSTSEC-2020-0071]: https://rustsec.org/advisories/RUSTSEC-2020-0071.html
[RUSTSEC-2020-0159]: https://rustsec.org/advisories/RUSTSEC-2020-0159.html
