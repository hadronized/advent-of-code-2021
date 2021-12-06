const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<u32> {
  input.split(',').flat_map(|n| n.parse()).collect()
}

fn solve1(input: &str) -> u32 {
  let mut counters = parse(input);

  for _ in 0..80 {
    let mut newborn = 0;

    for counter in &mut counters {
      if *counter == 0 {
        *counter = 6;
        newborn += 1;
      } else {
        *counter -= 1;
      }
    }

    for _ in 0..newborn {
      counters.push(8);
    }
  }

  counters.len() as _
}

fn main() {
  println!("part1");
  println!("sample: {}", solve1(SAMPLE));
  println!("input: {}", solve1(INPUT));
}
