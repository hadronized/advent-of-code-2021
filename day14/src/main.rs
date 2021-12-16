use std::collections::HashMap;

const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> (Vec<char>, HashMap<[char; 2], char>) {
  let parts: Vec<_> = input.split("\n\n").collect();
  let template = parts[0].chars().collect();

  let rules = parts[1]
    .split('\n')
    .filter(|l| !l.is_empty())
    .map(|l| {
      let rule: Vec<_> = l.split(" -> ").collect();
      let rule0: Vec<_> = rule[0].chars().collect();
      let rule1: Vec<_> = rule[1].chars().collect();

      ([rule0[0], rule0[1]], rule1[0])
    })
    .collect();

  (template, rules)
}

fn solve12(input: &str, steps: usize) -> u64 {
  let (template, rules) = parse(input);

  let mut pairs = HashMap::<[char; 2], i64>::new();
  for pair in template.windows(2) {
    let p = [pair[0], pair[1]];
    *pairs.entry(p).or_default() += 1;
  }

  let mut pairs_target = pairs.clone();

  for _ in 0..steps {
    for (pair, count) in pairs.iter().filter(|(_, &count)| count > 0) {
      *pairs_target.entry(*pair).or_insert(*count) -= *count;

      let p = rules.get(pair).expect("unknown rule pair");
      let a = [pair[0], *p];
      let b = [*p, pair[1]];
      *pairs_target.entry(a).or_default() += count;
      *pairs_target.entry(b).or_default() += count;
    }

    pairs = pairs_target.clone();
  }

  let mut freqs = HashMap::<char, i64>::new();
  for (pair, count) in pairs {
    *freqs.entry(pair[0]).or_default() += count / 2;
    *freqs.entry(pair[1]).or_default() += count / 2;
  }

  let (min, max) =
    freqs
      .into_iter()
      .filter(|(_, x)| *x > 0)
      .fold((u64::MAX, u64::MIN), |(min, max), (_, a)| {
        let a = a as u64;
        (min.min(a), max.max(a))
      });

  max - min
}

fn main() {
  println!("part1");
  println!("sample: {}", solve12(SAMPLE, 10));
  println!("input: {}", solve12(INPUT, 10));

  println!("part2");
  println!("sample: {}", solve12(SAMPLE, 40));
  println!("input: {}", solve12(INPUT, 40));
}
