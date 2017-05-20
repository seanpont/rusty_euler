#![allow(dead_code)]
extern crate bit_vec;
extern crate num;
#[macro_use]
extern crate log;

mod primes;
mod collect;
mod p14;

fn main() {
  p14::solve();
}


