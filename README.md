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
fn return_is_not_supported_now_T_T(x: &str) -> &str {
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

    let s = return_is_not_supported_now_T_T("fuga");
    println!("s = {}", s);

    S::method_test();
}
```

## Output
```shell
Enter into the function `test_arg_ret` [src/main.rs#line=5]
test_arg_ret x = hoge
Return from the function `test_arg_ret` [src/main.rs#line=5]
s = hoge
Enter into the function `return_is_not_supported_now_T_T` [src/main.rs#line=11]
test_arg_ret x = fuga
s = fuga
Enter into the function `method_test` [src/main.rs#line=20]
S::method_test
Return from the function `method_test` [src/main.rs#line=20]
```
