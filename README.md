<p align="center">
  <img
    src="./allay.gif"
    width="25%"
    align="center"
    alt="Animated Allay"
  />
  <h1 align="center">Allay</h1>
  <p align="center">
    Your Personal Creator Assistant
  </p>
</p>


> **Warning**
>
> This project is in a work-in-progress status. Many features as well as
> links may not work yet. Consider waiting for a stable release if you
> want to use this program.


- 📖 [Read the Documentation](https://allay-mc.github.io/docs/)
- 📦 [Crate](https://crates.io/crates/allay)


## Status

1. [ ] Make it work
2. [ ] Make it right
3. [ ] Make it fast


## Installation

### Quick Install

1. Goto the [Releases](https://github.com/allay-mc/allay/releases) section.
2. Download the file that you need for your operating system from the latest
   stable release.
3. Unzip the file.
4. - **Windows**: Add the path to the file to your `PATH` environment variable.
   - **Linux/macOS**: Move the file to `/usr/local/bin/`.


### Via Cargo

```console
cargo install allay
```

... or with [binstall](https://github.com/cargo-bins/cargo-binstall) ...

```console
cargo binstall allay
```


## Quickstart

```bash
# note: This is a bash script
mkdir my-project
cd $_
allay init
```


## Versioning

Allay adheres to [Semantic Versioning](https://semver.org/). The changelog is
documented [here](https://allay-mc.github.io/docs/changelog.html).


## TODO

- tutorial for first time add-on dev and migrating from classic variant
- remove prev build if it has same version as current one
- each build should have an ID
- warn if pack icon / world icon is missing
- option to insert arbitary data in manifest to support backwards compability
- make `update` command a feature (enabled by default)?
- inform (panic) that a `manifest.json` is not necessary
- `clean` command to clean build & cache, `fullclean` to also reset uuids
- ensure uuid is not empty when using one
- `-q, --quiet` option
- remove DRY code
- validate config
- consider https://learn.microsoft.com/en-us/minecraft/creator/documents/basegameversioning#setup
- create `Environment` in `main.rs` and pass it to cli which passes it to other mods
- consider https://learn.microsoft.com/en-us/minecraft/creator/documents/packagingaworldtemplate#template-world_behaviorresource_packsjson
- add CONTRIBUTING.md
- assert language syntax in config
- maybe use enum for language
- option to save (official (?)) script in `~/.allay/global_scripts/` to save mem (?)
- support custom language groups
- improve error messages
- fully seperate cli and impl
- implement logging with file and `-v` flag
- check if all JSON files are valid JSON (with comment) files
- update command
- `watch` command
- wt should automatically contain bp and rp
- script that support markup such as `<red>foo</red>` and support custom tags
- rethink panics, expects, etc
- MSI for Windows
- script for templating such that something like `include` works
- script or built.in support for https://learn.microsoft.com/en-us/minecraft/creator/documents/packagingaskinpack#skinsjson
