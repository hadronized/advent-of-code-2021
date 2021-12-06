use std::collections::BTreeSet;

const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> (Vec<u32>, Vec<Vec<Vec<i32>>>) {
  let parts: Vec<_> = input.split("\n\n").collect();
  let randoms = parts[0]
    .split(',')
    .flat_map(|n| n.parse::<u32>())
    .collect::<Vec<_>>();
  let boards = parts[1..]
    .iter()
    .map(|l| {
      l.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
          l.split_whitespace()
            .flat_map(|n| n.parse::<i32>())
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  (randoms, boards)
}

fn solve1(input: &str) -> u32 {
  let (randoms, mut boards) = parse(input);
  let exclude = BTreeSet::new();
  for rand in randoms {
    mark(&mut boards, rand, &exclude);

    if let Some(i) = check_winner(&boards, &exclude) {
      return rand * board_score(&boards[i]);
    }
  }

  0
}

fn solve2(input: &str) -> u32 {
  let (randoms, mut boards) = parse(input);
  let mut exclude = BTreeSet::new();
  let mut winning_score = 0;
  for rand in randoms {
    mark(&mut boards, rand, &exclude);

    while let Some(i) = check_winner(&boards, &exclude) {
      winning_score = rand * board_score(&boards[i]);
      exclude.insert(i);
    }
  }

  return winning_score;
}

fn mark(boards: &mut Vec<Vec<Vec<i32>>>, n: u32, exclude: &BTreeSet<usize>) {
  for (_, board) in boards
    .iter_mut()
    .enumerate()
    .filter(|(i, _)| !exclude.contains(i))
  {
    for row in board {
      for x in row {
        if *x == n as i32 {
          *x = -1;
        }
      }
    }
  }
}

fn check_winner(boards: &Vec<Vec<Vec<i32>>>, exclude: &BTreeSet<usize>) -> Option<usize> {
  for (i, board) in boards
    .iter()
    .enumerate()
    .filter(|(i, _)| !exclude.contains(i))
  {
    // check rows
    if board.iter().any(|l| l.iter().all(|&n| n == -1)) {
      return Some(i);
    }

    // check columns
    for j in 0..board[0].len() {
      let mut counts = 0;
      for row in board {
        if row[j] == -1 {
          counts += 1;
        }
      }

      if counts == board.len() {
        return Some(i);
      }
    }
  }

  None
}

fn board_score(board: &Vec<Vec<i32>>) -> u32 {
  board.iter().flatten().filter(|&&n| n != -1).sum::<i32>() as u32
}

#[allow(dead_code)]
fn display_board(board: &Vec<Vec<i32>>) {
  for row in board {
    println!("{:?}", row);
  }

  println!("");
}
#[allow(dead_code)]
fn display_boards(boards: &Vec<Vec<Vec<i32>>>) {
  for (i, board) in boards.iter().enumerate() {
    println!("board {}:", i);
    display_board(board);
  }
}

fn main() {
  println!("part1");
  println!("sample: {}", solve1(SAMPLE));
  println!("input: {}", solve1(INPUT));

  println!("part2");
  println!("sample: {}", solve2(SAMPLE));
  println!("input: {}", solve2(INPUT));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_check_winner() {
    let mut boards = vec![
      vec![
        vec![1, 2, 3, 4, 5],
        vec![-1, 7, 8, 9, 10],
        vec![-1, 12, 13, 14, 15],
        vec![-1, 17, 18, 19, 20],
        vec![-1, -1, -1, -1, 66],
      ],
      vec![
        vec![10, 20, 30, 40, -1],
        vec![60, 70, 80, 90, -1],
        vec![110, 120, 130, 140, -1],
        vec![160, 170, 180, 190, -1],
        vec![210, 220, 230, 240, 99],
      ],
    ];

    let mut exclude = BTreeSet::new();

    // no winner when we start
    assert_eq!(check_winner(&boards, &exclude), None);

    // after 66, board 0 is winner
    mark(&mut boards, 66, &exclude);
    assert_eq!(check_winner(&boards, &exclude), Some(0));
    exclude.insert(0);

    // after 1, 0 is not winner again because it was already in the past
    mark(&mut boards, 1, &exclude);
    assert_eq!(check_winner(&boards, &exclude), None);

    // after 99, board 1 is winner
    mark(&mut boards, 99, &exclude);
    assert_eq!(check_winner(&boards, &exclude), Some(1));
  }

  #[test]
  fn test_part2() {
    let mut boards = vec![
      vec![
        vec![1, 2, 3, 4, 5],
        vec![-1, 7, 8, 9, 10],
        vec![-1, 12, 13, 14, 15],
        vec![-1, 17, 18, 19, 20],
        vec![-1, -1, -1, -1, 66],
      ],
      vec![
        vec![10, 20, 30, 40, -1],
        vec![60, 70, 80, 90, -1],
        vec![110, 120, 130, 140, -1],
        vec![160, 170, 180, 190, -1],
        vec![210, 220, 230, 240, 99],
      ],
    ];
    let randoms = vec![66, 1, 99];
    let mut exclude = BTreeSet::new();
    let mut score = 0;

    for rand in randoms {
      mark(&mut boards, rand, &exclude);

      if let Some(i) = check_winner(&boards, &exclude) {
        score = rand as u32 * board_score(&boards[i]);
        exclude.insert(i);
      }
    }

    assert_eq!(
      score,
      (10
        + 20
        + 30
        + 40
        + 60
        + 70
        + 80
        + 90
        + 110
        + 120
        + 130
        + 140
        + 160
        + 170
        + 180
        + 190
        + 210
        + 220
        + 230
        + 240)
        * 99
    );
  }
}
