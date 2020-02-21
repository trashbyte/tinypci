## tinypci &ensp; [![Crates.io](https://img.shields.io/badge/tinypci%20@%200.1.0-crates.io-brightgreen)](https://crates.io/crates/tinypci) [![Docs.rs](https://docs.rs/tinypci/badge.svg)](https://docs.rs/tinypci/)

A simple, safe, rusty toolset for reading and writing PCI configuration memory.
I'm using it as part of my [untitled os project](https://github.com/trashbyte/os).
Currently incomplete and unstable, but feel free to use this as an example or a
reference for your own projects.

This crate is designed to have minimal dependencies. It uses stdlib by default,
but with `--no-default-features` (or `default-features = false` in `Cargo.toml`)
it'll work with no_std too. Note that currently it does require `alloc` for a
no_std build.

Aside from `alloc` in `no_std` mode, this crate has no other dependencies by default.
Depends on `serde` when the `serde` feature is enabled.
This crate must be built with nightly as it uses `#![feature(asm)]`.
