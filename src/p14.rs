/// The following iterative sequence is defined for the set of positive integers:
///
/// n → n/2 (n is even)
/// n → 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following sequence:
///
/// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
/// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
/// Although it has not been proved yet (Collatz Problem), it is thought that all starting
/// numbers finish at 1.
///
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.
pub fn solve() {
  let answer = (1..1_000_000)
      .map(|n| (n, collatz_length(n)))
      .max_by_key(|x| x.1)
      .expect("");
  println!("answer: {:?}", answer)
}

fn collatz_length(mut n: i64) -> i64 {
  let mut count = 1;
  while n != 1 {
    n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
    count += 1
  }
  count
}
