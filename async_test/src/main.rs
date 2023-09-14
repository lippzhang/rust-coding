

#[tokio::main]
async fn main() {
    println!("One");
    let future = prints_two();
    println!("Three");
    // 移除下面一行的注释，并且上下移动该行，看一下行为有什么变化。
    future.await; 

    // 异步闭包不稳定
    // let fut = async || {};

    // 异步代码块
    let msg = "Hello world".to_owned();

    let async_block = || async {
        println!("{}", msg);
    };
    async_block().await;
}
async fn prints_two() {
    println!("Two")
}
 
// fn regular_fn() -> String {
//     "i am a regular function".to_owned()
// }
 
// async fn async_fn() -> String {
//     "i am a regular function".to_owned()
// }