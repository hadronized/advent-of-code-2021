use std::collections::HashSet;

const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<Vec<u32>> {
  input
    .split_whitespace()
    .map(|line| line.chars().map(|c| (c as u8 - b'0') as u32).collect())
    .collect()
}

fn solve1(input: &str) -> u32 {
  let grid = parse(input);
  let width = grid[0].len();
  let height = grid.len();

  let mut lowest_points = 0;
  for x in 0..width {
    'y: for y in 0..height {
      let p = grid[y][x];
      let x = x as isize;
      let y = y as isize;

      for i in -1..=1 {
        for j in -1..=1 {
          if (i == j) || !(i == 0 || j == 0) {
            continue;
          }

          match grid
            .get((y + j) as usize)
            .and_then(|row| row.get((x + i) as usize))
          {
            Some(height) if p >= *height => continue 'y,
            _ => (),
          }
        }
      }

      lowest_points += p + 1;
    }
  }

  lowest_points
}

fn solve2(input: &str) -> u32 {
  let grid = parse(input);
  let width = grid[0].len();
  let height = grid.len();
  let mut unexplored: HashSet<(usize, usize)> = (0..width)
    .into_iter()
    .flat_map(|w| (0..height).into_iter().map(move |h| (w, h)))
    .collect();

  // list of basins size and current basin
  let mut basin_sizes = Vec::new();

  // next locations to visit
  let mut next_locations = Vec::new();

  // explore the whole grid
  while let Some(location) = get_explorable(&grid, &mut unexplored) {
    next_locations.push(location);
    let mut basin = 0;

    while let Some(location) = next_locations.pop() {
      basin += 1;
      next_to_visit(&grid, &mut unexplored, &mut next_locations, location);
    }

    basin_sizes.push(basin);
  }

  basin_sizes.sort_by(|a, b| b.cmp(a));
  basin_sizes.into_iter().take(3).product()
}

// Get any visitable location.
fn get_explorable(
  grid: &Vec<Vec<u32>>,
  unexplored: &mut HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
  unexplored
    .iter()
    .find(|(i, j)| grid[*j][*i] != 9)
    .cloned()
    .map(|location| {
      unexplored.remove(&location);
      location
    })
}

// Get neighbors and add them to the list of next location to visit.
fn next_to_visit(
  grid: &Vec<Vec<u32>>,
  unexplored: &mut HashSet<(usize, usize)>,
  next_locations: &mut Vec<(usize, usize)>,
  (x, y): (usize, usize),
) {
  for j in -1..=1 {
    for i in -1..=1 {
      if (i == j) || !(i == 0 || j == 0) {
        continue;
      }

      let x = ((x as isize) + i) as usize;
      let y = ((y as isize) + j) as usize;

      if !unexplored.contains(&(x, y)) {
        // already explored
        continue;
      }

      unexplored.remove(&(x, y));

      let l = grid[y][x];
      if l < 9 {
        next_locations.push((x, y));
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
