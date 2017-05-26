use primes;

/// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide
/// evenly into n).
/// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of
/// a and b are called amicable numbers.
///
/// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
/// therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142;
/// so d(284) = 220.
///
/// Evaluate the sum of all the amicable numbers under 10000.
pub fn solve() {
  let sieve = primes::Sieve::new(100000);
  let mut pairs = Vec::new();
  for i in 1..10000 {
    let j = sieve.divisors(i).iter().sum();
    let k = sieve.divisors(j).iter().sum();
    if i != j && i == k && !pairs.contains(&i) {
      pairs.push(i);
      pairs.push(j);
    }
  }
  println!("{:?}", pairs);
  println!("answer: {:?}", pairs.into_iter().sum::<u64>());
}












