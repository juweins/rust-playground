# About this Tool

This tool should be an alternative implementation of grep. The idea for this tool comes from the official rust-lang tutorials section. Using this tool you can search for a pattern in a file or in a directory. The tool is written in Rust and highlights the found pattern in the output.

This CLI Tool is WIP to improve my Rust. 🦀:)


### Installation

The binary name for ripgrep is `grrs`. I did not change the name of the binary to avoid confusion with the origin of the initial idea. The latest, released binary is located in the `target/release` directory. You can copy the binary to your `PATH` or use the binary directly.

So far, the tool has only been tested on Linux (via WSL2 for Win11). It has not been published to crates.io yet. I may do this when the tool is more mature.

## Features

### Feature Overview

With `grrs` you can:
- Search for a pattern in a file
- Highlight the found pattern in the output
- Save the output to a file or print to stdout


### Feature Discussion
My implementation of `grrs` differs from alternatives like `ripgrep` in the following ways:

- As of 01/2023 `grrs` there is no support for regex expressions. The tool only supports simple patterns. This is a deliberate decision to keep the tool simple and to avoid the complexity of regex expressions in a first version. I may add regex support in the future.

- `grrs` does not support the `-v` flag (like in `ripgrep`) to invert the search. I may add this feature in the future.

- `grrs` does not support the `-i` flag (like in `ripgrep`) to ignore case. So far, the tool is case insensitive.

- `grrs` has only been developed and tested to search for patterns inside files. In a future release I may add support for searching inside directories as it is a handy feature.

### Feature Roadmap
This section lists the features I plan to add to the tool. I will add features as I need them or as I find the time to implement them. I will also add features as I learn more about Rust. Feel free to come up with feature requests or to contribute to the tool! Development together is fun! :) 👍🏽

#### CLI Tool Roadmap
- [x] Search for a pattern in a file
- [ ] Search for a pattern in a directory
- [x] Highlight the found pattern in the output

- [ ] Add support for regex expressions
- [ ] Add support for invert search
- [ ] Add support for case-sensitive search

#### Cross-Platform Roadmap
- [ ] Add support for multiple platforms
    - [ ] Add support for Windows
    - [ ] Add support for MacOS
    - [x] Add support for Linux
- [ ] Add support for multiple shells
    - [ ] Add support for PowerShell
    - [x] Add support for Bash
    - [x] Add support for ZSH

#### Repo Roadmap
- [ ] Add a proper Documentation
- [ ] Add a Tutorial to the README.md
- [ ] CI/CD: Add a GitHub Action to automatically build the binary for different platforms
- [x] CI/CD: Add a GitHub Action to automatically build the binary
- [ ] CI/CD: Add a GitHub Action to automatically publish the binary to crates.io


## Installation

So far, the tool has not been published to crates.io. You can install the tool by cloning the repo and building the binary yourself. The latest, released binary is located in the `target/release` directory. You can copy the binary to your `PATH` or use the binary directly.

If you want to try the tool without having rust installed or modifying your `PATH`, you can use the binary directly from this repo. The binary is located in the `target/release` directory.

As an alternative, you can find this tool packed inside an alpine image in my [Docker Hub](https://hub.docker.com/repositories/juweins)! :)


### Building

`grrs` is a rust implementation similar to well-known tools like grep, ripgrep or ag. In order to build the tool you need to have a rust compiler installed. You can install the rust compiler via [rustup](https://rustup.rs/).

This tool compiles with Rust 1.56.0 (stable) or newer. In general, the tool tracks the latest stable release of the Rust compiler. I personally develop on nightly, but I try to keep the tool compatible with stable.

To build `grrs` on your own, clone the repo and run `cargo build --release`. The binary will be located in the `target/release` directory. You can also copy the binary from this repo to your `PATH` or use the binary directly:

```
$ git clone https://github.com/juweins/rust-playground.git
$ cd rust-playground/grrs
$ cargo build --release
$ ./target/release/grrs --version
```

### Testing

This tool is relatively well covered and comes with both, unit and integration tests. To run the unit tests, use:

```
$ cargo test
```

To run the full test suite of `grrs`, use:

```
$ cargo test --all
```

from inside the `grrs` directory.