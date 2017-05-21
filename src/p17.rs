


pub fn solve() {
  let mut count = 0;
  for i in 1..1001 {
    let mut buf = String::new();
    to_text(i, &mut buf);
    count += buf.len();
  }
  println!("answer: {}", count);
}



fn to_text<'a>(x: i32, buf: &mut String) {
  match x {
    1 => buf.push_str("one"),
    2 => buf.push_str("two"),
    3 => buf.push_str("three"),
    4 => buf.push_str("four"),
    5 => buf.push_str("five"),
    6 => buf.push_str("six"),
    7 => buf.push_str("seven"),
    8 => buf.push_str("eight"),
    9 => buf.push_str("nine"),
    10 => buf.push_str("ten"),
    11 => buf.push_str("eleven"),
    12 => buf.push_str("twelve"),
    13 => buf.push_str("thirteen"),
    14 => buf.push_str("fourteen"),
    15 => buf.push_str("fifteen"),
    16 => buf.push_str("sixteen"),
    17 => buf.push_str("seventeen"),
    18 => buf.push_str("eighteen"),
    19 => buf.push_str("nineteen"),
    20 => buf.push_str("twenty"),
    30 => buf.push_str("thirty"),
    40 => buf.push_str("forty"),
    50 => buf.push_str("fifty"),
    60 => buf.push_str("sixty"),
    70 => buf.push_str("seventy"),
    80 => buf.push_str("eighty"),
    90 => buf.push_str("ninety"),
    21...99 => {
      to_text((x / 10) * 10, buf);
      to_text(x % 10, buf);
    },
    100...999 => {
      to_text(x / 100, buf);
      buf.push_str("hundred");
      if x % 100 > 0 {
        buf.push_str("and");
        to_text(x % 100, buf);
      }
    }
    1000 => buf.push_str("onethousand"),
    _ => assert!(false)
  }
  buf.trim();
}


