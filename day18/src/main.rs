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
