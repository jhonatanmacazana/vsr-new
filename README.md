<h1 align="center">
   vsr-new
</h1>

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

### From source

If you want to build `vsr-new` from source, you need Rust on your OS. You can then use `cargo` to build everything

``` bash
cargo install vsr-new
```

## 🤔 To-do

* [ ] CI/CD
* [ ] Distribute binary with Chocolatey (Windows)
* [ ] Distribute binary with apt (Linux)
* [ ] Dsitribute binaries from github
