/// You are given the following information, but you may prefer to do some research for yourself.
///
/// 1 Jan 1900 was a Monday.
/// Thirty days has September,
/// April, June and November.
/// All the rest have thirty-one,
/// Saving February alone,
/// Which has twenty-eight, rain or shine.
/// And on leap years, twenty-nine.
/// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is
/// divisible by 400.
/// How many Sundays fell on the first of the month during the twentieth century
/// (1 Jan 1901 to 31 Dec 2000)?


#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Month {
  January,
  February,
  March,
  April,
  May,
  June,
  July,
  August,
  September,
  October,
  November,
  December,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum DayOfWeek { Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday }

struct Calendar {
  year: i32,
  month: Month,
  day: i32,
  day_of_week: DayOfWeek,
  days_in_month: i32,
}

impl Calendar {
  pub fn new() -> Calendar {
    Calendar {
      year: 1900,
      month: Month::January,
      day: 1,
      day_of_week: DayOfWeek::Monday,
      days_in_month: Calendar::days_in_month(Month::January, 1900)
    }
  }

  fn days_in_month(month: Month, year: i32) -> i32 {
    match month {
      Month::January => 31,
      Month::February => if year % 4 == 0 && year % 400 != 0 { 29 } else { 28 },
      Month::March => 31,
      Month::April => 30,
      Month::May => 31,
      Month::June => 30,
      Month::July => 31,
      Month::August => 31,
      Month::September => 30,
      Month::October => 31,
      Month::November => 30,
      Month::December => 31,
    }
  }

  fn next(&mut self) {
    if self.day == self.days_in_month {
      self.month = match self.month {
        Month::January => Month::February,
        Month::February => Month::March,
        Month::March => Month::April,
        Month::April => Month::May,
        Month::May => Month::June,
        Month::June => Month::July,
        Month::July => Month::August,
        Month::August => Month::September,
        Month::September => Month::October,
        Month::October => Month::November,
        Month::November => Month::December,
        Month::December => {
          self.year += 1;
          Month::January
        },
      };
      self.day = 1;
      self.days_in_month = Calendar::days_in_month(self.month, self.year);
    } else {
      self.day += 1;
    }
    self.day_of_week = match self.day_of_week {
      DayOfWeek::Sunday => DayOfWeek::Monday,
      DayOfWeek::Monday => DayOfWeek::Tuesday,
      DayOfWeek::Tuesday => DayOfWeek::Wednesday,
      DayOfWeek::Wednesday => DayOfWeek::Thursday,
      DayOfWeek::Thursday => DayOfWeek::Friday,
      DayOfWeek::Friday => DayOfWeek::Saturday,
      DayOfWeek::Saturday => DayOfWeek::Sunday,
    }
  }
}

pub fn solve() {
  let mut answer = 1;
  let mut c = Calendar::new();

  while c.year < 2001 {
    c.next();
    // How many Sundays fell on the first of the month during the twentieth century
    if c.year >= 1901 && c.day_of_week == DayOfWeek::Sunday && c.day == 1 {
      answer += 1
    }
  }
  println!("answer: {}", answer);
}
