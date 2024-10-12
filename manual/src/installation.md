# Installation

## Quick Installation (Unix only)

Enter the command below to quickly install Allay.

```console
curl https://allay-mc.github.io/getallay/getallay.sh -sSf | sh
```

## Manual Installation

Visit the [release page](https://github.com/allay-mc/allay/releases/) and
select the archive that matches your platform. Extract it and put the
executable to some path of your choise (usually something like `/usr/local/bin`
on Unix and `C:\Program Files` on Windows).

## Via Cargo

If your platform is not supported or you just prefer to build Allay yourself,
then you can do so by following the instructions below.

1. Make sure you have [Rust and Cargo][Rust] installed.
2. Install with `cargo install --git https://github.com/allay-mc/allay.git` or
   clone the repository with `git clone https://github.com/allay-mc/allay.git`
   and then run `cargo install --path allay`.

[Rust]: https://www.rust-lang.org/
