use std::fs;

pub fn part1() {
  let coordinates = fs::read_to_string("input/aoc2021/input05.txt")
    .expect("Failed to read input")
    .trim()
    .lines()
    .map(|line| {
      let c = line
        .split(" -> ")
        .map(|pair| {
          let n = pair
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

          (n[0], n[1])
        })
        .collect::<Vec<(usize, usize)>>();

      (c[0], c[1]) // from coordinate -> to coordinate
    }).collect::<Vec<((usize, usize), (usize, usize))>>();

  let mut map = [[0; 1000]; 1000];
  for ((mut x1, mut y1), (mut x2, mut y2)) in coordinates {
    if x1 != x2 && y1 != y2 {
      println!("skip diagonals {:?} -> {:?}", (x1, y1), (x2, y2));
    }

    if y1 == y2 {
      if x2 < x1 {
        std::mem::swap(&mut x1, &mut x2);
      }

      for x in x1..=x2 {
        map[y1][x] += 1;
      }
    }

    if x1 == x2 {
      if y2 < y1 {
        std::mem::swap(&mut y1, &mut y2);
      }

      for y in y1..=y2 {
        map[y][x1] += 1;
      }
    }
  }

  let mut count = 0;
  for x in 0..1000 {
    for y in 0..1000 {
      if map[x][y] >= 2 {
        count += 1
      }
    }
  }

  println!("count: {}", count);
}

pub fn part2() {
  let coordinates = fs::read_to_string("input/aoc2021/input05.txt")
    .expect("Failed to read input")
    .trim()
    .lines()
    .map(|line| {
      let c = line
        .split(" -> ")
        .map(|pair| {
          let n = pair
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

          (n[0], n[1])
        })
        .collect::<Vec<(usize, usize)>>();

      (c[0], c[1]) // from coordinate -> to coordinate
    }).collect::<Vec<((usize, usize), (usize, usize))>>();

  let mut map = [[0; 1000]; 1000];

  for ((mut x1, mut y1), (mut x2, mut y2)) in coordinates {
    if y1 == y2 {
      if x2 < x1 {
        std::mem::swap(&mut x1, &mut x2);
      }

      for x in x1..=x2 {
        map[y1][x] += 1;
      }
    } else if x1 == x2 {
      if y2 < y1 {
        std::mem::swap(&mut y1, &mut y2);
      }

      for y in y1..=y2 {
        map[y][x1] += 1;
      }
    } else if (x1 as i32 - x2 as i32).abs() == (y1 as i32 - y2 as i32).abs() {
      println!("diagonals {:?} -> {:?}", (x1, y1), (x2, y2));
      let rx: Vec<usize> = if x1 <= x2 {
        (x1..=x2).collect()
      } else {
        (x2..=x1).rev().collect()
      };

      let ry: Vec<usize> = if y1 <= y2 {
        (y1..=y2).collect()
      } else {
        (y2..=y1).rev().collect()
      };


      for (&x, &y) in rx.iter().zip(ry.iter()) {
        map[y][x] += 1;
      }
    } else {
      println!("skip not 45 degree diagonals {:?} -> {:?}", (x1, y1), (x2, y2));
    }
  }

  let mut count = 0;
  for x in 0..1000 {
    for y in 0..1000 {
      if map[x][y] >= 2 {
        count += 1
      }
    }
  }

  println!("count: {}", count);
}
