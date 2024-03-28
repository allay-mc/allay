# Introduction To Resource Packs

> This page explains how to reproduce the example from
> <https://learn.microsoft.com/en-us/minecraft/creator/documents/resourcepack?view=minecraft-bedrock-stable>
> with Allay.

![Image showing a pig deeply confused by its environment containing green dirt blocks](https://learn.microsoft.com/en-us/minecraft/creator/documents/media/resourcepack/introduction-to-resource-packs.jpg?view=minecraft-bedrock-stable)

Before building your first Add-On for Minecraft: Bedrock Edition, you need to create a pack to hold your
custom content. There are two types of packs that a creator can make: resource packs and behavior packs. For
this tutorial, we're going to be focusing on resource packs.

For Minecraft to find and use your resource files, you must set up the folders and files in a specific way.
This tutorial will guide you through creating this folder and file structure.

After running `allay init` create directories and files like shown below.

```diff,nolang
.
├── allay.toml
└── src/
    ├── BP/
    ├── RP/
+   │   └── textures/
+   │       └── blocks/
+   │           └── dirt.png
    ├── SP/
    └── WT/
```

The `dirt.png` file should be some texture of your choice. You can also download the green block below and
use it instead.

![Custom Dirt Texture](https://learn.microsoft.com/en-us/minecraft/creator/documents/media/resourcepack/dirt.png?view=minecraft-bedrock-stable)

The image should have a quadratic size (e.g. 16x16 or 32x32).

Now `allay build` the project and export it to Minecraft.

Your custom texture will be used on every dirt block in the world, but it will not be used on blocks of dirt
with grass on them because those blocks have a different name.

```admonish todo
Add images explaining how to enable the add-on.
```
