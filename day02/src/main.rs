const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn solve1(input: &str) -> i32 {
  let (h, depth) = input
    .split('\n')
    .filter(|l| !l.is_empty())
    .map(|l| {
      let words: Vec<_> = l.split(' ').collect();
      (words[0], words[1].parse::<i32>().unwrap())
    })
    .fold((0, 0), |(h, depth), (motion, amount)| match motion {
      "forward" => (h + amount, depth),
      "down" => (h, depth + amount),
      "up" => (h, depth - amount),
      _ => panic!("NOPE"),
    });

  h * depth
}

fn solve2(input: &str) -> i32 {
  let (h, depth, _) = input
    .split('\n')
    .filter(|l| !l.is_empty())
    .map(|l| {
      let words: Vec<_> = l.split(' ').collect();
      (words[0], words[1].parse::<i32>().unwrap())
    })
    .fold(
      (0, 0, 0),
      |(h, depth, aim), (motion, amount)| match motion {
        "forward" => (h + amount, depth + aim * amount, aim),
        "down" => (h, depth, aim + amount),
        "up" => (h, depth, aim - amount),
        _ => panic!("NOPE"),
      },
    );

  h * depth
}

fn main() {
  println!("part1");
  println!("sample: {}", solve1(SAMPLE));
  println!("input: {}", solve1(INPUT));

  println!("part2");
  println!("sample: {}", solve2(SAMPLE));
  println!("input: {}", solve2(INPUT));
}
