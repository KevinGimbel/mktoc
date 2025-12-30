# Changelog

## v5.1.0 - 2025-12-30

This release fixes the CI release job and updates the `Cargo.lock` file. See the previous release for fixed and changed things.

## v5.1.0 - 2025-12-30

### Added

- A pre-commit hook has been added for easy integration of mktoc into projects. It can be used like so:

```yaml
repos:
  - repo: https://github.com/KevinGimbel/mktoc
    rev: 5.1.0
    hooks:
      - id: mktoc
```

### Fixed 

- Duplicate headings are rendered correctly (See [#26](https://github.com/KevinGimbel/mktoc/issues/26))
- Exclude list for Cargo has been updated to remove unneeded files from the Cargo package

## v5.0.0 - 2025-12-28

New release who this?
This release mainly updates Rust as well as all dependencies, and introduces a new development setup with Nix.

`mktoc` has been added to `nixpkgs` ([Link](https://search.nixos.org/packages?channel=25.11&show=mktoc&query=mktoc)) and `hermit` ([Link](https://github.com/cashapp/hermit-packages/blob/master/mktoc.hcl)) for easier installation.

### Changed 
- Upgrade to Rust Edition 2024 from 2021
- Upgrade to Rust version 1.91.1 from 1.64.0
- Upgrade criterion from 0.4 to 0.8
- Change regex version to 1 instead of 1.31, to automatically receive updates

### Dev Tools
- [nix](https://nix.dev/) flake has been added for reproducible development environments
- [just](https://github.com/casey/just) has been added as task runner, replacing `make`

## v4.0.0

- use Rust Edition 2021 (upgrade from 2018)

## v3.3.0

- refactor `make_toc` function for easier third-party integration [#13](https://github.com/KevinGimbel/mktoc/issues/13)

## v3.2.0

- add `-w` and `--wrap-in-details` option to cli [e75e288](https://github.com/KevinGimbel/mktoc/commit/e75e288)
- small readme updated [e75e288](https://github.com/KevinGimbel/mktoc/commit/e75e288)

## v3.1.0

- Replace structopts with clap [#10](https://github.com/KevinGimbel/mktoc/issues/10) by [@oylenshpeegul](https://github.com/oylenshpeegul)
- wrap ToC in details block [#8](https://github.com/KevinGimbel/mktoc/issues/8) by [@KevinGimbel](https://github.com/KevinGimbel)
- Links in Headlines produce wrong output [#12](https://github.com/KevinGimbel/mktoc/issues/12) by [@KevinGimbel](https://github.com/KevinGimbel)

### Misc

- add more tests

## v3.0.0

- Implement JSON settings
- Add build for GitHub Binary releases
