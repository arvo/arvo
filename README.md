# Arvo

![Build status](https://travis-ci.org/arvo/arvo.svg?branch=master)

The Arvo programming language.

## Requirements for building the compiler

### Rust

Install the latest version of Rust via rustup using the following command

```sh
curl https://sh.rustup.rs -sSf | sh`
```

### LLVM

Install LLVM version 3.9. For Ubuntu users see http://apt.llvm.org/. For macOS users use `brew link --force llvm@3.9` (after installing Homebrew https://brew.sh/).

## Building Arvo

Build the Arvo binary nad libraries using

```sh
cargo build
```

## Contributors

+ Benjamin Wang
+ Migara Liyanagamage
+ Uwe Zimmer
