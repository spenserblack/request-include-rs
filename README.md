# request-include
`include!` from a request

This is basically just a shortcut to skip downloading a response, and using `include!`/`include_str!` on that downloaded file.

# Usage
```rust
use request_include::include_str as request_str;

const DATA: &str = request_str!("API-url");
```

# Warning
*__Use at your own risk__*. By requesting a value at compile-time, it is very possible that, with no changes to the code,
the value assigned may change between compilations. You should probably *never* use this in production, unless you're
*absolutely sure* that you will get the expected result when you compile. If you use this in your project, you should probably
*never* expect anyone who uses your project to compile it themselves, and instead instruct users to download pre-built
binaries.
