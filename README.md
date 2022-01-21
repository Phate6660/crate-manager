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
pijul [1.0.0-beta]
ripgrep [13.0.0]
vivid [0.7.0]
xcp [0.9.0]
```

To install each crate, use `crate-manager install`.

## Usage

- `crate-manager export`: Export installed crates to `$HOME/exported_crates.txt`
- `crate-manager install`: Install crates from the above file
- `crate-manager install true`: Same as above but it also installs the specific version
- `crate-manager list`: Lists the packages tracked in `$HOME/exported_crates.txt`

## TODO

- Allow user to install missing external deps
  + Integrate with [`dep-organizer`](https://github.com/Phate6660/dep-organizer)?
- Fix crates with external dependencies being listed twice with `crate-manager list`
