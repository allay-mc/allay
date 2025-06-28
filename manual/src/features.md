# Features

Allay offers a set of features that you can enable or disable when installing Allay.

## `git`

This feature enables [git][] related features such as

- downloading resources such as JSON schemas and Minecraft version history,
- initialization of [git][] repositories when creating a new Allay project and
- fetching user data such as name and email.

This feature is **enabled by default**.

## `json-validation`

This feature enables JSON validation.

This feature is **enabled by default** and implies [`git`](#git).

## `json-schema`

This feature offers a JSON schema for the [Allay configuration file][].

This feature is **enabled by default**.

## `completions`

This feature enables the `completions` command of the Allay command-line interface which can be used to obtain [command-line completions][].

This feature is **enabled by default**.

## `share`

This feature enables the `share` command of the Allay command-line interface which can be used to send built add-ons to other devices in the same network.

This feature is **enabled by default**.

## `manual`

This feature enables the `manual`/`docs` command of the Allay command-line interface which can be used to open the manual for the installed version of Allay.

> [!NOTE]
> When this feature is enabled, the entire manual is embedded into the Allay executable which increases the binary size by a lot.

This feature is **enabled by default**.

[command-line completions]: https://en.wikipedia.org/wiki/Command-line_completion
[git]: https://git-scm.com/
