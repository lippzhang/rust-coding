use proc_macro::TokenStream;
mod raw_builder;
use raw_builder::BuilderContext;
#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}

// 创建一个名为RawBuilder的宏，把input打印出来
#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    println!("打印的input {:#?}", input);
    BuilderContext::render(input).unwrap().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       
    }
}
