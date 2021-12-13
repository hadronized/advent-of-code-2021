const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Fold {
  X(u32),
  Y(u32),
}

fn parse(input: &str) -> (Vec<Vec<u32>>, Vec<Fold>) {
  let parts: Vec<_> = input.split("\n\n").collect();

  let mut max_x = 0;
  let mut max_y = 0;
  let coords: Vec<_> = parts[0]
    .split_whitespace()
    .map(|l| {
      let coords: Vec<_> = l.split(',').flat_map(|c| c.parse()).collect();
      max_x = max_x.max(coords[0]);
      max_y = max_y.max(coords[1]);
      [coords[0], coords[1]]
    })
    .collect();

  let mut points: Vec<Vec<u32>> = (0..=max_y)
    .into_iter()
    .map(|_| (0..=max_x).into_iter().map(|_| 0).collect())
    .collect();

  for [x, y] in coords {
    points[y][x] = 1;
  }

  let folds = parts[1]
    .split('\n')
    .filter(|l| !l.is_empty())
    .map(|l| {
      let l = &l["fold along ".len()..];
      let l = l.as_bytes();
      let v = String::from_utf8(l[2..].to_vec()).unwrap();
      match l[0] {
        b'x' => Fold::X(v.parse().unwrap()),
        b'y' => Fold::Y(v.parse().unwrap()),
        _ => panic!("shouldn’t happen"),
      }
    })
    .collect::<Vec<_>>();

  (points, folds)
}

fn solve12(input: &str) {
  let (mut points, folds) = parse(input);
  let mut first_fold = true;

  for fold in folds {
    match fold {
      Fold::Y(y) => {
        let len = points.len() as isize;
        for row in (len - 2 * y as isize - 1)..y as isize {
          let row = row as usize;

          for col in 0..points[row].len() {
            points[row][col] |= points[(len - row as isize - 1) as usize][col];
          }
        }

        points.resize(y as usize, Vec::new());
      }

      Fold::X(x) => {
        for row in 0..points.len() {
          let len = points[row].len() as isize;
          let x = x as isize;

          for col in (len - 2 * x - 1)..x {
            let col = col as usize;
            points[row][col] |= points[row][(len - col as isize - 1) as usize];
          }

          points[row].resize(x as usize, 0);
        }
      }
    }

    if first_fold {
      let count = points
        .iter()
        .flat_map(|p| p.iter().filter(|&&x| x == 1))
        .count();
      println!("count = {}", count);
      first_fold = false;
    }
  }

  for row in &points {
    for &x in row {
      print!("{}", if x == 0 { '.' } else { '#' });
    }

    println!();
  }
}

fn main() {
  println!("sample");
  solve12(SAMPLE);

  println!("input");
  solve12(INPUT);
}
