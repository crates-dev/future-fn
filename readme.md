<center>

# future-fn

[![](https://img.shields.io/crates/v/future-fn.svg)](https://crates.io/crates/future-fn)
[![](https://img.shields.io/crates/d/future-fn.svg)](https://img.shields.io/crates/d/future-fn.svg)
[![](https://docs.rs/future-fn/badge.svg)](https://docs.rs/future-fn)
[![](https://github.com/eastspire/future-fn/workflows/Rust/badge.svg)](https://github.com/eastspire/future-fn/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/future-fn.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/future-fn/)

> A Rust library providing macros to simplify the creation of asynchronous closures with external state captured by move. Useful for structuring asynchronous code with ease and clarity.

## Installation

To install `future-fn` run cmd:

```sh
cargo add future-fn
```

## Usage

```rust
use future_fn::*;
use std::time::Duration;
use tokio::time::sleep;

let string: String = String::from("test");
let number: i32 = 1;
let future_fn = future_fn!(string, number, {
    let tmp_string: String = String::from("test");
    assert_eq!(string, tmp_string);
    assert_eq!(number, 1);
});
future_fn().await;

let future_fn = future_fn!(string, number, |data| {
    let tmp_string: String = String::from("test");
    sleep(Duration::from_millis(360)).await;
    assert_eq!(string, tmp_string);
    assert_eq!(data, 1);
    assert_eq!(number, 1);
});
future_fn(1).await;

let future_fn = future_fn!(string, number, |data: i32| {
    let tmp_string: String = String::from("test");
    sleep(Duration::from_millis(360)).await;
    assert_eq!(string, tmp_string);
    assert_eq!(data, 1);
    assert_eq!(number, 1);
    sleep(Duration::from_millis(360)).await;
});
future_fn(1).await;

let future_fn = future_fn!(string, number, |data: i32| {
    let tmp_string: String = String::from("test");
    sleep(Duration::from_millis(360)).await;
    assert_eq!(string, tmp_string);
    assert_eq!(data, 1);
    assert_eq!(number, 1);
});
future_fn(1).await;
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [eastspire <root@ltpp.vip>](mailto:root@ltpp.vip).
