# Changelog

## v5.0.0-rc3

- Testing new build scripts

## v5.0.0

## New

- mktoc has been added to `nixpkgs` and `hermit` for easier installation

## Changes 

- Upgrade to Rust Edition 2024 from 2021
- add [nix](https://nix.dev/) flake for reproducible development environments
- update to Rust to 1.91.1


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
