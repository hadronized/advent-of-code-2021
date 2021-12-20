use std::ops::RangeInclusive;

const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
  let input: String = input.chars().skip("target area: x=".len()).collect();
  let parts: Vec<Vec<i32>> = input
    .trim()
    .split(", y=")
    .map(|r| r.split("..").flat_map(|n| n.parse()).collect())
    .collect();

  (parts[0][0]..=parts[0][1], parts[1][0]..=parts[1][1])
}

type Pos = [i32; 2];
type Vel = [i32; 2];

fn step_probe((pos, vel): (Pos, Vel)) -> (Pos, Vel) {
  let new_pos = [pos[0] + vel[0], pos[1] + vel[1]];
  let new_vel_x;

  if vel[0] < 0 {
    new_vel_x = vel[0] + 1;
  } else if vel[0] > 0 {
    new_vel_x = vel[0] - 1;
  } else {
    new_vel_x = 0;
  }

  let new_vel = [new_vel_x, vel[1] - 1];

  (new_pos, new_vel)
}

fn find_x_vel(range: &RangeInclusive<i32>) -> i32 {
  let mut starting_vel = 1;
  loop {
    let mut probe = ([0, 0], [starting_vel, 0]);

    while probe.1[0] > 0 {
      probe = step_probe(probe);
    }

    let x = probe.0[0];

    if range.contains(&x) {
      break starting_vel;
    }

    starting_vel += 1;
  }
}

fn find_y_vel(vel_x: i32, range: &RangeInclusive<i32>) -> (i32, i32) {
  let mut starting_vel = 1;
  let mut best = (0, 0);

  // 1000 because only SHEEP have shame!
  while starting_vel < 1000 {
    let mut probe = ([0, 0], [vel_x, starting_vel]);
    let mut best_height = 0;
    let mut went_through = false;

    while probe.0[1] > *range.start() {
      probe = step_probe(probe);
      best_height = best_height.max(probe.0[1]);
      went_through = range.contains(&probe.0[1]);
    }

    if went_through {
      best = (starting_vel, best_height);
    }

    starting_vel += 1;
  }

  best
}

fn solve2(input: &str) -> u32 {
  let ranges = parse(input);
  let mut solutions = 0;

  for vel_x in -500..500 {
    'vel_y: for vel_y in -500..500 {
      let mut probe = ([0, 0], [vel_x, vel_y]);

      while probe.0[1] > *ranges.1.start() {
        probe = step_probe(probe);
        if ranges.0.contains(&probe.0[0]) && ranges.1.contains(&probe.0[1]) {
          solutions += 1;
          continue 'vel_y;
        }
      }
    }
  }

  solutions
}

fn solve1(input: &str) -> i32 {
  let ranges = parse(input);
  let vel_x = find_x_vel(&ranges.0);
  let (_, pos_y) = find_y_vel(vel_x, &ranges.1);
  pos_y
}

fn main() {
  println!("part1");
  println!("sample: {}", solve1(SAMPLE));
  println!("input: {}", solve1(INPUT));

  println!("part2");
  println!("sample: {}", solve2(SAMPLE));
  println!("input: {}", solve2(INPUT));
}
