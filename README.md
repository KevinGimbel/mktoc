# `mktoc`
> Blazingly fast Markdown Table of Content generator

![](https://github.com/kevingimbel/mktoc/workflows/Clippy%20check/badge.svg)

<!-- BEGIN mktoc -->
- [`mktoc`](#`mktoc`)
- [About](#about)
- [Installation](#installation)
  - [Cargo](#cargo)
  - [Binary](#binary)
- [Usage](#usage)
- [mktoc [--write] <FILE>](#mktoc-[--write]-<file>)
- [Performance](#performance)
<!-- END mktoc -->

## About

`mktoc` parses markdown files and generates a Table Of Content linking all headlines up to heading level 6 deep, or as specified by command line arguments. A start depth and maximum depth can be specified.

## Installation

`mktoc` can be installed using Cargo, the Rust package manager.

### Cargo

```sh
$ cargo install mktoc
```

### Binary

Binaries are actually not available yet. If you know how releasing binaries with Rust can be implemented, please let me know!

## Usage

Specify `--write` to overwrite the given file, otherwise the modified content is written to stdout.

```
# mktoc [--write] [--max-depth|-M] [--min-depth|-m] <FILE>
$ mktoc --write README.md
$ mktoc --write -m 2 -M 4 README.md
```

See `mktoc --help` for list of all arguments and flags.

```
mktoc 1.1.0

USAGE:
    mktoc [FLAGS] [OPTIONS] <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -w, --write      

OPTIONS:
    -M, --max-depth <max-depth>     [default: 6]
    -m, --min-depth <min-depth>     [default: 1]

ARGS:
    <file> 
```
 
## Performance

`mktoc` is fast but can probably be even faster! Pull Requests and bug reports are appreciated!

## License

MIT, see LICENSE file.