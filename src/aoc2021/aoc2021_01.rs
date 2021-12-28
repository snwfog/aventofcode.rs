use std::fs;

pub fn part1() {
  let content =
    fs::read_to_string("input/aoc2021/aoc2021_01.txt").expect("Failed to read input file");
  let numbers: Vec<i32> = content
    .trim()
    .split("\n")
    .map(|s| s.parse().unwrap())
    .collect();

  let mut inc = 0;
  for n in 1..numbers.len() {
    if numbers[n] > numbers[n - 1] {
      inc += 1;
    }
  }

  println!("{}", inc);
}

pub fn part2() {
  let content =
    fs::read_to_string("input/aoc2021/aoc2021_01.txt").expect("Failed to read input file");
  let numbers: Vec<i32> = content
    .trim()
    .split("\n")
    .map(|s| s.parse().unwrap())
    .collect();

  let mut inc = 0;
  for n in 3..numbers.len() {
    let a = numbers[n] + numbers[n - 1] + numbers[n - 2];
    let b = numbers[n - 1] + numbers[n - 2] + numbers[n - 3];
    if a > b {
      inc += 1;
    }
  }

  println!("{}", inc);
}
