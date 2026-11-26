# RobloxPackages

RobloxPackages downloads the Luau packages that ship with Roblox, such as Foundation, Promise, and Dash, and copies them into a folder in your project. It installs the newest Roblox version by default, can pin an older one, and can keep only the packages you name.

## Install

With [Rokit](https://github.com/rojo-rbx/rokit):

```sh
rokit add flipbook-labs/roblox-packages
```

## Use

No setup is needed. The packages download from Roblox's CDN, so there is no account, token, or config file to create.

Run `install` with a destination folder:

```sh
roblox-packages install RobloxPackages
```

Each package is a module in `RobloxPackages/Packages/`. Foundation, for example, lands at `RobloxPackages/Packages/Foundation.lua`. The destination also receives the release's `rotriever.toml` and `rotriever.lock`. Files with the same names are overwritten, and the folder is not cleared first.

To install only some packages, pass `-d` once for each one. The tool also keeps the packages those depend on:

```sh
roblox-packages install RobloxPackages -d Foundation -d Promise
```

## Versions

Without `-v`, `install` uses the newest Roblox version. To see what is available, run `list`:

```sh
roblox-packages list --limit 5
```

`list` prints the newest versions last, and `--limit` defaults to 20. To install a specific version, pass `-v` with a prefix of a version ID from `list`, long enough to match only one version:

```sh
roblox-packages install RobloxPackages -v <version-id-prefix>
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT.
