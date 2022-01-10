use std::collections::HashMap;
use std::fs;

pub fn part1() {
  let numbers = fs::read_to_string("input/aoc2021/input06.txt")
    .expect("File not found")
    .trim()
    .split(",")
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  let mut m: HashMap<i32, i32> = HashMap::new();
  for n in numbers {
    m.entry(n).and_modify(|e| *e += 1).or_insert(1);
  }

  for day in 0..80 {
    let mut next_day: HashMap<i32, i32> = HashMap::new();
    for (k, v) in m.iter() {
      if *k == 0 {
        next_day.entry(8).and_modify(|e| *e += v).or_insert(*v); // spawn new
        next_day.entry(6).and_modify(|e| *e += v).or_insert(*v); // current
      } else {
        next_day.entry(k - 1).and_modify(|e| *e += v).or_insert(*v);
      }

      // println!("{} {}", k, v);

      if day == 79 {
        println!("{:?}, total: {}", next_day, next_day.values().sum::<i32>());
      }
    }

    m = next_day;
  }
}

pub fn part2() {
  let numbers = fs::read_to_string("input/aoc2021/input06.txt")
    .expect("File not found")
    .trim()
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();

  let mut m: HashMap<i64, i64> = HashMap::new();
  for n in numbers {
    m.entry(n).and_modify(|e| *e += 1).or_insert(1);
  }

  for day in 0..256 {
    let mut next_day: HashMap<i64, i64> = HashMap::new();
    for (k, v) in m.iter() {
      if *k == 0 {
        next_day.entry(8).and_modify(|e| *e += v).or_insert(*v); // spawn new
        next_day.entry(6).and_modify(|e| *e += v).or_insert(*v); // current
      } else {
        next_day.entry(k - 1).and_modify(|e| *e += v).or_insert(*v);
      }

      // println!("{} {}", k, v);

      if day == 255 {
        println!("{:?}, total: {}", next_day, next_day.values().sum::<i64>());
      }
    }

    m = next_day;
  }
}
