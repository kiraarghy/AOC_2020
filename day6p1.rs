fn main() {
  fn get_score(input: &str) -> u64 {
    let mut score: u64 = 0;
    let lines: Vec<&str> = input.split("\n\n").collect();
    for str in lines {
      let mut current_str = str.trim().chars().collect::<Vec<char>>();
      current_str.sort_unstable();
      current_str.dedup();
      // println!("{} next", current_str.iter().collect::<String>());
      score = score + current_str.iter().collect::<String>().trim().len() as u64;
    }
    return score;
  };
  assert_eq!(
    get_score(
      "abc

a
b
c

ab
ac

a
a
a
a

b"
    ),
    11
  )
}
