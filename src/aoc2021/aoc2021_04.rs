use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

pub fn part1() {
  let mut boards = parse_boards().unwrap();
  let numbers = fs::read_to_string("input/aoc2021/aoc2021_04a.txt")
    .expect("Failed to read input file")
    .split(",")
    .map(|n| n.trim().parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  for n in &numbers {
    for b in &mut boards {
      if b.try_hit(&n) {
        if b.win() {
          println!("winning number: {}", n);
          println!("{}", b);
          println!("get number {}", b.get_number());

          // for b in boards {
          //   println!("board");
          //   println!("{}", b);
          // }
          return;
        }
      }
    }
  }
}

pub fn part2() {
  let mut boards = parse_boards().unwrap();
  let numbers = fs::read_to_string("input/aoc2021/aoc2021_04a.txt")
    .expect("Failed to read input file")
    .split(",")
    .map(|n| n.trim().parse::<i32>().unwrap())
    .collect::<Vec<i32>>();

  for n in numbers {
    for b in &mut boards {
      if b.win() {
        continue; // skip win board
      }

      if b.try_hit(&n) {
        if b.win() {
          println!("winning number: {}", n);
          println!("{}", b);
          println!("get number {}", b.get_number());
        }
      }
    }
  }
}

fn parse_boards() -> Result<Vec<Board>, Box<dyn Error>> {
  let file = File::open("input/aoc2021/aoc2021_04b.txt")?;
  let mut boards = vec![];
  let mut i = 0;
  for l in io::BufReader::new(file).lines() {
    let line = l?;
    if line.len() == 0 {
      i = 0;
      continue;
    }

    if i == 0 {
      boards.push(Board { board: vec![], numbers: HashSet::new() });
    }

    // println!("line: {}", line);

    let row = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    boards.last_mut().unwrap().numbers.extend(row.iter().cloned());
    boards.last_mut().unwrap().board.push(row);
    i += 1;
  }

  // println!("{:?}", boards);

  Ok(boards)
}

#[derive(Debug)]
struct Board {
  board: Vec<Vec<i32>>,
  numbers: HashSet<i32>,
}

impl Board {
  fn get_number(&self) -> i32 {
    let mut n = 0;
    for row in &self.board {
      for c in row {
        if *c < 100 {
          n += c;
        }
      }
    }

    n
  }

  fn try_hit(&mut self, n: &i32) -> bool {
    if self.numbers.contains(n) {
      for row in &mut self.board {
        row.iter().position(|x| x == n).map(|i| row[i] = 100 + n);
        // println!("{:?}", row);
      }
      return true;
    }

    false
  }

  fn win(&self) -> bool {
    if self.board[0][0] >= 100 && self.board[0][1] >= 100 && self.board[0][2] >= 100 && self.board[0][3] >= 100 && self.board[0][4] >= 100 {
      return true;
    }

    if self.board[1][0] >= 100 && self.board[1][1] >= 100 && self.board[1][2] >= 100 && self.board[1][3] >= 100 && self.board[1][4] >= 100 {
      return true;
    }

    if self.board[2][0] >= 100 && self.board[2][1] >= 100 && self.board[2][2] >= 100 && self.board[2][3] >= 100 && self.board[2][4] >= 100 {
      return true;
    }

    if self.board[3][0] >= 100 && self.board[3][1] >= 100 && self.board[3][2] >= 100 && self.board[3][3] >= 100 && self.board[3][4] >= 100 {
      return true;
    }

    if self.board[4][0] >= 100 && self.board[4][1] >= 100 && self.board[4][2] >= 100 && self.board[4][3] >= 100 && self.board[4][4] >= 100 {
      return true;
    }

    if self.board[0][0] >= 100 && self.board[1][0] >= 100 && self.board[2][0] >= 100 && self.board[3][0] >= 100 && self.board[4][0] >= 100 {
      return true;
    }

    if self.board[0][1] >= 100 && self.board[1][1] >= 100 && self.board[2][1] >= 100 && self.board[3][1] >= 100 && self.board[4][1] >= 100 {
      return true;
    }

    if self.board[0][2] >= 100 && self.board[1][2] >= 100 && self.board[2][2] >= 100 && self.board[3][2] >= 100 && self.board[4][2] >= 100 {
      return true;
    }

    if self.board[0][3] >= 100 && self.board[1][3] >= 100 && self.board[2][3] >= 100 && self.board[3][3] >= 100 && self.board[4][3] >= 100 {
      return true;
    }

    if self.board[0][4] >= 100 && self.board[1][4] >= 100 && self.board[2][4] >= 100 && self.board[3][4] >= 100 && self.board[4][4] >= 100 {
      return true;
    }

    false
  }
}


impl Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    for row in &self.board {
      writeln!(f, "{:?}", row);
    }

    Ok(())
  }
}
