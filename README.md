# Knitcheck

A library for type checking knitting patterns using the Rust type system and const generics. Requires Rust nightly (version 1.56 or greater).

## Setting up to build/run the project

- [Install rust](https://rustup.rs) via rustup
- Install the `nightly-2021-11-08` toolchain (`rustup install nightly-2021-11-08`)
- Clone the repository
- Run `rustup override set nightly-2021-11-08` in the repo root. This will make the nightly toolchain you just installed the default for the current project, so running any build/test commands should just work and use the appropriate compiler.

Other versions of Rust nightly should work, but GitHub actions should guarantee that the version specified above works.

## Running tests

To run all the tests, use the command:

```bash
cargo test
```
