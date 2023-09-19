use log::{info, trace, warn, debug, error, Level, log_enabled};

fn main() {
   // 注意，env_logger 必须尽可能早的初始化
   env_logger::init();

   debug!("this is a debug {}", "message");
   error!("this is printed by default");

   if log_enabled!(Level::Info) {
       let x = 3 * 4; // expensive computation
       info!("the answer was: {}", x);
   }
  println!("Hello, world!");
  deal_with_something()
  
}

pub fn deal_with_something() {
  let a = 8;
  let b = 16;
  trace!("a trace log {}", a);
  info!("a info log");
  warn!("a warn log {}", b);
}