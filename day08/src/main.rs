use std::collections::HashMap;

const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse<'a>(input: &'a str) -> Vec<(Vec<&'a str>, Vec<&'a str>)> {
  input
    .split('\n')
    .filter(|line| !line.is_empty())
    .map(|line| {
      let mut parts: Vec<Vec<&str>> = line
        .split(" | ")
        .map(|strings| strings.split_whitespace().collect::<Vec<_>>())
        .collect();
      let b = parts.pop().unwrap();
      let a = parts.pop().unwrap();
      (a, b)
    })
    .collect()
}

fn solve1(input: &str) -> u32 {
  let lines = parse(input);
  let mut uniques = 0;
  for (_, output) in lines {
    for out in output {
      if [2, 3, 4, 7].iter().find(|&&v| v == out.len()).is_some() {
        uniques += 1;
      }
    }
  }

  uniques
}

fn solve2(input: &str) -> u32 {
  let lines = parse(input);
  let mut sum = 0;

  for (mut patterns, output) in lines {
    let mut mappings = vec![String::new(); 10];

    // search for uniques first
    patterns.retain(|pat| match pat.len() {
      2 => {
        mappings[1] = pat.to_string();
        false
      }

      4 => {
        mappings[4] = pat.to_string();
        false
      }

      3 => {
        mappings[7] = pat.to_string();
        false
      }

      7 => {
        mappings[8] = pat.to_string();
        false
      }

      _ => true,
    });

    // we can deduce 9 easily: it contains both 4 and 7 and has length 6; we can also deduce 3, since it has 1
    patterns.retain(|pat| {
      if pat.len() == 6
        && mappings[4].chars().all(|seg| pat.contains(seg))
        && mappings[7].chars().all(|seg| pat.contains(seg))
      {
        mappings[9] = pat.to_string();
        false
      } else if pat.len() == 5 && mappings[1].chars().all(|seg| pat.contains(seg)) {
        mappings[3] = pat.to_string();
        false
      } else {
        true
      }
    });

    // 2: 9 doesn’t contain it
    patterns.retain(|pat| {
      if pat.len() == 5 && !pat.chars().all(|seg| mappings[9].contains(seg)) {
        mappings[2] = pat.to_string();
        false
      } else {
        true
      }
    });

    // 5: it’s not 2 nor 3
    patterns.retain(|pat| {
      if pat.len() == 5 {
        mappings[5] = pat.to_string();
        false
      } else {
        true
      }
    });

    // 6: contains 5 and is not 9
    patterns.retain(|pat| {
      if pat.len() == 6 && pat != &mappings[9] && mappings[5].chars().all(|seg| pat.contains(seg)) {
        mappings[6] = pat.to_string();
        false
      } else {
        true
      }
    });

    // 0 is the last one we haven’t found yet
    mappings[0] = patterns[0].to_string();

    // lol we’re done, let’s decode those shitty numbers; first, reverse the mappings so that we can work correctly
    let digits: HashMap<_, _> = mappings
      .into_iter()
      .enumerate()
      .map(|(i, digit)| (sort_string(&digit), i as u32))
      .collect();

    let n = output
      .into_iter()
      .flat_map(|out| digits.get(&sort_string(out)))
      .fold(0, |n, d| n * 10 + d);

    sum += n;
  }

  sum
}

fn sort_string(s: &str) -> String {
  let mut bytes: Vec<_> = s.bytes().collect();
  bytes.sort();
  unsafe { String::from_utf8_unchecked(bytes) }
}

fn main() {
  println!("part1");
  println!("sample: {:?}", solve1(SAMPLE));
  println!("input: {:?}", solve1(INPUT));

  println!("part2");
  println!("sample: {:?}", solve2(SAMPLE));
  println!("input: {:?}", solve2(INPUT));
}
