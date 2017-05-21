use std::collections::HashMap;

/// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and
/// down, there are exactly 6 routes to the bottom right corner.
/// How many such routes are there through a 20×20 grid?
pub fn solve() {
  let mut memo = HashMap::new();
  println!("{}", count_paths(20, 20, &mut memo));
}

fn count_paths(x: i64, y: i64, memo: &mut HashMap<(i64, i64), i64>) -> i64 {
  match (x, y) {
    (0, 0) => 0,
    (0, _) => 1,
    (_, 0) => 1,
    (_, _) => {
      if let Some(c) = memo.get(&(x, y)) { return *c }
      let c = count_paths(x-1, y, memo) + count_paths(x, y-1, memo);
      memo.insert((x, y), c);
      c
    }
  }
}