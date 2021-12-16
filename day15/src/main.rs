use std::collections::{HashMap, VecDeque};

const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<Vec<u8>> {
  input
    .split_whitespace()
    .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
    .collect()
}

type Point = (usize, usize);
type Weight = u32;
type Parents = HashMap<Point, Option<(Point, Weight)>>;

fn solve1(input: &str) -> u32 {
  shortest_path(parse(input))
}

fn solve2(input: &str) -> u32 {
  let mut caves = parse(input);

  let col = caves[0].len();
  for row in &mut caves {
    for i in 1..5 {
      let new_row: Vec<_> = row.iter().take(col).map(|&x| (x + i - 1) % 9 + 1).collect();
      row.extend_from_slice(&new_row);
    }
  }

  let row_len = caves.len();
  for i in 1..5 {
    for r in 0..row_len {
      let row = caves[r].iter().map(|&x| (x + i - 1) % 9 + 1).collect();
      caves.push(row);
    }
  }

  shortest_path(caves)
}

fn shortest_path(caves: Vec<Vec<u8>>) -> u32 {
  // map for a given node its parent node by which we should go to minimize a path to this node; this associates both
  // the parent point, and its weight
  let mut parents: Parents = (0..caves.len())
    .into_iter()
    .flat_map(|row| {
      (0..caves[0].len())
        .into_iter()
        .map(move |col| ((row, col), None))
    })
    .collect();

  // list of things to visit next
  let mut visits = VecDeque::new();

  visits.push_back((0, 0)); // we start at the top-left corner
  parents.insert((0, 0), Some(((0, 0), 0))); // start has itself as parent and no weight

  while let Some(current) = visits.pop_front() {
    // current best parent node
    let (_, cp_weight) = parents.get(&current).copied().flatten().unwrap();

    // get the list of neighbors from the current position
    for (n_row, n_col, n_weight) in neighbors(&caves, current) {
      // get the current best parent node; if none, insert the current node as best solution; if the parent is not known
      // yet, we simply use the current one and the neighbor’s weight
      let neighbor_parent = parents.entry((n_row, n_col)).or_insert(None);

      match neighbor_parent {
        Some((location, np_weight)) => {
          if cp_weight + (n_weight as u32) < *np_weight {
            // the current node is a better solution
            *location = current;
            *np_weight = cp_weight + n_weight as u32;

            // keep exploring
            visits.push_back((n_row, n_col));
          }
        }

        _ => {
          *neighbor_parent = Some((current, cp_weight + n_weight as u32));
          // keep exploring
          visits.push_back((n_row, n_col));
        }
      }
    }
  }

  let goal = (caves.len() - 1, caves[0].len() - 1);
  parents
    .get(&goal)
    .copied()
    .flatten()
    .map(|(_, weight)| weight)
    .unwrap_or(0)
}

fn neighbors(caves: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> Vec<(usize, usize, u8)> {
  let mut nodes = Vec::new();

  for r in -1..=1 {
    for c in -1..=1 {
      if (r != 0 && c != 0) || (r == 0 && c == 0) {
        continue;
      }

      let row = (row as isize + r) as usize;
      let col = (col as isize + c) as usize;

      if let Some(&weight) = caves.get(row).and_then(|r| r.get(col)) {
        nodes.push((row, col, weight));
      }
    }
  }

  nodes
}

fn main() {
  println!("part1");
  println!("sample: {}", solve1(SAMPLE));
  println!("input: {}", solve1(INPUT));

  println!("part2");
  println!("sample: {}", solve2(SAMPLE));
  println!("input: {}", solve2(INPUT));
}
