fn main() {
  fn higher_or_lower_bound(input: (u64, u64), char: char) -> (u64, u64) {
    let higherbound = (((input.1 - input.0) as f64) / 2.0).round() as u64;
    if char == 'F' || char == 'L' {
      return (input.0, input.1 - higherbound);
    } else {
      return (input.0 + higherbound, input.1);
    }
  }

  assert_eq!(higher_or_lower_bound((0, 127), 'F'), (0, 63));
  assert_eq!(higher_or_lower_bound((0, 63), 'B'), (32, 63));
  assert_eq!(higher_or_lower_bound((32, 47), 'B'), (40, 47));
  assert_eq!(higher_or_lower_bound((0, 7), 'R'), (4, 7));
  assert_eq!(higher_or_lower_bound((4, 7), 'L'), (4, 5));
  assert_eq!(higher_or_lower_bound((4, 5), 'R'), (5, 5));

  fn parse_boarding_pass(input: &str) -> (&str, &str) {
    let row = &input[..7];
    let column = &input[7..];
    return (row, column);
  }
  assert_eq!(parse_boarding_pass("BFFFBBFRRR"), ("BFFFBBF", "RRR"));

  fn get_row_number(input: &str) -> u64 {
    let mut row_bound = (0, 127);
    for c in input.chars() {
      row_bound = higher_or_lower_bound(row_bound, c)
    }
    return row_bound.0;
  }
  fn get_column_number(input: &str) -> u64 {
    let mut column_bound = (0, 7);
    for c in input.chars() {
      column_bound = higher_or_lower_bound(column_bound, c)
    }
    return column_bound.0;
  }
  fn get_seat_id(input: &str) -> u64 {
    let parsed_boarding_pass = parse_boarding_pass(input);
    return get_row_number(parsed_boarding_pass.0) * 8 + get_column_number(parsed_boarding_pass.1);
  }
  assert_eq!(get_row_number(parse_boarding_pass("BFFFBBFRRR").0), 70);
  assert_eq!(get_column_number(parse_boarding_pass("BFFFBBFRRR").1), 7);
  assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
  assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
  assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
  fn ids(input: &str) -> Vec<u64> {
    let parsed_input: Vec<&str> = input.lines().collect();
    let mut ids: Vec<u64> = Vec::new();
    for string in parsed_input.iter() {
      ids.push(get_seat_id(string));
    }
    ids.sort();
    return ids;
  }
  fn find_missing_id(input: &[u64]) -> u64 {
    let mut missing_id: u64 = 0;
    let mut iter = input.iter().peekable();
    for &int in input.iter() {
      iter.next();
      if let Some(x) = iter.peek() {
        if *x - &int > 1 {
          missing_id = int + 1
        }
      };
    }
    return missing_id;
  }
}
