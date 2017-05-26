use bit_vec::BitVec;

pub struct Sieve(BitVec);

impl Sieve {
  pub fn new(size: usize) -> Sieve {
    assert!(size > 0);
    let mut s = BitVec::from_elem(size as usize, true);
    s.set(0, false);
    s.set(1, false);
    for i in 2..((size as f32).powf(0.5) as usize + 1) {
      if s.get(i as usize).expect("wha?") {
        let mut j = i * i;
        while j < size {
          s.set(j, false);
          j += i;
        }
      }
    }
    Sieve(s)
  }

  pub fn iter(&self) -> Iter {
    Iter { sieve: self, idx: 0, end_idx: self.0.len() }
  }

  pub fn factorize(&self, mut x: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for p in self {
      let p = p as u64;
      while x % p == 0 {
        factors.push(p);
        x /= p;
      }
      if x == 1 { break; }
    }
    assert_eq!(x, 1, "sieve too small!");
    factors
  }

  pub fn divisors(&self, x: u64) -> Vec<u64> {
    if x == 0 || x == 1 {
      return vec![x];
    }
    let factors = self.factorize(x);
    let mut divisors = Vec::with_capacity(2 << factors.len());
    divisors.push(1);
    divisors.push(factors[0]);
    let mut in_a_row=1;
    for i in 1..factors.len() {
      let p = factors[i];
      in_a_row = if factors[i] == factors[i - 1] { in_a_row + 1} else { 1 };
      let d_len = divisors.len();
      for j in d_len * (in_a_row-1) / in_a_row.. d_len {
        let d = divisors[j] * p;
        divisors.push(d);
      }
    }
    divisors.pop();
    divisors.sort();
    divisors
  }
}

pub struct Iter<'a> {
  sieve: &'a Sieve,
  idx: usize,
  end_idx: usize,
}

impl<'a> Iterator for Iter<'a> {
  type Item = usize;

  #[inline]
  fn next(&mut self) -> Option<usize> {
    loop {
      self.idx += 1;
      if self.idx == self.end_idx { return None; }
      if self.sieve.0[self.idx] { return Some(self.idx); }
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let rem = self.end_idx - self.idx;
    (rem, Some(rem))
  }
}

impl<'a> IntoIterator for &'a Sieve {
  type Item = usize;
  type IntoIter = Iter<'a>;

  fn into_iter(self) -> Iter<'a> {
    self.iter()
  }
}

#[cfg(test)]
mod tests {
  use primes;

  #[test]
  fn test_primes() {
    let sieve = primes::Sieve::new(12);
    let p = sieve.iter().collect::<Vec<usize>>();
    assert_eq!(p, vec![2, 3, 5, 7, 11]);

    let factors = sieve.factorize(84);
    assert_eq!(factors, vec![2, 2, 3, 7]);

    let factors = sieve.factorize(2);
    assert_eq!(factors, vec![2]);

    let divisors = sieve.divisors(220);
    assert_eq!(divisors, vec![1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110]);

    let divisors = sieve.divisors(54);
    assert_eq!(divisors, vec![1, 2, 3, 6, 9, 18, 27]);

    let divisors = sieve.divisors(2);
    assert_eq!(divisors, vec![1]);

    let divisors = sieve.divisors(1);
    assert_eq!(divisors, vec![1]);

    let sieve = primes::Sieve::new(1000);
    for i in 2..1000 {
      let mut d = Vec::new();
      for j in 1..i {
        if i % j == 0 { d.push(j) }
      }
      assert_eq!(sieve.divisors(i), d);
    }
  }
}

