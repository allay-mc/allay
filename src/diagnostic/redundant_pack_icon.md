# Redundant Pack Icon

Often an add-on that consists of multiple packs (for example a behavior pack together with a resource pack)
use the same icon. Sometimes howver they use a different icon in order to identify the kind of pack better.
Allay by default uses the `pack_icon.png` file located in the root of your project (or uses its own if
absent) for all packs. To override this behavior and use individual pack icons, you need to modify your
configuration file like below.

```toml
[project]
# ...

[localization]
# ...

[BP]
custom-pack-icon = true

[RP]
custom-pack-icon = true
```

Now you can use custom `pack_icon.png` files in the root of the appropiate source directories (`src/BP/` and
`src/RP`).

```text
.
├── allay.toml
├── pack_icon.png
└── src/
    ├── BP/
    │   ├── entities/
    │   └── pack_icon.png
    ├── RP/
    │   ├── textures/
    │   └── pack_icon.png
    ├── SP/
    │   └── skins.json
    └── WT/
```
