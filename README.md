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


> [!WARNING]
> This project is in a work-in-progress status. Many features as well as
> links may not work yet. Consider waiting for a stable release if you
> want to use this program.


- 📖 [Read the Documentation](https://allay-mc.github.io/docs/)
- 📦 [Crate](https://crates.io/crates/allay)

<p align="center">
<a href="https://allay-mc.github.io/docs/">
  <img
    alt="Documentation"
    src="https://img.shields.io/badge/Documentation-Read-brown?style=for-the-badge"
  />
</a>

<a href="https://crates.io/crates/allay">
  <img
    alt="Crates.io"
    src="https://img.shields.io/crates/v/allay?style=for-the-badge&label=Version&color=lightblue"
  />
</a>
<a href="https://github.com/allay-mc/allay/commit/master">
  <img
    alt="GitHub commit activity (branch)"
    src="https://img.shields.io/github/commit-activity/m/allay-mc/allay?style=for-the-badge"
  />
</a>

<a href="https://github.com/allay-mc/allay/releases">
  <img
    alt="GitHub (Pre-)Release Date"
    src="https://img.shields.io/github/release-date-pre/allay-mc/allay?style=for-the-badge"
  />
</a>

<a href="https://github.com/allay-mc/allay/graphs/contributors">
  <img
    alt="GitHub contributors"
    src="https://img.shields.io/github/contributors/allay-mc/allay?style=for-the-badge"
  />
</a>
</p>


## Status

1. [ ] Make it work[^1]
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

# intialize new project
mkdir my-project
cd $_
allay init

# populate pack(s)
echo '{"foo": "bar"}' > src/BP/hello.txt

# build pack(s)
allay build
```


## Versioning

Allay adheres to [Semantic Versioning](https://semver.org/). The changelog is
documented [here](./CHANGELOG.md).


[^1]: *Most* features work.
