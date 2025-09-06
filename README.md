# roblox-packages

Tool for extracting Luau packages from Roblox Studio.

## Installation

The preferred installation method is using [Rokit](https://github.com/rojo-rbx/rokit), a toolchain manager for Roblox projects. Rokit can manage your installed version of roblox-packages and other ecosystem tools, and allows you to easily upgrade to newer versions as they become available.

### Installing Rokit

Follow the installation instructions on the Rokit page.

### Installing roblox-packages

Navigate to your project directory using your terminal, and run the following command:
Terminal

```
rokit add flipbook-labs/roblox-packages
```

### Upgrading roblox-packages

When a new version of roblox-packages becomes available, Rokit makes it easy to upgrade. Navigate to your project directory using your terminal again, and run the following command:
Terminal

```
rokit update flipbook-labs/roblox-packages
```

If you prefer to install roblox-packages globally and have it accessible on your entire system, instead of only in a specific project, you can do this with Rokit as well. Just add the --global option to the end of the commands above.
```sh
rokit add flipbook-labs/roblox-packages
```

You can download pre-built binaries for most systems directly from the GitHub Releases page. There are many tools that can install binaries directly from releases, and it’s up to you to choose what tool to use. Lune is compatible with both Foreman and Aftman.

Usage:

## License

[MIT License](LICENSE)
