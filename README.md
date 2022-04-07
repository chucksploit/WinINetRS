This is in really early stages but I wanted to call dips on the create name.
Expect changes to the API.

It is meant as a small wrapper around [wininet](https://docs.microsoft.com/en-us/windows/win32/api/wininet/),
so that higher level HTTP client (like [generic-async-http-client](https://github.com/User65k/generic-async-http-client)) can make use of it.

It provides the ANSI and the unicode (`--features unicode`) versions of WinINet.

For now this works:

1. `cross +nightly rustc -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-gnu --release --example async`
2. `wine target/x86_64-pc-windows-gnu/release/examples/async.exe`
