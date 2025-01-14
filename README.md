# 💄 llvm-cov-pretty

More beautiful HTML reports for `llvm-cov` ([cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) specifically).

- Dark theme support (switches automatically based on your browser setting).
- Syntax highlighting.
- Reduced clutter from instantiation annotations.\
- More colors, because why not.
- Progress bars (like in grcov).

## Installation

### From source

To build the project from source, you need a recent installation of Rust. The recommended installation method is through [rustup](https://rustup.rs/). Then open up a terminal and install the project as follows:

```sh
cargo install llvm-cov-pretty
```

The binary will be installed into your `$HOME/.cargo/bin` folder by default. Please make sure it is available from your `$PATH`.

### From AUR

Arch linux users can use an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers) to install the project from the [AUR](https://aur.archlinux.org). For example, using `paru`:

```sh
paru -S llvm-cov-pretty
```

### Pre-build binaries

If you don't want to build the project yourself, you can download pre-build binaries instead. Have a look at the [release page](https://github.com/dnaka91/llvm-cov-pretty/releases) and find the appropriate file for your platform.

## Usage

The tool operates on the JSON outpu from llvm-cov. As cargo-llvm-cov directly prints those to the standard output, you can pipe the programs together like so:

```sh
cargo llvm-cov --json | llvm-cov-pretty
```

Of course you can do this in two steps as well:

```sh
cargo llvm-cov --json > coverage.json
llvm-cov-pretty coverage.json
```

There are a few extra commands (like generating shell completions) and options (like disabling instantiation annotations) as well. Have a look at the output of `llvm-cov-pretty --help`.

## Development

To start developing on the project, you'll need a few extra step in addition to the ones described in the [From source](#from-source) section.

- Install a recent version of Node.js and Yarn (usually comes bundled with Node.js).
- Ensure all submodules are initialized with `git submodule update --init`.

Then open up a new terminal and execute `yarn run watch`. It will watch the project files and rebuild the stylesheet at `assets/style.css` when needed. With that you're ready to hack along.

Before you create a custom build, it's recommended to stop the watch job again and run `yarn run build` once, which will create a minified and optimized version of the stylesheet. It is compiled into the binary during builds.

## License

This project is licensed under the [AGPL-3.0 License](LICENSE) (or
<https://www.gnu.org/licenses/agpl-3.0.html>).
