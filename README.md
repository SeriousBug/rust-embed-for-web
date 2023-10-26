# Rust Embed for Web <!-- omit in toc -->
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-2-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
![Crates.io](https://img.shields.io/crates/v/rust-embed-for-web)
[![Test coverage report](https://img.shields.io/codecov/c/github/SeriousBug/rust-embed-for-web)](https://codecov.io/gh/SeriousBug/rust-embed-for-web)
[![lint checks](https://img.shields.io/github/actions/workflow/status/SeriousBug/rust-embed-for-web/lint.yml?label=lints&branch=master)](https://github.com/SeriousBug/rust-embed-for-web/actions/workflows/lint.yml)
[![MIT license](https://img.shields.io/github/license/SeriousBug/rust-embed-for-web)](https://github.com/SeriousBug/rust-embed-for-web/blob/master/LICENSE.txt)

Rust Macro which embeds files into your executable. A fork of `rust-embed` with a focus on usage in web servers.

## Differences from `rust-embed`

This crate opts to make some choices that may increase the size of your
executable in exchange for better performance at runtime. In particular:

- Contents of the file may be stored multiple times, both compressed and
  uncompressed. This makes it possible to serve files from a server, depending
  on whether the client accepts compression or not, without having to compress
  or decompress anything at runtime.
  - If the compression makes little difference, for example a jpeg file won't
    compress much further if at all, then the compressed version is not included.
  - You can also disable this behavior by adding an attribute `#[gzip = false]` and `#[br = false]`
    When disabled, the compressed files won't be included for that embed.
- Some metadata that is useful for web headers like `ETag` and `Last-Modified`
  are computed ahead of time and embedded into the executable. This makes it
  possible to use these in a web server without any computation at runtime.
- File hashes are encoded with `base85` instead of hex, which is slightly more
  compact. When used as `ETag` values for files in requests, this slightly
  reduces the amount of data that has to be transferred.
- The file data (in release builds) is returned as a `&'static` reference. This
  makes is easy to use the file data in a server response without creating
  copies or reference counting.
  - In debug builds, the files are read dynamically when the embed is accessed.
    This means you don't have to recompile to see changes to embedded files when
    debugging.

## Installation

```toml
[dependencies]
rust-embed-for-web="11.1.3"
```

## Usage

To use this macro, add an empty struct, then add the derive. Then, you specify the folder to use.

```rust
use rust_embed_for_web::{EmbedableFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn main() {
  let index = Asset::get("index.html").unwrap().data();
  let contents = std::str::from_utf8(index.as_ref()).unwrap();
  println!("Index file: {}", contents);
}
```

The path for the `folder` is resolved relative to where `Cargo.toml` is.

### Disabling compression

You can add `#[gzip = false]` and/or `#[br = false]` attributes to your embed to
disable gzip and brotli compression for the files in that embed.
`rust-embed-for-web` will only include compressed files where the compression
actually makes files smaller so files that won't compress well like images or
archives already don't include their compressed versions. However you can

## Features

Both of the following features are enabled by default.

### `interpolate-folder-path`

Allow environment variables and `~`s to be used in the `folder` path. Example:

```rust
#[derive(RustEmbed)]
#[folder = "~/${PROJECT_NAME}/assets"]
struct Asset;
```

`~` will expand into your home folder, and `${PROJECT_NAME}` will expand into
the value of the `PROJECT_NAME` environment variable.

### `include-exclude`

You can filter which files are embedded by adding one or more `#[include = "*.txt"]` and `#[exclude = "*.jpg"]` attributes.
Matching is done on relative file paths --the paths you use for the `.get` call-- via [`globset`](https://docs.rs/globset/latest/globset/).
Excludes are processed first, then includes are applied to grant exceptions.

> ⚠️ This is different from the original `rust-embed` crate, so double check
> your include and exclude attributes to make sure the files are correct.

For example, if you wanted to exclude all `.svg` files except for one named
`logo.svg`, you could do:

```rust
#[derive(RustEmbed)]
#[exclude = "*.svg"]
#[include = "logo.svg"]
struct Assets;
```

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/hengfeiyang"><img src="https://avatars.githubusercontent.com/u/1628250?v=4?s=100" width="100px;" alt="Hengfei Yang"/><br /><sub><b>Hengfei Yang</b></sub></a><br /><a href="https://github.com/SeriousBug/rust-embed-for-web/commits?author=hengfeiyang" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/peroxid"><img src="https://avatars.githubusercontent.com/u/4387517?v=4?s=100" width="100px;" alt="peroxid"/><br /><sub><b>peroxid</b></sub></a><br /><a href="https://github.com/SeriousBug/rust-embed-for-web/issues?q=author%3Aperoxid" title="Bug reports">🐛</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
