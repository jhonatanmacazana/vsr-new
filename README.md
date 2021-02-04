<h1 align="center">
   vsr-new
</h1>

<p align="center">
    <a href="https://github.com/jhonatanmacazana/vsr-new"><img src="https://img.shields.io/github/v/release/jhonatanmacazana/vsr-new" alt="release version"/></a>
    <a href="https://github.com/jhonatanmacazana/vsr-new"><img src="https://img.shields.io/github/downloads/jhonatanmacazana/vsr-new/total" alt="Release"/></a>
    <a href="https://github.com/jhonatanmacazana/vsr-new"><img src="https://img.shields.io/github/workflow/status/jhonatanmacazana/vsr-new/CI?label=CI" alt="CI"/></a>
    <a href="https://github.com/jhonatanmacazana/vsr-new"><img src="https://img.shields.io/github/workflow/status/jhonatanmacazana/vsr-new/Release?label=release" alt="Release"/></a>
</p>

<p align="center">
    <a href="#"><img src="https://img.shields.io/github/languages/top/jhonatanmacazana/vsr-new?color=purple" alt="language"/></a>
    <a href="https://crates.io/crates/vsr-new"><img src="https://img.shields.io/crates/v/vsr-new" alt="crates.io"/></a>
    <a href="https://crates.io/crates/vsr-new"><img src="https://img.shields.io/crates/d/vsr-new" alt="Release"/></a>
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

### Binaries

Checkout the latest [release](https://github.com/jhonatanmacazana/vsr-new/releases/latest). Download the binary for your specific OS.


### From source

If you want to build `vsr-new` from source, you need Rust on your OS. You can then use `cargo` to build everything

``` bash
cargo install vsr-new
```

## 🤔 To-do

* [x] CI/CD
* [x] Distribute binaries from github
* [ ] Distribute binary with Chocolatey (Windows)
* [ ] Distribute binary with apt (Linux)
