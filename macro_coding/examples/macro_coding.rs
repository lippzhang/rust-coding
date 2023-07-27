#![allow(unused)]
fn main() {
  println!("macro_coding start");
  // assert_fn();
  // assert_eq_fn();
  // assert_ne_fn();
  // concat_fn();
  // env_fn();
  let this_file = file!();
// println!("defined in file: {}", this_file);
// =======================
  format!("test");
  format!("hello {}", "world!");
  let a = format!("x = {}, y = {y}", 10, y = 30);
  println!("{}",a);
// =======================
  let my_string = include!("monkeys.in");
  assert_eq!("🙈🙊🙉🙈🙊🙉", my_string);
  println!("{}", my_string);
// =======================
  let my_string2 = include_str!("spanish.in");
  assert_eq!(my_string2, "lippzhang66");
  println!("{}", my_string2);
// =======================
  let foo = 'f';
  assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

  let bar = Some(4);
  assert!(matches!(bar, Some(x) if x > 2));
// =======================

  println!("macro_coding end");

}

fn env_fn(){
  let path: &'static str = env!("PATH");
  println!("the $PATH variable at the time of compiling was: {}", path);
}
fn concat_fn() {
  let s = concat!("test", 10, 'b', true);
  assert_eq!(s, "test10btrue");
  // let doc: &'static str = env!("ss", "what's that?");
}

fn assert_fn(){
 // 这些断言的 panic 消息是给定表达式的字符串化值。
 assert!(true);

 fn some_computation() -> bool { true } // 一个非常简单的函数

 assert!(some_computation());

 // 使用自定义消息进行断言
//  let x = true;
 let x = false;
 assert!(x, "x wasn't true!");

 let a = 3; let b = 27;
 assert!(a + b == 30, "a = {}, b = {}", a, b);
 // assert!(a + b == 31, "a = {}, b = {}", a, b);

}

fn assert_eq_fn() {
  let a = 3;
  let b = 1 + 2;
  assert_eq!(a,b);
  assert_eq!(a,b,"we are testing addition with {} and {}", a,b)
}
fn assert_ne_fn() {
  let a = 3;
  let b = 2;
  assert_ne!(a, b);

  assert_ne!(a, b, "we are testing that the values are not equal");
}

