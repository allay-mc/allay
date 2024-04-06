# `com.mojang` folder not found on Android

Make sure you

- are using [Termux][]
- have used the [`termux-setup-storage` command][termux-setup-storage]
- have set the file storage location of Minecraft to "External"
  (Minecraft > Settings > Storage > File Storage Location)

If the steps above do not resolve your issues, find the `com.mojang` folder on your system and set the
environment variable `COM_MOJANG` to that path.

[Termux]: https://termux.dev/en/
[termux-setup-storage]: https://wiki.termux.com/wiki/Termux-setup-storage
