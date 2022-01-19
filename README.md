# crate-manager

To get started, export your crates with `crate-manager export` (there will be no output).<br>
The list of crates will be exported to: `$HOME/exported_crates.txt`.<br>
Each line consists of at least `name=version`, and if the crate has any<br>
external deps, they will be listed in the format of `name=version=dep,dep`.

To list the crates you've exported, use `crate-manager list`:
```
$ crate-manager list
bat [0.19.0]
bottom [0.6.6]
cargo-edit [0.8.0]
cargo-update [8.1.2]
cavif [1.3.3]
coreutils [0.0.8]
deno [1.17.3]
dep-organizer [0.1.0]
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
pkg-gentoo [0.1.0]
prj [0.1.0]
ripgrep [13.0.0]
rsfetch [0.1.0]
vivid [0.7.0]
xcp [0.9.0]
```

To install each crate, use `crate-manager install`.

## TODO

- Allow user to install specific crate version
- Allow user to install missing external deps
  + Integrate with [`dep-organizer`](https://github.com/Phate6660/dep-organizer)?
