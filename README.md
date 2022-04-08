[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![Crates.io][crates-badge]][crates-url]
[![Released API docs](https://docs.rs/win_inet/badge.svg)](https://docs.rs/win_inet)
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/win_inet.svg
[crates-url]: https://crates.io/crates/win_inet
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/User65k/WinINetRS/blob/main/LICENSE

This is in really early stages but I wanted to call dips on the create name.
Expect changes to the API.

It is meant as a small wrapper around [wininet](https://docs.microsoft.com/en-us/windows/win32/api/wininet/),
so that higher level HTTP client (like [generic-async-http-client](https://github.com/User65k/generic-async-http-client)) can make use of it.

It provides the ANSI and the unicode (`--features unicode`) versions of WinINet.

For now this works:

1. `cross +nightly rustc -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-gnu --release --example async`
2. `wine target/x86_64-pc-windows-gnu/release/examples/async.exe`
