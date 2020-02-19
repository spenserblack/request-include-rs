# curl-include
Use curl to `include!`

# Usage
```rust
use curl_include::include_str as curl_str;

const DATA: &str = curl_str!("API-url");
```
