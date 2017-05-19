extern crate bit_vec;
mod primes;
mod collect;

fn p12() {
  let sieve = primes::Sieve::new(100000);
  println!("constructed sieve");
  let mut max_divisors = 0;
  for n in 1..100000 {
    let (a, b) = if n % 2 == 0 { (n/2, n+1) } else { (n, (n + 1) / 2) };
    let mut pf = sieve.factorize(a);
    pf.extend(sieve.factorize(b));
    let num_divisors = collect::count_map(&pf).values().map(|x| x+1).product::<i64>();
    if num_divisors > max_divisors {
      println!("{} has {} divisors", (n * (n + 1) / 2), num_divisors);
      max_divisors = num_divisors;
      if num_divisors > 500 { break; }
    }
  }
}

fn main() {
  p12();
}


