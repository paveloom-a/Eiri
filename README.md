# Description

## Notes for developers

To lint the code and the manifest file using [`clippy`](https://github.com/rust-lang/rust-clippy), run

```shell
cargo lint
```

Before building, make sure you have all the [dependencies](https://github.com/fltk-rs/fltk-rs#dependencies) installed, including [Ninja](https://ninja-build.org/) for faster builds. Then, run

```shell
cargo build --release
```

to build the binary.
