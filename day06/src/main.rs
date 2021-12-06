const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<u64> {
  input.split(',').flat_map(|n| n.parse()).collect()
}

fn solve(input: &str, days: usize) -> u64 {
  let counters = parse(input);
  let mut ages = [0; 9];

  for counter in counters {
    ages[counter as usize] += 1;
  }

  for _ in 0..days {
    for i in 0..8 {
      ages.swap(i, i + 1);
    }

    ages[6] += ages[8];
  }

  ages.into_iter().sum()
}

fn main() {
  println!("part1");
  println!("sample: {}", solve(SAMPLE, 80));
  println!("input: {}", solve(INPUT, 80));

  println!("part2");
  println!("sample: {}", solve(SAMPLE, 256));
  println!("input: {}", solve(INPUT, 256));
}
