# Plugins

Plugins are little programs that extend Allay by for instance transforming the source data.


## Writing Plugins

To write your own plugin you should be comfortable with some programming language.


### Environment Variables

When Allay executes a plugin, it provides a few environment variables that help you managing files for
instance.

```admonish caution
**Never** let a plugin modify source files (files within one of the `BP`, `RP`, `SP` or `WT` directories).
Allay under the hood copies all source files to a temporary directory where files can be freely modified.
After all plugins run, these files are zipped to one build file. The environments variables point to
the paths in the temporary directory ("prebuilt directory").
```

- `ALLAY_DEBUG` --- Whether the project is built in debug mode.
- `ALLAY_PREBUILD` --- The root of the prebuilt directory.
- `ALLAY_PROJECT_ROOT` --- The path to the root of the project (the directory with the `allay.toml` file).
- `ALLAY_VERSION` --- The version of Allay that is beeing used.

On top of that users may define extra environment variables in the
[`[env]`](./configuration.md#the-env-section) section of the configuration file.


### Configuration

Plugins may be configured in three different ways:

1. `options` field in `[[plugin]]` section --- Tranforms the value into JSON and passes it as the first (or
  second when `with` is set) argument for the executable.
2. `args` field in `[[plugin]]` section --- Passes each string to the executable as arguments.
3. `env` section --- The plugin can access environment variables set in the configuration file.
4. Code modification --- The code itself can be modified.

Option 1 requires the program to parse JSON string. Some programming languages have this feature in their
standard library like Python and JavaScript but it's always possible to to use third-party dependencies.
Option 2 can be used when option 1 cannot be satisfied. This option is also used commonly for command-line
applications that are not specifically designed for Allay. Note that this approach may be limited as Allay
passes value as is and don't make use of things like [filename expansion][]. Option 3 should only be used as a
last resort when the plugin is not capable of accessing arguments or environment variables which is almost
never the case.


### Write and Use a Plugin

Below is an example of a plugin written in the Python programming language.

```python,filepath=plugins/info.py
import os

version = os.environ["ALLAY_VERSION"]
project_root = os.environ["ALLAY_PROJECT_ROOT"]

print(f"Allay v{version}")
print(project_root)
```

By convention, plugins related to the Allay project are placed in a directory named `plugins`:

```text,nolang
.
├── allay.toml
├── plugins/
│   └── info.py
└── src/
```

To tell Ally to run this plugin, we need to mention it in the `allay.toml` file.

```toml,filepath=allay.toml
# ...

[[plugin]]
run = "plugins/info.py"
with = "python3"
```

Running the `info.py` script yourself is likely going to yield unexpected results as the environment
variables are not set. Plugins are intended to be executed by Allay and can therefore only be triggered by
building the project (`allay build`).


### Filters

Filters can be added to plugins to only run them when certain conditions are met. This can be achieved by
adding the `when` field:

```toml,filepath=allay.toml
# ...

[[plugin]]
run = "plugins/info.py"
with = "python3"
when = 'env("ALLAY_DEBUG") == "1"'
```

In this case the plugin would only run if the project is beeing built in debug mode. Allay uses [rhai][] for
evaluation. You can learn [rhai][] by reading its [documentation][rhai docs]. Allay provides the following
functions which are useful for writing filters:

- `arch()` --- A string describing the architecture of the CPU that is currently in use.
- `dll_extension()` --- Specifies the file extension used for shared libraries on this platform that goes after the dot. Example value is `so`.
- `dll_prefix()` --- Specifies the filename prefix used for shared libraries on this platform. Example value is `lib`.
- `dll_suffix()` --- Specifies the filename suffix used for shared libraries on this platform. Example value is `.so`.
- `env(key)` --- Fetches the environment variable `key` from the current process or an empty string if `key` is absent or the value is invalid unicode.
- `env_present(key)` --- Evaluates `true` when the environment variable with the key `key` is present or `false` otherwise.
- `exe_extension()` --- Specifies the file extension, if any, used for executable binaries on this platform. Example value is `exe`.
- `exe_suffix()` --- Specifies the filename suffix used for executable binaries on this platform. Example value is `.exe`.
- `family()` --- The family of the operating system. Example value is `unix`.
- `os()` --- A string describing the specific operating system in use. Example value is `linux`.


[filename expansion]: https://www.gnu.org/software/bash/manual/html_node/Filename-Expansion.html
[rhai]: https://rhai.rs/
[rhai docs]: https://rhai.rs/book/language/comments.html
