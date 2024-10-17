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
  - [`custom-manifest`](#the-custom-manifest-field)
  - [`custom-pack-icon`](#the-custom-pack-icon-field)
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

Specify the name of the project.

```toml
[project]
name = "Hello World"
```

It's also possible to localize the name:

```toml
[project.name]
de-de = "Hallo Welt"
en-us = "Hello World"
```


### The `description` field

Defines the description of the project. This has the same structure as [`name`](#the-name-field).


### The `version` field

The version of the project of the form `<major>.<minor>.<patch>`.

```toml
[project]
# ...
version = "1.3.0"
```


### The `authors` field

Specify the authors of the project. This is an array with strings of the form
`<full name> <email (optional)>`.

```toml
[project]
# ...
authors = ["John Doe <john.doe@example.org>", "Jane Doe"]
```


### The `license` field

Specify the license of the project. This should but is not required to match a [SPDX][] identifier.

```toml
[project]
# ...
license = "MIT"
```


### The `url` field

Specify the URL of the project.

```toml
[project]
# ...
url = "https://github.com/allay-mc/allay"
```


### The `min-engine-version` field

> This is the minimum version of the game that this pack was written for. This is a required field for
> resource and behavior packs. This helps the game identify whether any backwards compatibility is needed
> for your pack. You should always use the highest version currently available when creating packs.


## The `[localization]` section

### The `primary-language` field

### The `groups` field

## The `[env]` section

This section can be used to provide arbitrary arguments for plugins.

```toml
[env]
FOO = "1"
BAR = "Hello"
```

```admonish title="See Also"
[Plugins Chapter](./plugins.md)
```

## The `[build]` section

### The `extra-watch-dirs` field

Controls which directories should trigger a rebuild on changes when using the `watch` command.

```toml
[build]
extra-watch-dirs = ["plugins"]
```


## The `[[plugin]]` sections

### The `name` field { #the-plugin-name-field }

A plugin can optionally be granted a name which is useful as you can more easily identify what output
came from which plugin during the build process.

```toml
[[plugin]]
name = "Greeter"
run = "plugins/greet.rb"
with = "ruby"
```


### The `run` and `with` fields

The `run` and `with` fields are used to specify the program used to run the plugin.

```toml
[[plugin]]
run = "first-argument"
with = "program"
args = ["second-argument", "third-argument"]
```

Alternatively, you can use the following form:

```toml
[[plugin]]
run = "program"
args = ["first-argument", "second-argument", "third-argument"]
```

Both snippets effectively do the same and if you use the combination of `with` + `run` or just `run`
pretty much depends on your taste and the program you are invoking. Let's look at an example which
uses a Dart program as a plugin.

In the console you would write:

```console
dart run plugins/hello.dart
```

To use it as a plugin, we add it to the configuration file:

```toml
[[plugin]]
run = "run"
with = "dart"
args = ["plugins/hello.dart"]
```

But we can use the alternative which would look better in this case:

```toml
run = "dart"
args = ["run", "plugins/hello.dart"]
```


### The `args` and `options` fields

The `args` array passes each value to the program as an argument whereas the value used for `options`
is serialized into JSON and passed as a single argument to the plugin.


### The `when` field

```admonish title="See Also"
[Filters](./plugins.md#filters)
```


### The `threaded` field

```admonish
This field is unused but might be used in the future.
```


### The `panic` field


```admonish
This field is unused but might be used in the future.
```


## The `[BP]`, `[RP]`, `[SP]` and `[WT]` sections

These sections can be used to for pack-specific configurations.

The `[BP]` section also allows specifying the type of behavior pack by setting `type` to `data` or `script`.
The `[WT]` section also allows specifying `allow_random_seed` and `base_game_version`.


### The `custom-manifest` field

Whether to use the `manifest.json` file in the pack's directory instead of generating one.


### The `custom-pack-icon` field

Whether to use the `pack_icon.png` file in the pack's directory instead of generating one. This field does
not exist for world template configuration.


### The `name` and `description` field

By default `project.name` and `project.description` are applied for all packs. You can override those with
the `name` and `description` field. They both have the same structure as `project.name`/`project.description`.


### The `dependencies` field


[SPDX]: https://spdx.org/licenses/
