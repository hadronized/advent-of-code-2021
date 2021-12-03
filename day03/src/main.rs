const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn solve1(input: &str) -> u32 {
  let bits: Vec<_> = input
    .split_whitespace()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect();
  let half_len = bits.len() as u32 / 2;

  let bit_size = bits[0].len();
  let mut ones = vec![0; bit_size];

  for bits in bits {
    for i in 0..bit_size {
      ones[i] += (bits[i] == '1') as u32;
    }
  }

  let mut gamma = 0;
  let mut epsilon = 0;

  for i in 0..bit_size {
    if ones[i] > half_len {
      gamma = gamma * 2 + 1;
      epsilon = epsilon * 2;
    } else {
      gamma = gamma * 2;
      epsilon = epsilon * 2 + 1;
    }
  }

  gamma * epsilon
}

fn solve2(input: &str) -> u32 {
  let bits: Vec<_> = input
    .split_whitespace()
    .map(|l| l.chars().collect::<Vec<_>>())
    .collect();
  let mut o2_bits = bits.clone();
  let mut co2_bits = bits.clone();

  let bit_size = bits[0].len();

  // o2
  for i in 0..bit_size {
    let mut ones = 0;
    let mut zeroes = 0;
    for bits in &o2_bits {
      ones += (bits[i] == '1') as u32;
      zeroes += (bits[i] == '0') as u32;
    }

    o2_bits.retain(|bits| {
      if ones >= zeroes {
        bits[i] == '1'
      } else {
        bits[i] == '0'
      }
    });

    if o2_bits.len() == 1 {
      break;
    }
  }

  let o2 = o2_bits[0]
    .iter()
    .fold(0, |o2, b| o2 * 2 + (*b == '1') as u32);

  // co2
  for i in 0..bit_size {
    let mut ones = 0;
    let mut zeroes = 0;
    for bits in &co2_bits {
      ones += (bits[i] == '1') as u32;
      zeroes += (bits[i] == '0') as u32;
    }

    co2_bits.retain(|bits| {
      if ones < zeroes {
        bits[i] == '1'
      } else {
        bits[i] == '0'
      }
    });

    if co2_bits.len() == 1 {
      break;
    }
  }

  let co2 = co2_bits[0]
    .iter()
    .fold(0, |co2, b| co2 * 2 + (*b == '1') as u32);

  o2 * co2
}

fn main() {
  println!("part1");
  println!("sample: {}", solve1(SAMPLE));
  println!("input: {}", solve1(INPUT));

  println!("part2");
  println!("sample: {}", solve2(SAMPLE));
  println!("input: {}", solve2(INPUT));
}
