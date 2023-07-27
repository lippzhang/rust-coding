# macro_custom_coding
文档地址： https://zhuanlan.zhihu.com/p/608978583
## 声明宏
```rust
#[macro_export]
macro_rules! my_macro_test {
    ($ctx: ident, $exp: expr) => {
        match $exp {
          Ok(x) => {
            println!("$ctx is {:?}", $ctx);
            println!("{:?}", x);
            x
          },
          Err(e) => {
            println!("{:?}", e);
            e
          }
        }
    };
}

```

## 过程宏过程
1、 cargo new xxxx -lib
2、在 cargo.toml中的书写
```js
[lib]
proc-macro = true
```
3、在lib.rs写宏
```rs
use proc_macro::TokenStream;

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}
```