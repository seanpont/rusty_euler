use num::BigUint;
use num::pow;
use num::FromPrimitive;
use num::Zero;
use num::Integer;

/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
///
/// What is the sum of the digits of the number 2^1000?
pub fn solve() {
  let mut x = pow(BigUint::from_u32(2).expect("pow"), 1000);
  let ten = BigUint::from_u32(10).expect("ten");
  let mut ans = BigUint::zero();
  while x > BigUint::zero() {
    let (q, r) = BigUint::div_mod_floor(&x, &ten);
    ans = ans + r;
    x = q;
  }
  println!("answer: {}", ans);
}