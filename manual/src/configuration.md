# Configuration

The configuration file `allay.toml` contains metadata about your Allay project next to some additional
configuration options. Most of the values are used to generate the `manifest.json` manifest file for each
add-on whereas some of them control the build process.

- [`$schema`](#the-schema-field)
- [`debug`](#the-debug-field)
- [`[project]`](#the-project-section)
  - [`name`](#the-name-field)
  - [`description`](#the-description-field)
  - [`version`](#the-version-field)
  - [`authors`](#the-authors-field)
  - [`license`](#the-license-field)
  - [`url`](#the-url-field)
  - [`min-engine-version`](#the-min-engine-version-field)
- [`[localization]`](#the-localization-section)
  - [`primary-language`](#the-primary-language-field)
  - [`groups`](#the-groups-field)
- [`[env]`](#the-env-section)
- [`[build]`](#the-build-section)
  - [`extra-watch-dirs`](#the-extra-watch-dirs-field)
- [`[[plugin]]`](#the-plugin-sections)
  - [`name`](#the-plugin-name-field)
  - [`run`](#the-run-and-with-fields)
  - [`with`](#the-run-and-with-fields)
  - [`args` / `options`](#the-args-and-options-fields)
  - [`when`](#the-when-field)
  - [`threaded`](#the-threaded-field)
  - [`panic`](#the-panic-field)
- [`[BP]`, `[RP]`, `[SP]` and `[WT]`](#the-bp-rp-sp-and-wt-sections)
  - [`custom_manifest`](#the-custommanifest-field)
  - [`custom_pack_icon`](#the-custompackicon-field)
  - [`name` and `description`](#the-name-and-description-field)
  - [`dependencies`](#the-dependencies-field)

```toml
{{#include ../../src/scaffolding/allay.toml}}
```

## The `$schema` field

Can be used for editor completions.

```toml
"$schema" = "https://allay.github.io/config-schema.json"

[project]
# ...

[localization]
# ...
```

## The `debug` field

Defines whether builds run in `debug` mode. For example, the `manifest.json` file in debug mode is formatted
with indention and compressed in release mode. Plugins may access the `ALLAY_DEBUG` variable for variable
behavior.


## The `[project]` section

### The `name` field

### The `description` field

Defines the description of the project. This has the same structure as [`name`](#the-name-field).


### The `version` field

### The `authors` field

### The `license` field

### The `url` field

### The `min-engine-version` field

## The `[localization]` section

### The `primary-language` field

### The `groups` field

## The `[env]` section

## The `[build]` section

### The `extra-watch-dirs` field

## The `[[plugin]]` sections

### The `name` field { #the-plugin-name-field }

### The `run` and `with` fields

### The `args` and `options` fields

### The `when` field

### The `threaded` field

### The `panic` field

## The `[BP]`, `[RP]`, `[SP]` and `[WT]` sections

These sections can be used to for pack-specific configurations.

The `[BP]` section also allows specifying the type of behavior pack by setting `type` to `data` or `script`.


### The `custom_manifest` field

### The `custom_pack_icon` field

### The `name` and `description` field

### The `dependencies` field
