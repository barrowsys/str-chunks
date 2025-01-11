# str-chunks

[![GitHub last commit](https://img.shields.io/github/last-commit/barrowsys/str-chunks)](https://github.com/barrowsys/str-chunks)
[![Crates.io](https://img.shields.io/crates/v/str-chunks)](https://crates.io/crates/str-chunks/)
[![Docs.rs](https://docs.rs/str-chunks/badge.svg)](https://docs.rs/str-chunks)

## About

implements char-wise chunked iteration of str

the methods `str_chunks`, `str_chunks_exact`, `str_rchunks`, and `str_rchunks_exact`
behave like the similarly named methods on slice,
but return string slices that are `chunk_size` chars long.
take note: these slices are not necessarily `chunk_size` *bytes* long. `chunk.len() != chunk_size`

import `ImplStrChunks` to get methods on `str`

## Usage

Add to your Cargo.toml:
```
[dependencies]
str-chunks = "0.1.0"
```
See [docs.rs](https://docs.rs/str-chunks) for examples.


or just copy [src/str_chunks.rs] into your own project. im not your mom.


