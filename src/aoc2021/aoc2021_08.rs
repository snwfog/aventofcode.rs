use std::collections::HashMap;
use std::fs;

pub fn part1() {
  let entries = fs::read_to_string("input/aoc2021/input08.txt")
    .expect("Failed to read input");

  let mut total = 0;
  for line in entries.trim().split("\n") {
    let parts = line.split("|").collect::<Veuse std::collections::HashMap;
                                            use std::fs;

                                            pub fn part1() {
                                              let entries = fs::read_to_string("input/aoc2021/input08.txt")
                                                .expect("Failed to read input");

                                              let mut total = 0;
                                              for line in entries.trim().split("\n") {
                                                let parts = line.split("|").collect::<Vec<&str>>();
                                                let digits = parts[1].trim().split_whitespace().collect::<Vec<&str>>();
                                                for seg in digits {
                                                  if let Some(_) = parse_digit(seg) {
                                                    total += 1;
                                                    println!("{}", seg);
                                                  }
                                                }
                                              }

                                              println!("{}", total);
                                            }

                                            pub fn part2() {
                                              let entries = fs::read_to_string("input/aoc2021/input08.txt")
                                                .expect("Failed to read input");

                                              let mut total = 0;
                                              for line in entries.trim().split("\n") {
                                                total += decode(line);
                                              }

                                              pritnln("{}", total);
                                            }

                                            fn parse_digit(segs: &str) -> Option<u32> {
                                              match segs.len() {
                                                2 => Some(1),
                                                4 => Some(4),
                                                3 => Some(7),
                                                7 => Some(8),
                                                _ => None,
                                              }
                                            }

                                            fn decode(line: &str) -> u32 {
                                              let parts = line.split("|").collect::<Vec<&str>>();
                                            }

                                            struct Digit(u8, u8, u8, u8, u8, u8, u8);

                                            const ZERO: Digit = Digit(1, 2, 3, 0, 5, 6, 7);
                                            const ONE: Digit = Digit(0, 0, 3, 0, 0, 6, 0);
                                            const TWO: Digit = Digit(1, 0, 3, 4, 5, 0, 7);
                                            const THREE: Digit = Digit(1, 0, 3, 4, 0, 6, 7);
                                            const FOUR: Digit = Digit(0, 2, 3, 4, 0, 6, 0);
                                            const FIVE: Digit = Digit(1, 2, 0, 4, 0, 6, 7);
                                            const SIX: Digit = Digit(1, 2, 0, 4, 5, 6, 7);
                                            const SEVEN: Digit = Digit(1, 0, 3, 0, 0, 6, 0);
                                            const EIGHT: Digit = Digit(1, 2, 3, 4, 5, 6, 7);
                                            const NINE: Digit = Digit(1, 2, 3, 4, 0, 6, 7);

                                            //  000
                                            // 1   2
                                            // 1   2
                                            //  333
                                            // 4   5
                                            // 4   5
                                            //  666

                                            struct DigitSet {
                                              id: Digit,
                                              digits: [Digit; 7],
                                            }
c<&str>>();
    let digits = parts[1].trim().split_whitespace().collect::<Vec<&str>>();
    for seg in digits {
      if let Some(_) = parse_digit(seg) {
        total += 1;
        println!("{}", seg);
      }
    }
  }

  println!("{}", total);
}

pub fn part2() {
  let entries = fs::read_to_string("input/aoc2021/input08.txt")
    .expect("Failed to read input");

  let mut total = 0;
  for line in entries.trim().split("\n") {
    total += decode(line);
  }

  pritnln("{}", total);
}

fn parse_digit(segs: &str) -> Option<u32> {
  match segs.len() {
    2 => Some(1),
    4 => Some(4),
    3 => Some(7),
    7 => Some(8),
    _ => None,
  }
}

fn decode(line: &str) -> u32 {
  let parts = line.split("|").collect::<Vec<&str>>();
}

struct Digit(u8, u8, u8, u8, u8, u8, u8);

const ZERO: Digit = Digit(1, 2, 3, 0, 5, 6, 7);
const ONE: Digit = Digit(0, 0, 3, 0, 0, 6, 0);
const TWO: Digit = Digit(1, 0, 3, 4, 5, 0, 7);
const THREE: Digit = Digit(1, 0, 3, 4, 0, 6, 7);
const FOUR: Digit = Digit(0, 2, 3, 4, 0, 6, 0);
const FIVE: Digit = Digit(1, 2, 0, 4, 0, 6, 7);
const SIX: Digit = Digit(1, 2, 0, 4, 5, 6, 7);
const SEVEN: Digit = Digit(1, 0, 3, 0, 0, 6, 0);
const EIGHT: Digit = Digit(1, 2, 3, 4, 5, 6, 7);
const NINE: Digit = Digit(1, 2, 3, 4, 0, 6, 7);

//  000
// 1   2
// 1   2
//  333
// 4   5
// 4   5
//  666

struct DigitSet {
  id: Digit,
  digits: [Digit; 7],
}
