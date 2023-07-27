// 文档地址： https://zhuanlan.zhihu.com/p/608978583
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

#[macro_export]
macro_rules! my_vec {
    // 没带任何参数的 my_vec，我们创建一个空的 vec
    () => {
        std::vec::Vec::new()
    };

    // 处理 my_vec![1, 2, 3, 4]
    ($($el:expr),*) => ({
        let mut v = std::vec::Vec::new();
        $(v.push($el);)*
        v
    });

    // 处理 my_vec![0; 10]
    ($el:expr; $n:expr) => {
        std::vec::from_elem($el, $n)
    }
}


fn main() {
    // let mut v = my_vec![];
    // v.push(1);

    // // 调用时可以使用 [], (), {}
    // let _v = my_vec!(1, 2, 3, 4);
    // let _v = my_vec![1, 2, 3, 4];c
    // let v = my_vec! {1, 2, 3, 4};
    // println!("{:?}", v);

    // println!("{:?}", v);

    // let v = my_vec![1; 10];
    // println!("{:?}", v);
    // ===================
    let a = String::from("lippzhang");
    let b = my_macro_test!(a, Err(5));
    let b = my_macro_test!(a, Ok(5));
    println!("{}", b)
    // =================================================================
}

// # 声明宏
// ## 比如 vec![]、println!、以及 info! 都是声明宏
// ## 使用场景： 如果重复性的代码无法用函数来封装，那么可以选择用声明宏


// # 过程宏：主要是进行深度操作和改写代码语法树，更加灵活强大

// 过程宏可以细分为3种
// ## 函数宏（function-like macro）：看起来像函数的宏，但是是在编译期进行处理的
// > 比如 sqlx 里的 query 宏，它内部展开出一个 expand_query 函数宏
// ## 属性宏（attribute macro）：可以在其他代码块上添加属性，为代码块提供更多功能
// > 比如 rocket 的 get/put 等路由属性
// ## 派生宏（derive macro）：为 derive 属性添加新的功能，是平时使用最多且最复杂的宏
// > 比如 #[derive(Debug)]为数据结构提供 Debug trait 的实现

// ## 使用场景
// 派生宏：派生宏可以在特定的场景使用，有需要则可以使用
// 比如希望一个数据结构能提供 Debug trait 的能力，但为自己定义的每个数据结构实现 Debug trait 太过繁琐，而且代码所做的操作又都是一样的，这时就可以考虑使用派生宏来简化这个操作
// 一般来说，如果你定义的 trait 别人实现起来有固定的模式可循，那么可以考虑为其构建派生宏。

// 比如Rust的serde库，我们的数据结构只需要添加 #[derive(Serialize, Deserialize)] 宏，就可以轻松序列化成 JSON、YAML 等好多种类型（或者从这些类型中反序列化）
// 函数宏和属性宏并没有特定的使用场景。例如sqlx 用函数宏来处理 SQL query、tokio 使用属性宏
// #[tokio::main] 来引入 runtime。它们可以帮助目标代码的实现逻辑变得更加简单，但一般除非特别必要，否则并不推荐写