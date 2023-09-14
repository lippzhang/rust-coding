use std::fmt::Display;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let mark_twain = "Samuel Clemens".to_owned();

    async_print(mark_twain).await;
}

// error
// fn async_print<T: Display>(msg: T) -> JoinHandle<()> {
//     tokio::task::spawn(async move {
//         println!("{}", msg);
//     })
// }

fn async_print<T: Display + Send + 'static>(msg: T) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        println!("{}", msg);
    })
}