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
入==>  `test_arg_ret` [ /Users/uezato/Rust/play_macros/src/main.rs:5 ]
test_arg_ret x = hoge
<==出  `test_arg_ret` [ /Users/uezato/Rust/play_macros/src/main.rs:5 ]
s = hoge
入==>  `return_is_supported` [ /Users/uezato/Rust/play_macros/src/main.rs:11 ]
test_arg_ret x = fuga
<==出  `return_is_supported` [ /Users/uezato/Rust/play_macros/src/main.rs:11 ]
s = fuga
入==>  `method_test` [ /Users/uezato/Rust/play_macros/src/main.rs:20 ]
S::method_test
<==出  `method_test` [ /Users/uezato/Rust/play_macros/src/main.rs:20 ]

```


## Settings
### For Mac & iTerm2 & Emacs Users
First, please save the following file into "your/favarite/path/emacs_for_iterm2"
```shell
#!/bin/bash

file=$1
line=$2

if [ "$line" = "" ]; then
 /usr/local/bin/emacs $file &
else
 /usr/local/bin/emacs +$line $file &
fi
```

Next, please open the preferences of iTerm2 and change it as follows:
<img width="900" src="https://user-images.githubusercontent.com/26326704/48461885-425cbb00-e819-11e8-96e6-2d4e396e88b6.png">

Finally, you can open a file at a given line by your emacs with `command+clicking` on a file link:
<img width="900" src="https://user-images.githubusercontent.com/26326704/48462265-d9764280-e81a-11e8-9914-2d1856c19e52.png">
