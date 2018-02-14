#![feature(rustc_private)]
extern crate rand;
extern crate rls_data as data;
extern crate rustc_serialize;

pub mod warmup;
pub mod ana;

fn main() {
  // fizzbuzz(20);
  // guess_number();
  warmup::philosopher_eat();
  println!("\n\n\n");
  ana::exec();
}
