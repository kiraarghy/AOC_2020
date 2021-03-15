fn main() {
  fn intersect_of_ver<T: std::cmp::PartialEq + Clone>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let mut output: Vec<T> = vec![];
    for x in 0..vec1.len() {
      for y in 0..vec2.len() {
        if vec1[x] == vec2[y] {
          output.push(vec1[x].clone());
        }
      }
    }
    output
  }

  assert_eq!(intersect_of_ver(vec!['a', 'b'], vec!['a']), vec!['a']);

  fn get_score(input: &str) -> u64 {
    let mut score: u64 = 0;
    let groups: Vec<&str> = input.split("\n\n").collect();
    for str in groups {
      let people = str.lines();
      // could use a try_fold
      let string_vec_of_people = people.collect::<Vec<&str>>();
      score = score
        + string_vec_of_people
          .iter()
          .enumerate()
          .fold(vec![], |acc, (i, x)| {
            if i == 0 {
              return x.chars().collect();
            }
            if acc == vec![] {
              return acc;
            } else {
              return intersect_of_ver(acc, x.chars().collect());
            }
          })
          .iter()
          .collect::<String>()
          .len() as u64;
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
    6
  );
}
