# MSI Property Getter

This is an example using the [`msi`](https://crates.io/crates/msi) crate to get a property from a Windows Installer package on any platform.

To run the example and retrieve the `ProductCode` by default:

```bash
cargo run --example msigetprop -- package.msi
```

You can get additional info (`-v`) or trace (`-vv`) messages, or get another property value (`-p`). See `--help` for more information:

```bash
cargo run --example msigetprop -- --help
```

## Install

If you would like to install the examples:

```bash
cargo install --path . --examples
```

Make sure `~/.cargo/bin` is in your `PATH` to run examples without specifying the full path to the executable each time.
