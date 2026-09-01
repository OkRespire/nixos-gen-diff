# NixOS Generation Diff

A Rust TUI tool for comparing NixOS system generations and presenting
package changes in a cleaner, categorised format built on ratatui.

## Overview

NixOS stores previous system configurations as generations. This project
allows you to select two generations and compare the packages that changed
between them.

It uses `nix store diff-closures` to obtain the differences between the
selected generations and then categorises those changes into:

- Kernel updates
- Updated packages
- Removed packages
- Added packages
- Rebuilt packages
- Other changes


## Features

- List available NixOS generations
- Select two generations to compare
- Compare generation closures using `nix store diff-closures`
- Parse package changes from Nix output
- Detect package additions, removals, updates and rebuilds
- Identify kernel-related changes separately
- Display package version and size changes
- Group changes into categories

## Example
![Demo of nixos-gen-diff](./assets/demo.gif)

## Requirements
- Rust
- NixOS
- nixos-rebuild
- nix

This program expects all your profiles to be located at `/nix/var/nix/profiles/`

## Installation
Not yet published to a registry, so for now, run it directly from the flake:
```bash
$ nix run github:OkRespire/nixos-gen-diff
```
If you want to hack on the code instead, clone the repo and use the dev shell
(via `direnv allow`, or manually with `nix develop`) to get `cargo` and the
rest of the toolchain available.


## Why?
I realised that sometimes there was a huge wall of text with the package changes
after a rebuild and I wanted to make a clearer way of reading these packages,
so this project was born.

## What's next?
- [x] Better error handling
 - Currently the error handling is a bit... rudimentary.
- [x] Interactive TUI
- [ ] Potentially a better way of collecting generations and diffs without shell commands

