# Examples

These examples demonstrate how to build applications for Sony PlayStation Vita in Rut with various degrees of complexity.

## Prerequisites

- [Vita SDK](https://vitasdk.org/) must be installed, and `VITASDK` environment variable must be set to its location. You can add the environment variable to your `.bashrc` (or another configuration file if you are using a different shell), or you can use a tool like [direnv](https://direnv.net/), and put this in a `.envrc`.
- [cargo-vita](https://github.com/vita-rust/cargo-vita) tool is required for building `vpk`` files. Run this command to install it:
  ```sh
  cargo +nightly install cargo-vita
  ```
- [PrincessLog](https://github.com/CelesteBlue-dev/PSVita-RE-tools/tree/master/PrincessLog/build) is required for monitoring stdout/stderr from your Vita.
- [vitacompanion](https://github.com/devnoname120/vitacompanion) is required to upload `eboot.bin`/`vpk` files to Vita, and run commands.



## Building

To build the `vpk` for every example run the following from the `examples` folder:

```sh
cargo vita build vpk --release
```

To build the `vpk` for `std-tests` run:

```sh
cargo vita build vpk --release --package vita-std-tests --tests
```


To build the `vpk` for any specific package:

```sh
cargo vita build vpk --release --package {PACKAGE}
```


## Running

You can automate uploading of `vpk` to `ux0:/download/`, or update `eboot.bin` for already installed `vpk`.

To upload all `vpk` artifacts, use `--upload` flag of `vpk` subcommand:

```sh
cargo vita build vpk --upload --release
```

To update a specific `eboot.bin` and run it use `--update --run` flags of `eboot` subcommand. Keep in mind that `vpk` must be installed in order for that to work:

```sh
cargo vita build eboot --update --run --release --package {PACKAGE}
```
