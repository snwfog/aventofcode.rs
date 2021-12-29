use std::fs;
use std::ops::Index;
use std::str::FromStr;

pub fn part1() {
  let commands = fs::read_to_string("input/aoc2021/aoc2021_02.txt")
    .unwrap()
    .trim()
    .split("\n")
    .map(|s| s.parse::<Command>().unwrap())
    .collect::<Vec<Command>>();

  let mut pos = (0, 0);
  for command in commands {
    match command.direction {
      Direction::Forward => pos.0 += command.step,
      Direction::Up => pos.1 -= command.step,
      Direction::Down => pos.1 += command.step,
    }
  }

  println!("{:?}, mut: {}", pos, pos.1 * pos.0);
}

pub fn part2() {
  let commands = fs::read_to_string("input/aoc2021/aoc2021_02.txt")
    .unwrap()
    .trim()
    .split("\n")
    .map(|s| s.parse::<Command>().unwrap())
    .collect::<Vec<Command>>();

  let mut aim = 0;
  let mut pos = (0, 0);
  for cmd in commands {
    match cmd.direction {
      Direction::Forward => {
        pos.0 += cmd.step;
        pos.1 += aim * cmd.step;
      }
      Direction::Up => aim -= cmd.step,
      Direction::Down => aim += cmd.step,
    }
  }

  println!("{:?}, mut: {}", pos, pos.1 * pos.0);
}

#[derive(Debug, PartialEq)]
enum Direction {
  Up,
  Down,
  Forward,
}

#[derive(Debug, PartialEq)]
struct Command {
  step:      i32,
  direction: Direction,
}

impl FromStr for Command {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let commands = s.split(" ").collect::<Vec<&str>>();
    let (command, n) = (commands[0], commands[1]);
    let direction = match command {
      "forward" => Direction::Forward,
      "up" => Direction::Up,
      "down" => Direction::Down,
      _ => panic!("Unknown direction"),
    };

    Ok(Command {
      step: n.parse::<i32>().unwrap(),
      direction,
    })
  }
}
