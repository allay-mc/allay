# Changelog

Allay adheres to [Semantic Versioning](https://semver.org/).


## [unreleased][]

### Added

- Add support for `sync`ing on Linux.
- Better localization system: Now, not only `pack.name` and `pack.description` will be
  adapted to other languages, but also every other translation you add in any language
  file.


### Fixed

- `.gitignore` template now has the correct format
- Both stdout and stderr of plugins will now be printed to the console


## [0.1.0][] - 2023-05-20

_♻️ Rewrite_


## [0.1.0-beta.1][] - 2023-10-08

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


## [0.1.0-alpha.1][] - 2023-08-11

_🍰 Initial release_


[unreleased]: https://github.com/allay-mc/allay/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/allay-mc/allay/compare/v0.1.0-beta.1...v0.1.0
[0.1.0-beta.1]: https://github.com/allay-mc/allay/compare/v0.1.0-alpha.1...v0.1.0-beta.1
[0.1.0-alpha.1]: https://github.com/allay-mc/allay/releases/v0.1.0-alpha.1
