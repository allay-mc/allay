# Changelog

Allay adheres to [Semantic Versioning](https://semver.org/).


## [unreleased]

### Added

- Implement extra dependencies functionality for skin packs and world templates
  (#23).
- Add support for `exclude` field in the `[build]` section (#14).
- Add `use-custom-manifest` field (#5).
- Add `build` directory to gitignore.
- Executables can now be used as an alternative to scripts (as described
  [here](https://allay-mc.github.io/docs/scripts/executables.html)) (#15).
- Add `schema` command (only available with `config-schema` feature).
- Add `--bundle`, `--dir` and `--individual` flags for `build` command.
- `--release` can now be invoked with `-r` as well.

### Changed

- `base_game_version` can now be `"*"` (#9).
- `add` is now an optional feature and can be disabled.
- `doc` is now an optional feature and can be disabled.
- `--release` option for `build` now only depends on presence instead of a value.

### Fixed

- Resource packs now include extra dependencies from themselves instead of the ones
  from the behavior pack.

### Removed

- Remove unimplemented command `config`.
- Remove `development` field of `[project]` section (#21).
- Remove `build_type` field of `[build]` section.


## [0.1.0-beta.1] - 2023-10-08

### Added

- Add `doc` command.
- `add` command now handles unsuccessful HTTP responses.

### Fixed

- Localization now correctly falls back to the first match of the primary language
  group instead of the primary language.
- `add` command now uses `master` branch instead of `main` which does not exist.
- `add` command now adds `rb` file extension to added files.

### Changed

- Errors without much impact no longer stop the build process and instead just show
  an error message.
- `add` command no longer overrides already existing files.
- Exclude empty space is about section of the command when `NO_COLOR` is set.


## [0.1.0-alpha.1] - 2023-08-11

_🍰 Initial release_


[unreleased]: https://github.com/allay-mc/allay/compare/v0.1.0-beta.1...HEAD
[0.1.0-beta.1]: https://github.com/allay-mc/allay/compare/v0.1.0-alpha.1...v0.1.0-beta.1
[0.1.0-alpha.1]: https://github.com/allay-mc/allay/releases/v0.1.0-alpha.1
