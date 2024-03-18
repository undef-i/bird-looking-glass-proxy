# Bird Looking Glass Proxy
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

Bird Looking Glass Proxy is a svelte web API contrived to administer BIRD and Traceroute instances, furnishing a streamlined interface for network diagnostics and management.

[contributors-shield]: https://img.shields.io/github/contributors/undef-i/bird-looking-glass-proxy.svg?style=flat-square
[contributors-url]: https://github.com/undef-i/bird-looking-glass-proxy/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/undef-i/bird-looking-glass-proxy.svg?style=flat-square
[forks-url]: https://github.com/undef-i/bird-looking-glass-proxy/network/members
[stars-shield]: https://img.shields.io/github/stars/undef-i/bird-looking-glass-proxy.svg?style=flat-square
[stars-url]: https://github.com/undef-i/bird-looking-glass-proxy/stargazers
[issues-shield]: https://img.shields.io/github/issues/undef-i/bird-looking-glass-proxy.svg?style=flat-square
[issues-url]: https://github.com/undef-i/bird-looking-glass-proxy/issues
[license-shield]: https://img.shields.io/github/license/undef-i/bird-looking-glass-proxy.svg?style=flat-square
[license-url]: https://github.com/undef-i/bird-looking-glass-proxy/blob/main/LICENSE
```
USAGE:
    bird-looking-glass-proxy [OPTIONS]

OPTIONS:
    -c, --config <FILE>    Set the config file
    -e, --example          Export sample config file
    -h, --help             Print help information
```
## Commencing
To commence with Bird Looking Glass Proxy, follow the steps expounded below:
### Requisites
Before you embark, ensure you have the subsequent requisites:
* **BIRD**: Ensure that BIRD (BGP Internet Routing Daemon) is installed and configured properly. Bird Looking Glass Proxy relies on BIRD for routing information.
* **Traceroute**: Make sure that the traceroute utility is installed and available in your system. It is utilized for network diagnostics and troubleshooting, which is imperative for the functionality of Bird Looking Glass Proxy.
* **Rust**: If you intend to compile the project from source, ensure you have Rust installed. You can install Rust by following the instructions on rustup.rs.
### Installation
You can procure the binary executable for Bird Looking Glass Proxy from the [releases page](https://github.com/undef-i/bird-looking-glass-proxy/releases). Alternatively, you have the option to compile the project on your own.

1. Make sure that `bird` and `traceroute` are installed and available.
2. Obtain the binary executable by either downloading it from the release or compiling the project with `cargo`. The compiled binary will be available in the target/release directory.
3. Use the command `bird-looking-glass-proxy --example > config.toml` to create a template configuration file and modify it to your needs.
4. Run the executable.
### Tips
* The default configuration file is located at `./config.toml`, but you can customize the file location using the `-c` or `--config` parameter.

* Versions built with the `*-unknown-linux-gnu` toolchain may not function correctly on systems with `GLIBC` versions below `2.35`. To resolve this, use a statically linked version built with the `*-unknown-linux-musl` toolchain.

## Perspective
Our future aspirations for Bird Looking Glass Proxy encompass:

* Enhancing compatibility with different system architectures.
* Augmenting logging and error handling mechanisms.
* Implementing additional features for enhanced customization.

## Collaborative Endeavors
Contributions are cordially invited! Feel unshackled to open issues or submit pull requests to aid in ameliorating Bird Looking Glass Proxy.

## License
Bird Looking Glass Proxy is licensed under the GNU General Public License v3.0 (GPLv3). Peruse the LICENSE file for details.

## Contact
For inquiries or support, please reach out to i@nedifinita.com.



