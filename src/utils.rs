use num::BigUint;
use num::FromPrimitive;
use num::Zero;
use num::Integer;

/// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
///
/// What is the sum of the digits of the number 2^1000?
pub fn sum_digits(mut x: BigUint) -> BigUint {
  let ten = BigUint::from_u32(10).unwrap();
  let mut ans = BigUint::zero();
  while x > BigUint::zero() {
    let (q, r) = BigUint::div_mod_floor(&x, &ten);
    ans = ans + r;
    x = q;
  }
  ans
}