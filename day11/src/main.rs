const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<Vec<u8>> {
  input
    .split_whitespace()
    .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
    .collect()
}

fn solve1(input: &str) -> u32 {
  let mut octopuses = parse(input);
  let mut flashes = 0;
  let mut has_flashed: Vec<Vec<bool>> = octopuses
    .iter()
    .map(|row| row.iter().map(|_| false).collect())
    .collect();

  for _ in 0..100 {
    for row in &mut octopuses {
      for energy in row {
        *energy += 1;
      }
    }

    let mut prev_flashes = flashes;
    loop {
      for row in 0..octopuses.len() {
        for col in 0..octopuses[row].len() {
          if octopuses[row][col] > 9 {
            try_flash(&mut has_flashed, &mut octopuses, &mut flashes, (row, col));
          }
        }
      }

      if flashes == prev_flashes {
        break;
      }

      prev_flashes = flashes;
    }

    for row in 0..octopuses.len() {
      for col in 0..octopuses[row].len() {
        let octo = &mut octopuses[row][col];

        if *octo > 9 {
          *octo = 0;
        }

        has_flashed[row][col] = false;
      }
    }
  }

  flashes
}

fn solve2(input: &str) -> u32 {
  let mut octopuses = parse(input);
  let mut flashes = 0;
  let mut has_flashed: Vec<Vec<bool>> = octopuses
    .iter()
    .map(|row| row.iter().map(|_| false).collect())
    .collect();

  for step in 0.. {
    for row in &mut octopuses {
      for energy in row {
        *energy += 1;
      }
    }

    let mut prev_flashes = flashes;
    loop {
      for row in 0..octopuses.len() {
        for col in 0..octopuses[row].len() {
          if octopuses[row][col] > 9 {
            try_flash(&mut has_flashed, &mut octopuses, &mut flashes, (row, col));
          }
        }
      }

      if flashes == prev_flashes {
        break;
      }

      prev_flashes = flashes;
    }

    if octopuses
      .iter()
      .all(|row| row.iter().all(|&octo| octo == octopuses[0][0]))
    {
      return step;
    }

    for row in 0..octopuses.len() {
      for col in 0..octopuses[row].len() {
        let octo = &mut octopuses[row][col];

        if *octo > 9 {
          *octo = 0;
        }

        has_flashed[row][col] = false;
      }
    }
  }

  0
}

// Flash an octopus. This action will also increase its surroundings.
fn try_flash(
  has_flashed: &mut Vec<Vec<bool>>,
  octopuses: &mut Vec<Vec<u8>>,
  flashes: &mut u32,
  (row, col): (usize, usize),
) {
  if !has_flashed[row][col] {
    has_flashed[row][col] = true;
    *flashes += 1;
  } else {
    return;
  }

  for r in -1..=1 {
    for c in -1..=1 {
      if r == 0 && c == 0 {
        continue;
      }

      let r = (row as isize + r) as usize;
      let c = (col as isize + c) as usize;

      if let Some(v) = octopuses.get_mut(r).and_then(|row| row.get_mut(c)) {
        *v += 1;

        if *v > 9 {
          try_flash(has_flashed, octopuses, flashes, (r, c));
        }
      }
    }
  }
}

fn main() {
  println!("part1");
  println!("sample: {}", solve1(SAMPLE));
  println!("input: {}", solve1(INPUT));

  println!("part2");
  println!("sample: {}", solve2(SAMPLE));
  println!("input: {}", solve2(INPUT));
}
