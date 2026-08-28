# NixOS Generation Diff

A Rust CLI tool for comparing NixOS system generations and presenting
package changes in a cleaner, categorised format.

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

Currently this is a CLI, however TUI is going to be developed soon!

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

```text
gen no: 120 build date: 2026-08-10 12:32:10
gen no: 121 build date: 2026-08-12 18:21:43
gen no: 122 build date: 2026-08-18 14:04:12

Choose a generation number:
120

Choose another generation number:
122

--- Package Update Summary ---
[Kernel Updates]
linux: 6.15.8 -> 6.15.9

[Updated Packages]
firefox: 140.0 -> 141.0, 82.4 MiB
gcc: 14.2.1 -> 14.2.2, 12.1 MiB

[Added Packages]
...

[Removed Packages]
...

[Rebuilt Packages]
...
```

## Requirements
- Rust
- NixOS
- nixos-rebuild
- nix

This program expects all your profiles to be located at `/nix/var/nix/profiles/`


## Why?
I realised that sometimes there was a huge wall of text with the package changes
after a rebuild and I wanted to make a clearer way of reading these packages,
so this project was born.

## What's next?
- [x] Better error handling
 - Currently the error handling is a bit... rudimentary.
- [x] Interactive TUI
- [ ] Potentially a better way of collecting generations and diffs without shell commands

