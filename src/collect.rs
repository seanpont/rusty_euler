use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub fn count_map<T: Hash + Eq>(xs: &Vec<T>) -> HashMap<&T, i64> {
  let mut counts = HashMap::new();
  for x in xs {
    let count = counts.entry(x).or_insert(0);
    *count += 1;
  }
  counts
}

#[cfg(test)]
mod tests {
  use collect;
  #[test]
  fn test_count_map() {
    let letters = vec!['a', 'a', 'b', 'c', 'c', 'c'];
    let count = collect::count_map(&letters);
    assert_eq!(count[&'a'], 2);
    assert_eq!(count[&'b'], 1);
    assert_eq!(count[&'c'], 3);
    assert_eq!(count.values().product::<i64>(), 6);
    assert_eq!(count.values().map(|x| x + 1).product::<i64>(), 24);
  }
}

