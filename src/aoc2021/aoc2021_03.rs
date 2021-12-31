use std::fs;

pub fn part1() {
  let contents = fs::read_to_string("input/aoc2021/aoc2021_03.txt").unwrap();

  let mut bits = [0; 12];
  let mut size = 0;
  for n in contents.trim().split("\n") {
    size += 1;
    let mut pos = 0;
    for c in n.chars() {
      if c == '1' {
        bits[pos] += 1;
      }

      pos += 1
    }
  }

  println!("{:?}, total: {}", bits, size);

  let mut a = 0;
  let mut b = 0;
  for i in 0..12 {
    if size - bits[i] == bits[i] {
      panic!("equal size")
    }

    if bits[i] > size - bits[i] {
      a |= (1 << (12 - i - 1))
    } else {
      b |= (1 << (12 - i - 1))
    }
  }

  println!("{:b} {:b} {}", a, b, a * b);
}

pub fn part2() {
  oxygen();
  co2();
}

fn oxygen() {
  let contents = fs::read_to_string("input/aoc2021/aoc2021_03.txt").unwrap();
  let mut numbers = contents.trim().split("\n").collect::<Vec<&str>>();
  numbers.sort();

  for n in 0usize..12 {
    println!("numbers {}", numbers.len());

    if numbers.len() == 1 {
      break;
    }

    // position of the last 0
    let pivot = bsearch(&numbers, n);
    let (head, tail) = numbers.split_at(pivot);
    println!(
      "head: {}, tail: {}, head: {:?}, tail: {:?}",
      head.len(),
      tail.len(),
      head,
      tail
    );

    if head.len() > tail.len() {
      numbers = head.to_vec()
    } else {
      numbers = tail.to_vec()
    }
  }

  if numbers.len() == 1 {
    println!(
      "found oxygen {:?}",
      i32::from_str_radix(numbers[0], 2).unwrap()
    );
  }
}

fn co2() {
  let contents = fs::read_to_string("input/aoc2021/aoc2021_03.txt").unwrap();
  let mut numbers = contents.trim().split("\n").collect::<Vec<&str>>();
  numbers.sort();

  for n in 0usize..12 {
    println!("numbers {}", numbers.len());

    if numbers.len() == 1 {
      break;
    }

    // position of the last 0
    let pivot = bsearch(&numbers, n);
    let (head, tail) = numbers.split_at(pivot);
    println!("head: {}, tail: {}", head.len(), tail.len());

    if tail.len() < head.len() {
      numbers = tail.to_vec()
    } else {
      numbers = head.to_vec()
    }
  }

  if numbers.len() == 1 {
    println!(
      "found co2 {:?}",
      i32::from_str_radix(numbers[0], 2).unwrap()
    );
  }
}

fn bsearch(numbers: &[&str], n: usize) -> usize {
  let (mut i, mut j) = (0, numbers.len());

  while i <= j {
    let mid = (i + j) >> 1;
    let val = numbers[mid].as_bytes()[n];

    if val == '1' as u8 {
      if mid == 0 {
        return mid;
      }

      j = mid - 1;
    } else {
      if mid == numbers.len() - 1 {
        return mid;
      }

      i = mid + 1;
    }
  }

  i
}
