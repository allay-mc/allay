# `init.rhai`

Sometimes you may find yourself using repeating patterns in your `allay.toml` configuration file:

```toml,fp=allay.toml
[[plugin]]
# ...
when = 'allay::env.ALLAY_PROFILE == "debug"'

[[plugin]]
# ...
when = 'allay::env.ALLAY_PROFILE == "debug"'
```

This can be simplified by creating a file named `init.rhai`:

```javascript,icon=@https://rhai.rs/book/favicon.png,fp=init.rhai
export const debug = allay::env.ALLAY_PROFILE == "debug";
```

```toml,fp=allay.toml
[[plugin]]
# ...
when = 'debug'

[[plugin]]
# ...
when = 'debug'
```

