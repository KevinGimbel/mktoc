# `mktoc`
> Markdown Table of Content generator

![](https://github.com/kevingimbel/mktoc/workflows/Clippy%20check/badge.svg)
![Coverage](assets/coverage/flat.svg)

# Table of Contents
<!-- BEGIN mktoc {"min_depth":2} -->
- [About](#about)
- [Installation](#installation)
  - [Cargo](#cargo)
  - [Binary](#binary)
- [Usage](#usage)
  - [Update Markdown file](#update-markdown-file)
    - [Inline config](#inline-config)
  - [Command line](#command-line)
  - [Environment configuration](#environment-configuration)
- [Auto-run with VSCode](#auto-run-with-vscode)
- [Performance](#performance)
- [Contributing](#contributing)
  - [Where to start?](#where-to-start)
  - [Tooling](#tooling)
  - [Install dev build](#install-dev-build)
- [License](#license)
<!-- END mktoc -->

## About
[⬆️ Back to Top](#table-of-contents)

`mktoc` parses markdown files and generates a Table Of Content linking all headlines up to heading level 6 deep, or as specified by command line arguments, environment variables, or inline JSON config (see [Usage](#usage)).

## Installation
[⬆️ Back to Top](#table-of-contents)

`mktoc` can be installed using Cargo, the Rust package manager.

### Cargo

**Installation**
```sh
$ cargo install mktoc
```

**Update**
```sh
$ cargo install --force mktoc
```

### Binary

Binaries are not available yet. If you know how releasing binaries with Rust can be implemented, please let me know!

## Usage

### Update Markdown file
[⬆️ Back to Top](#table-of-contents)

Add the following HTML comment into the Markdown file where the Table of Contents should be rendered.

```html
<!-- BEGIN mktoc -->
<!-- END mktoc -->
```

Everything between those comments will be replaced!

#### Inline config
[⬆️ Back to Top](#table-of-contents)

Starting with version 3.0.0 it's possible to set values in-line which is great when collaborating with teams who may have individual settings.

```html
<!-- BEGIN mktoc {"min_depth": 2, "max_depth": 4} -->

- [About](#about)
- [Installation](#installation)
  - [Cargo](#cargo)
  - [Binary](#binary)
- [Usage](#usage)
  - [Update Markdown file](#update-markdown-file)
    - [Inline config](#inline-config)
  - [Command line](#command-line)
  - [Environment configuration](#environment-configuration)
- [Auto-run with VSCode](#auto-run-with-vscode)
- [Performance](#performance)
- [Contributing](#contributing)
  - [Where to start?](#where-to-start)
  - [Tooling](#tooling)
  - [Install dev build](#install-dev-build)
- [License](#license)
<!-- END mktoc -- >
```

this is equal to running `mktoc -m 2 -M 4` or setting these environment variables `MKTOC_MIN_DEPTH=2` and `MKTOC_MAX_DEPTH=4`.

Inline config takes priority over environment or CLI arguments.

### Command line
[⬆️ Back to Top](#table-of-contents)

Specify `--stdout` or `-s` to output generated content to `stdout` instead of overwriting file. By default the specified file will be overwritten.

```sh
# mktoc [FLAGS] [OPTIONS] [file] 
$ mktoc -s README.md
$ mktoc -m 2 -M 4 README.md
$ mktoc
```
If no arguments are given the default or configured (via environment) values are
used. 

See `mktoc --help` for list of all arguments and flags.

```sh
mktoc

USAGE:
mktoc [FLAGS] [OPTIONS] [file]

FLAGS:
-h, --help       Prints help information
-s, --stdout     If set will output to stdout instead of replacing content in file
-V, --version    Prints version information

OPTIONS:
-M, --max-depth <max-depth>    Maximum heading level [env: MKTOC_MAX_DEPTH=]  [default: 6]
-m, --min-depth <min-depth>    Minimum heading level [env: MKTOC_MIN_DEPTH=2]  [default: 1]

ARGS:
<file>     [default: README.md]
```

### Environment configuration
[⬆️ Back to Top](#table-of-contents)

mktoc can be configured with environment variables, namely:

- `MKTOC_MIN_DEPTH` equal to `-m`
- `MKTOC_MAX_DEPTH` equal to `-M`

Place these variables in a shell environment file such as `~/.bashrc` or
`~/.zshrc`, then just run mktoc without `-m` and `-M`

```sh
# The following in ~/.bashrc|~/.zshrc configures mktoc to render headings from level 2 to
# level 4
# 
# MKTOC_MIN_DEPTH=2
# MKTOC_MAX_DEPTH=4

$ mktoc README.md
```

## Auto-run with VSCode
[⬆️ Back to Top](#table-of-contents)

For VSCode the [Run on save](https://github.com/emeraldwalk/vscode-runonsave) extension can be used to trigger mktoc.

Install the extension and then add the following config to workspace or user `settings.json`.

```json
"emeraldwalk.runonsave": {
    "commands": [
        {
            "match": "\\.md$",
            "cmd": "mktoc ${file}"
        }
    ]
}
```

This will run the command for every markdown file on safe. If there is no mktoc comment in the Markdown file nothing happens.

## Performance
[⬆️ Back to Top](#table-of-contents)

`mktoc` is fast but can probably be even faster! Pull Requests and bug reports are appreciated!

## Contributing
[⬆️ Back to Top](#table-of-contents)

We love and welcome every form of contribution.

### Where to start?

Here are some good places to start:

* Issues with label [Good first issue](https://github.com/kevingimbel/mktoc/labels/good%20first%20issue)
* Issues with label [Documentation](https://github.com/kevingimbel/mktoc/labels/documentation)
* Providing example implementations or usage demos

### Tooling

- [mktoc](https://github.com/KevinGimbel/mktoc) is used for table of content generation in the README.md (neat!)
- [criterion](https://github.com/bheisler/criterion.rs) for benchmarking

### Install dev build

Sometimes it's nice to install a specific version of mktoc, this can be done with the following command:

```sh
# install specific commit
cargo install --git https://github.com/KevinGimbel/mktoc --force --rev $COMMIT_ID
# install branch
cargo install --git https://github.com/KevinGimbel/mktoc --force --branch $BRANCH_NAME
```


## License
[⬆️ Back to Top](#table-of-contents)

MIT, see LICENSE file.
