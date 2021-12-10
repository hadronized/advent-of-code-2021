const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<&str> {
  input.split_whitespace().collect()
}

fn del_score(del: char) -> u64 {
  match del {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => 0,
  }
}

fn del_close(close: char) -> char {
  match close {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
    _ => panic!("{} is an invalid closing delimiter", close),
  }
}

fn del_complete_score(del: char) -> u64 {
  match del {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
    _ => 0,
  }
}

fn solve1(input: &str) -> u64 {
  let lines = parse(input);
  let mut total_score = 0;

  for line in lines {
    let mut del = Vec::new();
    let mut line_score = 0;

    for c in line.chars() {
      let score = del_score(c);

      if score > 0 {
        // it’s a closing delimiter
        match del.pop() {
          Some(open) if del_close(open) != c => line_score += score,
          _ => (),
        }
      } else {
        // it should be an opening one, so simply add it to the stack
        del.push(c);
      }
    }

    total_score += line_score;
  }

  total_score
}

fn solve2(input: &str) -> u64 {
  let lines = parse(input);
  let mut scores = Vec::new();

  'line: for line in lines {
    let mut del = Vec::new();

    for c in line.chars() {
      let score = del_score(c);

      if score > 0 {
        // it’s a closing delimiter
        match del.pop() {
          Some(open) if del_close(open) != c => continue 'line,
          _ => (),
        }
      } else {
        // it should be an opening one, so simply add it to the stack
        del.push(c);
      }
    }

    scores.push(
      del
        .into_iter()
        .rev()
        .fold(0, |s, c| s * 5 + del_complete_score(del_close(c))),
    );
  }

  scores.sort();
  scores[scores.len() / 2]
}

fn main() {
  println!("part1");
  println!("sample: {:?}", solve1(SAMPLE));
  println!("input: {:?}", solve1(INPUT));

  println!("part2");
  println!("sample: {:?}", solve2(SAMPLE));
  println!("input: {:?}", solve2(INPUT));
}
