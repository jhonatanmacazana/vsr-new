<h1 align="center">
   vsr-new
</h1>

<p align="center">
    <a href="https://github.com/jhonatanmacazana/vsr-new/releases/latest"><img src="https://img.shields.io/github/v/release/jhonatanmacazana/vsr-new" alt="release version"/></a>
    <a href="https://github.com/jhonatanmacazana/vsr-new"><img src="https://img.shields.io/github/downloads/jhonatanmacazana/vsr-new/total" alt="downloads-github"/></a>
    <a href="https://github.com/jhonatanmacazana/vsr-new/actions?query=workflow%3ACI"><img src="https://github.com/jhonatanmacazana/vsr-new/workflows/CI/badge.svg" alt="wf-CI"></a>
    <a href="https://github.com/jhonatanmacazana/vsr-new/actions?query=workflow%3ARelease"><img src="https://github.com/jhonatanmacazana/vsr-new/workflows/Release/badge.svg" alt="wf-Release"></a>
</p>

<p align="center">
    <a href="#"><img src="https://img.shields.io/github/languages/top/jhonatanmacazana/vsr-new?color=purple" alt="language"/></a>
    <a href="https://crates.io/crates/vsr-new"><img src="https://img.shields.io/crates/v/vsr-new" alt="crates.io"/></a>
    <a href="https://crates.io/crates/vsr-new"><img src="https://img.shields.io/crates/d/vsr-new" alt="downloads-crates"/></a>
</p>

Rust utility for creating new projects based on my [templates](https://github.com/jhonatanmacazana/vscode-boilerplates) repo.

## 🔥 Usage 

Create a `algorithms` project with the `c` template

``` bash
> vsr-new c algorithms
```

Create a `testing` project with the `server-creation` template

``` bash
> vsr-new server-creation testing
```

List all available templates

``` bash
> vsr-new --types
```

##  🛠️ Installation

### `npx` [Recommended]

Is highly recommended to use `npx` on every usage of the binary.
```
npx vsr-new --types
```
It works on Windows, Linux and MacOS if you have Node installed. It will use the latest release - stable version of the binary.

### With Binaries

Checkout the latest [release](https://github.com/jhonatanmacazana/vsr-new/releases/latest). Download the binary for your specific OS.

### From source

If you want to build `vsr-new` from source, you need Rust on your OS. You can then use `cargo` to build everything

``` bash
cargo install vsr-new
```

### `npm`

You can install the binary with the Node Package Manager
```bash
npm install -g vsr-new
```
This is the less adviced for new users. If you want to avoid the `npx` preffix on each usage, download the binary for your OS and add it to your **OS path**.

## 🤔 To-do

* [x] CI/CD
* [x] Distribute binaries from github
* [x] Distribute binaries with npm
* [ ] Distribute binary with Chocolatey (Windows)
* [ ] Distribute binary with apt (Linux)
* [ ] Distribute binary with brew (Mac OS)

## Contributions welcome

This project welcomes contributions of any kind, whether you want to add new features, improve the documentation or just want to give some feedback.

## License

`vsr-new` is published under the MIT license. See the LICENSE file for more information.