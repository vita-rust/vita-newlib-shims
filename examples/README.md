# Examples

These examples demonstrate how to build applications for Sony PlayStation Vita in Rut with various degrees of complexity.

The examples use [cargo make](https://github.com/sagiegurari/cargo-make) for building, and require you to have [Vita SDK](https://vitasdk.org/) installed.

Also, you must have an environment variable `VITASDK` pointing to the location of Vita SDK, and your `PATH` must contain `$VITASDK/bin`, for example like that:

```sh
export VITASDK=/opt/vitasdk
export PATH=$VITASDK/bin:$PATH
```

You can add it to your `.bashrc` (or another configuration file if you are using a different shell), or you can use a tool like [direnv](https://direnv.net/), and put this in a `.envrc`.

## Building

To build a `vpk` for every example, run from the `examples` folder:

```sh
cargo make vpk
```

To build a specific example, run this command from the specific sub-folder.

## Running

Uploading and running `vpk` can be automated. The examples provide some `cargo make` commands for that.

In order for it to work, you must have `curl` and `nc` installed on your system, as well as [vitacompanion](https://github.com/devnoname120/vitacompanion) and [PrincessLog](https://github.com/CelesteBlue-dev/PSVita-RE-tools/tree/master/PrincessLog/build) installed on your Vita.

You will need to set up environment variables to connect to your Vita. Take a look at `Makefile.toml` in the examples.

### Commands

For the list of the commands see [Makefile.toml](./Makefile.toml)

To upload the `vpk` to Vita using [vitacompanion](https://github.com/devnoname120/vitacompanion), run:

```sh
cargo make vita-upload
```

This will upload the `vpk` to `ux0:/download` and open `VitaShell`. You will have to install the `vpk` manually.

To run the installed `vpk` on your Vita, run:

```sh
cargo make vita-run
```

To capture stdout and stderr from your Vita, install and set up `PrincessLog` and run:

```sh
cargo make vita-log
```

To download and parse a core dump:

```sh
cargo make vita-coredump
