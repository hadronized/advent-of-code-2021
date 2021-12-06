use std::collections::HashMap;

const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

type Point = [i32; 2];
type Line = (Point, Point);

fn parse(input: &str) -> Vec<Line> {
  input
    .split('\n')
    .filter(|l| !l.is_empty())
    .map(|l| {
      let points: Vec<_> = l
        .split(" -> ")
        .map(|parts| {
          let coords: Vec<_> = parts.split(',').flat_map(|c| c.parse::<i32>()).collect();
          [coords[0], coords[1]]
        })
        .collect();

      (points[0], points[1])
    })
    .collect()
}

fn rasterize(line: Line) -> Vec<Point> {
  if line.0[0] == line.1[0] {
    // x are the same so we change only y
    if line.0[1] < line.1[1] {
      (line.0[1]..=line.1[1])
        .into_iter()
        .map(|y| [line.0[0], y])
        .collect()
    } else {
      (line.1[1]..=line.0[1])
        .into_iter()
        .map(|y| [line.0[0], y])
        .collect()
    }
  } else if line.0[1] == line.1[1] {
    // y are the same so we only change x
    if line.0[0] < line.1[0] {
      (line.0[0]..=line.1[0])
        .into_iter()
        .map(|x| [x, line.0[1]])
        .collect()
    } else {
      (line.1[0]..=line.0[0])
        .into_iter()
        .map(|x| [x, line.0[1]])
        .collect()
    }
  } else {
    // diagonals
    if line.0[0] < line.1[0] {
      if line.0[1] < line.1[1] {
        (line.0[0]..=line.1[0])
          .into_iter()
          .zip(line.0[1]..=line.1[1])
          .map(|(x, y)| [x, y])
          .collect()
      } else {
        (line.0[0]..=line.1[0])
          .into_iter()
          .zip((line.1[1]..=line.0[1]).into_iter().rev())
          .map(|(x, y)| [x, y])
          .collect()
      }
    } else {
      if line.0[1] < line.1[1] {
        (line.1[0]..=line.0[0])
          .into_iter()
          .rev()
          .zip(line.0[1]..=line.1[1])
          .map(|(x, y)| [x, y])
          .collect()
      } else {
        (line.1[0]..=line.0[0])
          .into_iter()
          .rev()
          .zip((line.1[1]..=line.0[1]).into_iter().rev())
          .map(|(x, y)| [x, y])
          .collect()
      }
    }
  }
}

fn solve1(input: &str) -> u32 {
  let mut lines = parse(input);
  lines.retain(|l| l.0[0] == l.1[0] || l.0[1] == l.1[1]);

  let mut overlaps: HashMap<Point, u32> = HashMap::new();

  for line in lines {
    let points = rasterize(line);

    for p in points {
      *overlaps.entry(p).or_default() += 1;
    }
  }

  overlaps.iter().filter(|(_, &counts)| counts > 1).count() as _
}

fn solve2(input: &str) -> u32 {
  let lines = parse(input);
  let mut overlaps: HashMap<Point, u32> = HashMap::new();

  for line in lines {
    let points = rasterize(line);

    for p in points {
      *overlaps.entry(p).or_default() += 1;
    }
  }

  overlaps.iter().filter(|(_, &counts)| counts > 1).count() as _
}

fn main() {
  println!("part1");
  println!("sample: {:?}", solve1(SAMPLE));
  println!("input: {:?}", solve1(INPUT));

  println!("part2");
  println!("sample: {:?}", solve2(SAMPLE));
  println!("input: {:?}", solve2(INPUT));
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_rasterize() {
    assert_eq!(rasterize(([1, 1], [1, 3])), vec![[1, 1], [1, 2], [1, 3]]);
    assert_eq!(rasterize(([9, 7], [7, 7])), vec![[7, 7], [8, 7], [9, 7]]);
    assert_eq!(rasterize(([1, 1], [3, 3])), vec![[1, 1], [2, 2], [3, 3]]);
    assert_eq!(rasterize(([9, 7], [7, 9])), vec![[9, 7], [8, 8], [7, 9]]);
  }
}
