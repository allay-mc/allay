# Using TypeScript

[TypeScript][] is a commonly chosen alternative to JavaScript when working with Minecraft's
[Script API][]. When working with the [Script API][], it's a good idea to initialize the Allay
project with [npm][] or other package managers. This guide will use [npm][] as the package
manager.

First, initialize the project for [npm]:

```console
npm init
```

Then edit the generated `package.json` to look similar to the following:

```json,filepath=package.json
{
  "name": "myproject",
  "version": "0.1.0",
  "license": "MIT",
  "scripts": {
    "build": "tsc --outDir $ALLAY_PREBUILD"
  },
  "dependencies": {
    "@minecraft/server": "^1.13.0",
    "@minecraft/server-ui": "^1.3.0",
    "typescript": "next"
  }
}
```

We also create a `tsconfig.json` required to transpile our TypeScript files correctly:

```json,filepath=tsconfig.json
{
  "compilerOptions": {
    "rootDir": "./src",
    "module": "NodeNext",
    "moduleResolution": "nodenext",
    "target": "ES2016",
    "noImplicitAny": true,
    "removeComments": true,
    "preserveConstEnums": true,
    "sourceMap": false
  },
  "include": ["src/**/*"]
}
```

We won't use the [npm] CLI directly to run the `build` script like you usually would do. Instead,
we invoke the `build` script by using an Allay plugin:

```toml,filepath=allay.toml
# ...

[[plugin]]
name = "transpile typescript"
run = "npm"
args = ["run", "build"]
```

This is neccessary as the `package.json` makes use of the environment variable `ALLAY_PREBUILD`
which is only available when run by Allay.

The built add-on will still contain TypeScript files alongside the generated JavaScript files.
Because we don't need these files, we can use a plugin that removes the TypeScript files from the
build.

```toml
# ...

[[plugin]]
name = "exclude typescript files"
run = "plugins/exclude.rb"
with = "ruby"
options = { patterns = ["**/*.ts"] }
```

You can find this plugin [here](https://github.com/allay-mc/plugins/tree/master/plugins/exclude).

[npm]: https://www.npmjs.com/
[Script API]: https://learn.microsoft.com/en-us/minecraft/creator/scriptapi/?view=minecraft-bedrock-stable
[TypeScript]: https://www.typescriptlang.org/
