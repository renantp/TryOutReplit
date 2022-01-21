// use rand::Rng;
use std::io;
fn main() {
  let mut buf = String::new();
  println!("Name me: ");
  io::stdin().read_line(&mut buf).expect("Failed to read line");
  println!("Hello, world! My name is {}", buf);
}