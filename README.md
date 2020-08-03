# cc-version
cc version detection for build.rs

[![Actions Status](https://github.com/dalance/cc-version/workflows/Regression/badge.svg)](https://github.com/dalance/cc-version/actions)
[![Crates.io](https://img.shields.io/crates/v/cc-version.svg)](https://crates.io/crates/cc-version)
[![Docs.rs](https://docs.rs/cc-version/badge.svg)](https://docs.rs/cc-version)

## Usage

```Cargo.toml
[build-dependencies]
cc-version = "0.1.0"
```

## Example

```rust
use cc_version::cc_version;

fn main() {
    let builder = cc::Build::new();
    let tool = builder.get_compiler();
    let version = cc_version(&tool).unwrap();
    println!("cargo:warning=cc version {} is detected.", version);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
