## pci

A simple, safe, rusty toolset for reading and writing PCI configuration memory.
I'm using it as part of my [untitled os project](https://github.com/trashbyte/os).

This crate is designed to have minimal dependencies. It uses stdlib by default,
but with `--no-default-features` (or `default-features = false` in `Cargo.toml`)
it'll work with no_std too. Note that currently it does require `alloc` for a
no_std build.

Aside from `alloc` in `no_std` mode, this crate has no other dependencies. It must
be built with nightly as it uses `#![feature(asm)]`.
