# crate-manager

To get started, first copy `.cm_rules`\* to your `$HOME`.<br>
Then export your crates with `crate-manager export` (there will be no output).<br>
The list of crates will be exported to: `$HOME/exported_crates.txt`.<br>
Each line consists of at least `name=version`, and if the crate has any<br>
external deps, they will be listed in the format of `name=version=dep,dep`.

\* This file contains a list of lines formatted as `name=dep,dep`.<br>
It is used for specifying the external dependencies of crates.

To list the crates you've exported, use `crate-manager list`:
```
$ crate-manager list
bat [0.19.0]
bottom [0.6.6]
cargo-edit [0.8.0]
cargo-update [8.1.2]
cavif [1.3.3]
deno [1.17.3]
exa [0.10.1]
fd-find [8.3.1]
flamegraph [0.5.1]
git-delta [0.11.3]
hyperfine [1.12.0]
pastel [0.8.1]
pijul [1.0.0-beta], which also depends on:
- openssl
- libsodium
- libzstd
- xxhash
- pkg-config
ripgrep [13.0.0]
vivid [0.7.0]
xcp [0.9.0]
```

To install each crate, use `crate-manager install`.

## Usage

- `crate-manager export`: Export installed crates to `$HOME/exported_crates.txt`
- `crate-manager install`: Install crates from the above file
- `crate-manager install --exclude crate,crate2`: Exclude certain crates from being installed
- `crate-manager install true`: Install specific versions of crates\*
- `crate-manager list`: Lists the packages tracked in `$HOME/exported_crates.txt`

\* NOTE: If using both exclude and installing specific versions, you must pass the exclusions first.<br>
I currently hand-parse the given command line arguments and it relies on certain positioning.

## TODO

- Allow user to install missing external deps
  + Integrate with [`dep-organizer`](https://github.com/Phate6660/dep-organizer)?
- Better command line argument parsingm, I will either:
  + Write a library from scratch to handle this
  + Decide on a good crate besides `clap` or `structopt` to handle the parsing
    * (Nothing against them, they're just... really bloated. Especially for the basic parsing that I want.)
- Somehow I done bugged it and `crate-manager` is now double exporting crates
  + I assume it somehow started with the latest commit adding crate exclusion support
