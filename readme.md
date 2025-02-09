<center>

# async-func

[![](https://img.shields.io/crates/v/async-func.svg)](https://crates.io/crates/async-func)
[![](https://docs.rs/async-func/badge.svg)](https://docs.rs/async-func)
[![](https://github.com/ltpp-universe/async-func/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/async-func/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/async-func.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/async-func/)

> A Rust library providing macros to simplify the creation of asynchronous closures with external state captured by move. Useful for structuring asynchronous code with ease and clarity.

## Installation

To install `async-func` run cmd:

```sh
cargo add async-func
```

## Usage

```rust
use async_func::*;

let string: String = String::from("test");
let number: i32 = 1;
let func = async_func!(string, number, {
    let tmp_string: String = String::from("test");
    assert_eq!(string, tmp_string);
    assert_eq!(number, 1);
});
func().await;

let func = async_func!(string, number, |data| {
    let tmp_string: String = String::from("test");
    assert_eq!(string, tmp_string);
    assert_eq!(data, 1);
    assert_eq!(number, 1);
});
func(1).await;
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
