# curl-include
Use curl to `include!`

# Usage
```rust
use curl_include::include_str as curl_str;

const DATA: &str = curl_str!("API-url");
```

# Warning
*__Use at your own risk__*. By requesting a value at compile-time, it is very possible that, with no changes to the code,
the value assigned may change between compilations. You should probably never use this in production, unless you're
absolutely sure that you will get the expected result when you compile.
