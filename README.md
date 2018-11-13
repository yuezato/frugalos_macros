# frugalos_macros

# Examples
## `trace_info` macro
```rust
extern crate frugalos_macros;
use frugalos_macros::trace_info;

#[trace_info]
fn test_arg_ret(x: &str) -> &str {
    println!("test_arg_ret x = {}", x);
    x
}

#[trace_info]
fn return_is_supported(x: &str) -> &str {
    println!("test_arg_ret x = {}", x);
    return x;
}

struct S {}

impl S {
    #[trace_info]
    fn method_test() {
        println!("S::method_test");
    }
}

fn main() {
    let s = test_arg_ret("hoge");
    println!("s = {}", s);

    let s = return_is_supported("fuga");
    println!("s = {}", s);

    S::method_test();
}
```

## Output
```shell
入==>  `test_arg_ret` [src/main.rs#line=5]
test_arg_ret x = hoge
<==出  `test_arg_ret` [src/main.rs#line=5]
s = hoge
入==>  `return_is_not_supported_now_T_T` [src/main.rs#line=11]
test_arg_ret x = fuga
<==出  `return_is_not_supported_now_T_T` [src/main.rs#line=11]
s = fuga
入==>  `method_test` [src/main.rs#line=20]
S::method_test
<==出  `method_test` [src/main.rs#line=20]
```
