# Contributing to mangadex-api

<!-- Adapted from https://github.com/rust-lang/rust-clippy/blob/master/CONTRIBUTING.md -->

**First**: If you're unsure or afraid of _anything_, just ask or submit the issue or merge request
anyway. You won't be yelled at for giving it your best effort. The worst that can happen is that
you'll be politely asked to change something. We appreciate any sort of contributions, and don't
want a wall of rules to get in the way of that.

mangadex-api welcomes contributions from everyone. There are many ways to contribute to
mangadex-api, and the following document explains how you can contribute and how to get started. If
you have any questions about contributing or need help with anything, feel free to ask questions
on issues.

- [Getting started][section-getting-started]
    - [Basics][section-basics]
        - [Get the code][section-get-the-code]
        - [Building and Testing][section-building-and-testing]
        - [Merge Request][section-merge-request]
    - [High-level approach][section-high-level-approach]
- [Writing code][section-writing-code]
- [Contributions][section-contributions]

## Getting started

### Basics

If you've gone through the basic setup, skip ahead to the
[High-level approach section][section-high-level-approach].

#### Get the Code

First, make sure you have checked out the latest version of mangadex-api. If this is
your first time working on mangadex-api, create a fork of the repository and clone it
afterwards with the following command:

```bash
git clone git@gitlab.com:<your-username>/mangadex-api
```

If you've already cloned mangadex-api in the past, update it to the latest version:

```bash
# Upstream has to be the remote of the "gondolyr/mangadex-api" repo.
git fetch upstream

# Make sure that you are on the "main" branch.
git checkout main

# Rebase your "main" branch on the upstream "main".
git rebase upstream/main

# Push to the "main" branch of your fork.
git push
```

#### Building and Testing

You can build and test mangadex-api like every other Rust project:

```bash
cargo build
cargo test
```

#### Merge Request

All merge requests should be filed against the `main` branch, except in very particular scenarios.
Unless you know for sure you should target another branch, `main` will be the right choice (it's
also the default).

We follow a no merge-commit policy, meaning, when you encounter merge conflicts you are expected
to always rebase instead of merging. E.g. always use rebase when bringing the latest changes
from the `main` branch to your feature branch. Also, please make sure fixup commits are squashed
into other related commits with meaningful commit messages.

### High-level approach

1. Find something to fix/improve.
2. Change code (likely some file in `src/`).
3. Follow the instructions in the [Basics section][section-basics] to get set up.
4. Run `cargo test` in the root directory and adjust code until it passes.
5. Start committing your changes. Follow the [conventional commit specification][conventional-commits] while doing so.
6. Open an MR (also can be done after step 2. if you run into problems so that others can help).

## Writing code

We follow the default settings of the [rust-fmt][tool-rust-fmt] and [rust-clippy][tool-rust-clippy]
tools. Be sure to run the following before creating a merge request:

```bash
# Format Rust code.
cargo fmt

# Run the linter to catch common mistakes and unidiomatic code.
cargo clippy
```



## Contributions

Contributions to mangadex-api should be made in the form of GitLab merge requests. Each merge
request will be reviewed by a core contributor (someone with permission to land patches) and
either landed in the main tree or given feedback for changes that would be required.

All code in this repository is under the [Apache-2.0] or the [MIT] license.

[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[clippy-contributing]: https://github.com/rust-lang/rust-clippy/blob/master/CONTRIBUTING.md
[conventional-commits]: https://www.conventionalcommits.org/
[MIT]: https://opensource.org/licenses/MIT
[section-basics]: #basics
[section-building-and-testing]: #building-and-testing
[section-contributions]: #contributions
[section-get-the-code]: #get-the-code
[section-getting-started]: #getting-started
[section-high-level-approach]: #high-level-approach
[section-merge-request]: #merge-request
[section-writing-code]: #writing-code
[tool-rust-clippy]: https://github.com/rust-lang/rust-clippy
[tool-rust-fmt]: https://github.com/rust-lang/rustfmt
