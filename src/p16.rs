use num::BigUint;
use num::pow;
use num::FromPrimitive;
use utils;

/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
///
/// What is the sum of the digits of the number 2^1000?
pub fn solve() {
  let ans = utils::sum_digits(pow(BigUint::from_u32(2).expect("pow"), 1000));
  println!("answer: {}", ans);
}