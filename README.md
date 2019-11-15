# `mktoc`
> 

<!-- BEGIN mktoc -->
- [`mktoc`](#`mktoc`)
- [About](#About)
- [Installation](#Installation)
  - [Cargo](#Cargo)
  - [Binary](#Binary)
- [Usage](#Usage)
- [mktoc [--write] <FILE>](#mktoc-[--write]-<FILE>)
- [Performance](#Performance)
<!-- END mktoc -->

## About

`mktoc` parses markdown files and generates a Table Of Content linking all headlines up to heading level 6 deep.

## Installation

`mktoc` can be installed using Cargo, the Rust package manager, or by downloading a binary from GitHub.

### Cargo

```sh
$ cargo install mktoc
```

### Binary

Download latest release from [https://github.com/kevingimbel/mktoc/releases](https://github.com/kevingimbel/mktoc/releases) and place it somewhere in your `PATH`, e.g. `/usr/local/bin`.

## Usage

Specify `--write` to overwrite the given file, otherwise the modified content is written to stdout.

```
# mktoc [--write] <FILE>
$ mktoc --write README.md
```

See `mktoc --help` for list of all arguments and flags.

 
## Performance

`mktoc` is blazingly fast. Large files such as the README examples in `tests/files/` render in 0.009s (9ms) on average.
