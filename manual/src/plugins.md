# Plugins

## Table Of Contents

<!-- toc -->

---

Plugins are programs that extend Allay's functionality. They usually take the **source files** and transforms them in some way.
There are a few plugins created by the Allay developers themselves that you can use right away! All of them are written in the
[Ruby programming language][Ruby] so make sure to install it before utilizing the plugins. Within your project run the following
command:

```console,icon=%gnubash,fp=Console
mkdir plugins
git submodule add https://github.com/allay-mc/plugins.git plugins/thirdparty
```

## Filters

Filters can be used to prevent plugins from running under certain conditions. Filters are [Rhai][]
expressions which yield a logical value (`true`/`false`).

### Special variables

Allay provides a set of variables available in the `allay` module:

#### `allay::os`

`allay::os` yields the operating system of the current host system. This may be any of the following:

- `"linux"`
- `"windows"`
- `"macos"`
- `"android"`
- `"ios"`
- `"openbsd"`
- `"freebsd"`
- `"netbsd"`
- `"wasi"`
- `"hermit"`
- `"aix"`
- `"apple"`
- `"dragonfly"`
- `"emscripten"`
- `"espidf"`
- `"fortanix"`
- `"uefi"`
- `"fuchsia"`
- `"haiku"`
- `"hermit"`
- `"watchos"`
- `"visionos"`
- `"tvos"`
- `"horizon"`
- `"hurd"`
- `"illumos"`
- `"l4re"`
- `"nto"`
- `"redox"`
- `"solaris"`
- `"solid_asp3"`
- `"vita"`
- `"vxworks"`
- `"xous"`

> [!EXAMPLE]
> ```toml,filepath=allay.toml
> [[plugin]]
> # ...
> when = 'allay::os == "windows"'
> ```

#### `allay::env`

`allay::env` contains all environment variables such as those provided in the `[env]` section of `allay.toml`.
For more information, read [Environment](#environment).

> [!EXAMPLE]
> ```toml,filepath=allay.toml
> [env]
> FEATURE_FOO = "on"
>
> [[plugin]]
> # ...
> when = 'allay::env.FEATURE_FOO == "on" && allay::env.COLORTERM == "truecolor"'
> ```

#### `allay::is_command`

`allay::is_command` is a function that can be used to test whether a certain command exists on the running machine.

> [!EXAMPLE]
> ```toml,filepath=allay.toml
> [[plugin]]
> # ...
> when = 'allay::is_command("pwsh") || allay::os == "windows"'
> ```

## Environment

Both filters and plugins themselves have access to the same environment. The base environment is inherited from the
Allay executable. That means that every environment variable available to Allay will be passed onto plugins and filters.
On top of that, Allay provides the following environment variables:

<!-- NOTE: Please use alphabetical order when adding new environment variables. -->

- Every environment variable defined in the `[env]` section of `allay.toml`
- `ALLAY_PLUGIN_NAME` which is the name of the currently running plugin, or in the context of a filter the name of the plugin the filter belongs to.
- `ALLAY_PROFILE` which may be `debug` or `release` depending in which mode is used for the build.
- `ALLAY_PROJECT_ROOT` which is the path to the directory containing `allay.toml` of the current project.
- `ALLAY_VERSION` which reflects the vesion of Allay currently used by the running machine.

[Rhai]: https://rhai.rs/
[Ruby]: https://www.ruby-lang.org/
