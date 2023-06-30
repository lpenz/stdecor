[![CI](https://github.com/lpenz/stdecor/actions/workflows/ci.yml/badge.svg)](https://github.com/lpenz/stdecor/actions/workflows/ci.yml)
[![coveralls](https://coveralls.io/repos/github/lpenz/stdecor/badge.svg?branch=main)](https://coveralls.io/github/lpenz/stdecor?branch=main)
[![dependency status](https://deps.rs/repo/github/lpenz/stdecor/status.svg)](https://deps.rs/repo/github/lpenz/stdecor)
[![crates.io](https://img.shields.io/crates/v/stdecor)](https://crates.io/crates/stdecor)
[![packagecloud](https://img.shields.io/badge/deb-packagecloud.io-844fec.svg)](https://packagecloud.io/app/lpenz/debian/search?q=stdecor)

# stdecor

**stdecor** is a program that decorates streams of text with a custom
prefix, time, etc. It's able to call a subprocess to execute or run as
part of a pipe. When calling a program, it can decorate stdout and
stderr differently.

stdecor is specially useful when running multiple jobs in the same
shell.


## Installation

If you're a **Rust programmer**, stdecor can be installed with `cargo`:

```
$ cargo install stdecor
```

If you're a **Debian** user, stdecor is available in
[packagecloud](https://packagecloud.io/app/lpenz/debian/search?q=stdecor). Follow
these
[instruction](https://packagecloud.io/lpenz/debian/install#manual) to
use the package repository.

This repository also provides a **flake.nix** file for nix users.
