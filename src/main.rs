#![allow(dead_code)]
extern crate bit_vec;
extern crate num;
#[macro_use]
extern crate log;

mod primes;
mod collect;
mod utils;
mod p12;
mod p13;
mod p14;
mod p15;
mod p16;
mod p17;
mod p18;
mod p19;
mod p20;
mod p21;

fn main() {
  p21::solve();
}


