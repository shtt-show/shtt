# shtt
*Simple History Tracking Tool*


## Installation
```bash
cargo install --git https://github.com/shtt-show/shtt
```

## Commands
* [`drop`](docs/drop.md) - Drop a new release
* [`dump`](docs/dump.md) - Dump your repository state
* [`pull`](docs/pull.md) - Pull a new repo, or the latest code
* [`save`](docs/save.md) - Push your code to GitHub/Lab/Whatever
* [`wipe`](docs/wipe.md) - Blow your local, unsaved changes away


## Configuration
SHTT uses your existing git configuration for user name, email,
and authentication. If a `~/.gitconfig` is not found, `shtt`
will help you set one up.


## License
This project is licensed under the Mozilla Public License 2.0.
See the [LICENSE](LICENSE) file for details.
