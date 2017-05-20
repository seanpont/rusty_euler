#![allow(dead_code)]
extern crate bit_vec;
extern crate num;
#[macro_use]
extern crate log;

mod primes;
mod collect;
mod p13;

fn main() {
  p13::solve();
}


