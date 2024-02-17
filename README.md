# Bird Looking Glass Proxy
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

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

> Warning: This project has serious security vulnerabilities and is not recommended for use.


```
USAGE:
    bird-looking-glass-proxy [OPTIONS]

OPTIONS:
    -c, --config <FILE>    Set the config file
    -e, --example          Export sample config file
    -h, --help             Print help information
```
### Tips
* The default configuration file is located at `./config.toml`, but you can customize the file location using the `-c` or `--config` parameter.

* Versions built with the `*-unknown-linux-gnu` toolchain may not function correctly on systems with `GLIBC` versions below `2.35`. To resolve this, use a statically linked version built with the `*-unknown-linux-musl` toolchain.

## License
Bird Looking Glass Proxy is licensed under the GNU General Public License v3.0 (GPLv3). Peruse the LICENSE file for details.
