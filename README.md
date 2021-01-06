# request-include

[![Build Status](https://travis-ci.com/spenserblack/request-include-rs.svg?branch=master)](https://travis-ci.com/spenserblack/request-include-rs)

`include_str!` from a request

This is basically just a shortcut to skip downloading a response, and using `include_str!` on that downloaded file.

# Usage
```rust
use request_include::include_str as request_str;

// Without user agent
const DATA: &str = request_str!("API-url");

// With user agent (recommended)
const OTHER_DATA: &str = request_str!("API-url", "my user agent");
```

# Warning
*__Use at your own risk__*. By requesting a value at compile-time, it is very possible that, with no changes to the code,
the value assigned may change between compilations.
