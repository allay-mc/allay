<p align="center">
  <img
    src="https://raw.githubusercontent.com/allay-mc/assets/main/logo-1080x.png"
    width="25%"
    align="center"
    alt="Animated Allay"
  />
  <h1 align="center">Allay</h1>
  <p align="center">
    Your Personal Creator Assistant
  </p>
</p>

- 📖 [Read the Manual](https://allay-mc.github.io/allay/)
- 📦 [Crate](https://crates.io/crates/allay)


## Quickstart

```bash
# intialize new project
mkdir my-project
cd my-project
allay init

# populate add-on
$EDITOR src/BP/hello.json

# build add-on
allay build
```


## Additional Features

Feature               | Flag                | Description                     | Enabled by default
----------------------|---------------------|---------------------------------|-------------------
**share command**     | `share`             | Shares add-ons over HTTP        | yes
**export command**    | `export`            | Exports add-ons to Minecraft    | yes
**git**               | `git`               | Handles `git`                   | no
**shell completions** | `shell-completions` | Generates shell completions     | yes
**schema command**    | `config-schema`     | JSON schema for config file     | no
**watch command**     | `watch`             | Rebuild add-ons on file changes | yes
**manual command**    | `manual`            | Opens the manual                | yes

To enable features that are not active by default, use `-F <feature name>` when installing/building
Allay (for example: `cargo install -F config-schema allay`). To disable all default features, use the
`--no-default-features` (for example: `cargo install --no-default-features -F git allay`).


## Versioning

Allay adheres to [Semantic Versioning](https://semver.org/). The changelog is
documented [here](./CHANGELOG.md).

