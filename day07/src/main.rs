const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<i32> {
  input.split(',').flat_map(|x| x.parse()).collect()
}

fn solve1(input: &str) -> u32 {
  let positions = parse(input);
  positions
    .iter()
    .map(|pivot| positions.iter().map(|i| (pivot - i).abs() as u32).sum())
    .min()
    .unwrap_or(0)
}

fn solve2(input: &str) -> u32 {
  let positions = parse(input);
  positions
    .iter()
    .map(|pivot| {
      positions
        .iter()
        .map(|i| series_sum((pivot - i).abs() as u32))
        .sum()
    })
    .min()
    .unwrap_or(0)
}

fn series_sum(n: u32) -> u32 {
  n * (n + 1) / 2
}

fn main() {
  println!("part1");
  println!("sample: {:?}", solve1(SAMPLE));
  println!("input: {:?}", solve1(INPUT));

  println!("part2");
  println!("sample: {:?}", solve2(SAMPLE));
  println!("input: {:?}", solve2(INPUT));
}
