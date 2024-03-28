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
echo '{"foo": "bar"}' > src/BP/hello.json

# build add-on
allay build
```


## Additional Features

Feature             | Flag             | Description                     | Enabled by default[^1]
--------------------|------------------|---------------------------------|-----------------------
**share command**   | `share`          | Shares add-ons over HTTP        | yes
**export command**  | `export`         | Exports add-ons to Minecraft    | yes
**schema command**  | `config-schema`  | JSON schema for config file     | no
**watch command**   | `watch`          | Rebuild add-ons on file changes | yes
**manual command**  | `manual`         | Opens the manual                | yes

[^1]: This only applies for installing/building without specifying any flags.


## Versioning

Allay adheres to [Semantic Versioning](https://semver.org/). The changelog is
documented [here](./CHANGELOG.md).

