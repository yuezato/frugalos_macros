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

#[trace_info(comment="This is a comment.")]
fn comment_test1() {
    println!("can you see the above comment?");
}

#[trace_info(comment="[This is another comment.]")]
fn comment_test2() {
    println!("can you see the above comment?");
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

    comment_test1();

    comment_test2();
    
    let s = return_is_supported("fuga");
    println!("s = {}", s);

    S::method_test();
}
```

## Output
```shell
/Users/yuuya_uezato/public_frugalos/play_macros$ cargo run
   Compiling frugalos_macros v0.1.0 (/Users/yuuya_uezato/public_frugalos/frugalos_macros)
   Compiling play_macros v0.1.0 (/Users/yuuya_uezato/public_frugalos/play_macros)
    Finished dev [unoptimized + debuginfo] target(s) in 4.00s
     Running `target/debug/play_macros`
入==> `test_arg_ret` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:5 ]
test_arg_ret x = hoge
<==出 `test_arg_ret` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:5 ]
s = hoge
入==> `comment_test1` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:11 ]
      This is a comment.
can you see the above comment?
<==出 `comment_test1` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:11 ]
入==> `comment_test2` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:16 ]
      [This is another comment.]
can you see the above comment?
<==出 `comment_test2` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:16 ]
入==> `return_is_supported` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:21 ]
test_arg_ret x = fuga
<==出 `return_is_supported` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:21 ]
s = fuga
入==> `method_test` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:30 ]
S::method_test
<==出 `method_test` [ /Users/yuuya_uezato/public_frugalos/play_macros/src/main.rs:30 ]
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
