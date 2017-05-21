#![allow(dead_code)]
extern crate bit_vec;
extern crate num;
#[macro_use]
extern crate log;

mod primes;
mod collect;
mod p16;

fn main() {
  p16::solve();
}


