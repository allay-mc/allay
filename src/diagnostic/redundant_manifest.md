# Redundant Manifest

Allay automatically generates a `manifest.json` file for each of your packs which means you don't need to
write them yourself. Instead you define metadata in the `allay.toml` configuration file and if neccessary
control UUIDs with the `uuid` subcommand.

Forcing the use of a custom manifest however can be achived by setting the `custom-manifest` field of
the appropiate `[BP]`, `[RP]`, `[SP]` or `[WT]` to `true`. So considering the following project structure:

```text
.
├── allay.toml
└── src/
    ├── BP/
    │   ├── entities/
    │   └── manifest.json
    ├── RP/
    │   └── textures/
    ├── SP/
    └── WT/
```

If you would like to use your own `manifest.json` for your behavior pack, your `allay.toml` configuration
file should look something like this:

```toml
[project]
# ...

[localization]
# ...

[BP]
custom-manifest = true
```
