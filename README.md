# bird-looking-glass
```
USAGE:
    bird-looking-glass-proxy [OPTIONS]

OPTIONS:
    -c, --config <FILE>    Set the config file
    -e, --example          Export sample config file
    -h, --help             Print help information
```
## Quick Start
0. Make sure that `bird` and `traceroute` are installed and available.
1. Obtain the binary executable by either downloading it from the release or compiling the project with `cargo`.
2. Use the command `bird-looking-glass-proxy --example > config.toml` to create a template configuration file and modify it to your needs.
3. Run the executable.

### Tips
The default configuration file is located at `./config.toml`, but you can customize the file location using the `-c` or `--config` parameter.

## Known issues
* Versions built with the `*-unknown-linux-gnu` toolchain may not function correctly on systems with `GLIBC` versions below `2.35`. To resolve this, use a statically linked version built with the `*-unknown-linux-musl` toolchain.
