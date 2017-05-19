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
    assert_eq!(x, 1);
    factors
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
    let sieve = primes::Sieve::new(10);
    let p = sieve.iter().collect::<Vec<usize>>();
    assert_eq!(p, vec![2, 3, 5, 7]);
    for p in &sieve {
      println!("{:?}", p);
    }
    let factors = sieve.factorize(84);
    assert_eq!(factors, vec![2, 2, 3, 7])
  }
}

