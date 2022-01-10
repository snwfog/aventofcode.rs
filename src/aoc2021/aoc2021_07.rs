use std::collections::HashMap;
use std::fs;

pub fn part1() {
  let numbers = fs::read_to_string("input/aoc2021/input07.txt")
    .expect("Failed to read input file.")
    .trim()
    .split(",")
    .map(|x| x.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  // let min = positions.iter().min().unwrap();
  // let max = positions.iter().max().unwrap();

  let mut positions = [0; 2000];
  for n in numbers {
    positions[n] += 1;
  }

  let mut pivot: usize = 2000;
  let mut moves = i64::MAX;

  for p in 0..2000 {
    // calculate moves to align
    let mut moves_to_pivot = 0;
    for i in 0..2000 {
      if i == p { // nothing to do, cause items is on the pivot
        continue;
      } else if i < p {
        moves_to_pivot += (p as i64 - i as i64) * positions[i];
      } else {
        moves_to_pivot += (i as i64 - p as i64) * positions[i];
      }
    }

    if moves_to_pivot < moves {
      moves = moves_to_pivot;
      pivot = p;
    }
  }

  println!("moves: {}, location: {}", moves, pivot);
}

pub fn part2() {
  let numbers = fs::read_to_string("input/aoc2021/input07.txt")
    .expect("Failed to read input file.")
    .trim()
    .split(",")
    .map(|x| x.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  // let min = positions.iter().min().unwrap();
  // let max = positions.iter().max().unwrap();

  let mut positions = [0; 2000];
  for n in numbers {
    positions[n] += 1;
  }

  let mut pivot: usize = 2000;
  let mut moves = i64::MAX;

  for p in 0..2000 {
    // calculate moves to align
    let mut moves_to_pivot = 0;
    for i in 0..2000 {
      if i == p { // nothing to do, cause items is on the pivot
        continue;
      } else if i < p {
        let mm = (p as i64 - i as i64);
        moves_to_pivot += mm * (mm + 1) / 2 * positions[i];
      } else {
        let mm = (i as i64 - p as i64);
        moves_to_pivot += mm * (mm + 1) / 2 * positions[i];
      }
    }

    if moves_to_pivot < moves {
      moves = moves_to_pivot;
      pivot = p;
    }
  }

  println!("moves: {}, location: {}", moves, pivot);
}
