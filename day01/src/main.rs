const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn solve1(input: &str) -> u32 {
  let (count, _) = input
    .split_whitespace()
    .flat_map(|n| n.parse::<u32>())
    .fold((0, 0), |(count, prev_depth), depth| {
      let count = if depth > prev_depth { count + 1 } else { count };
      (count, depth)
    });

  count - 1 // because of the initial addition (no data)
}

fn solve2(input: &str) -> u32 {
  let depths: Vec<_> = input
    .split_whitespace()
    .flat_map(|n| n.parse::<u32>())
    .collect();
  let a = depths.iter();
  let b = depths.iter().skip(1);
  let c = depths.iter().skip(2);

  let (count, _) = a
    .zip(b)
    .zip(c)
    .fold((0, 0), |(count, prev_depth), ((a, b), c)| {
      let depth = a + b + c;
      let count = if depth > prev_depth { count + 1 } else { count };
      (count, depth)
    });

  count - 1
}

fn main() {
  println!("part1");
  println!("sample: {}", solve1(SAMPLE));
  println!("input: {}", solve1(INPUT));

  println!("part2");
  println!("sample: {}", solve2(SAMPLE));
  println!("input: {}", solve2(INPUT));
}
