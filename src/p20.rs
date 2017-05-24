use num::BigUint;
use num::One;
use num::FromPrimitive;
use utils;

pub fn solve() {
  let x = (1..101)
      .map(|n| BigUint::from_u32(n).unwrap())
      .fold(BigUint::one(), |acc, x| acc * x);
  let ans = utils::sum_digits(x);
  println!("p20 answer: {}", ans);
}











