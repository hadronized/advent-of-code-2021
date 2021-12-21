#![feature(box_patterns)]

use std::ops::Add;

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::digit1,
  combinator::map,
  sequence::{delimited, separated_pair},
  IResult,
};

#[derive(Debug)]
enum Snailfish {
  Pair(Box<Snailfish>, Box<Snailfish>),
  Number(u32),
}

impl Snailfish {
  fn reduce(mut self) -> Self {
    loop {
      let (reduced, changed) = self.do_reduction(1, None);

      self = reduced;
      if !changed {
        break self;
      }
    }
  }

  fn do_reduction(self, depth: usize, left_parent: &Option<u32>) -> (Self, bool) {
    match self {
      Snailfish::Pair(a, b) if depth == 4 => (
        Snailfish::Pair(Box::new(a.explode(left_parent, &b)), b),
        true,
      ),
      Snailfish::Pair(box Snailfish::Number(n), b) if n >= 10 => {
        (Snailfish::Pair(Box::new(Self::split(n)), b), true)
      }

      Snailfish::Pair(a, b) => {
        let depth = depth + 1;
        let (a, ab) = a.do_reduction(depth, left_parent);
        let (b, bb) = b.do_reduction(depth, &a);
        (Snailfish::Pair(Box::new(a), Box::new(b)), ab || bb)
      }

      _ => (self, false),
    }
  }

  fn explode(self) -> Self {
    self
  }

  fn split(n: u32) -> Self {
    let n2 = n as f32 * 0.5;
    let a = Snailfish::Number(n2.round() as _);
    let b = Snailfish::Number(n2.ceil() as _);
    Snailfish::Pair(Box::new(a), Box::new(b))
  }
}

impl Add for Snailfish {
  type Output = Snailfish;

  fn add(self, rhs: Self) -> Self::Output {
    Snailfish::Pair(Box::new(self), Box::new(rhs))
  }
}

fn parse_the_fucking_snailfish(input: &str) -> IResult<&str, Snailfish> {
  alt((
    map(digit1, |s: &str| Snailfish::Number(s.parse().unwrap())),
    delimited(
      tag("["),
      map(
        separated_pair(
          parse_the_fucking_snailfish,
          tag(","),
          parse_the_fucking_snailfish,
        ),
        |(a, b)| Snailfish::Pair(Box::new(a), Box::new(b)),
      ),
      tag("]"),
    ),
  ))(input)
}

fn main() {
  println!("part1");
  println!("sample: {:#?}", parse_the_fucking_snailfish("[[1,2],3]"));
}
