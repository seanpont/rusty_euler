#![allow(dead_code)]
extern crate bit_vec;
extern crate num;
#[macro_use]
extern crate log;

mod primes;
mod collect;
mod p15;

fn main() {
  p15::solve();
}


