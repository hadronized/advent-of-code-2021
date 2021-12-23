// fuck this puzzle, I’m out.

use std::{cell::RefCell, rc::Rc};

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::digit1,
  combinator::map,
  sequence::{delimited, separated_pair},
  IResult,
};

type PZ = Rc<RefCell<Zipper>>;

#[derive(Debug)]
enum ZipperContent {
  Number(u32),
  Pair { left: PZ, right: PZ },
}

#[derive(Debug)]
struct Zipper {
  parent: Option<PZ>,
  content: ZipperContent,
}

impl Zipper {
  fn depth(&self) -> usize {
    1 + self
      .parent
      .as_ref()
      .map(|p| p.borrow().depth())
      .unwrap_or(0)
  }

  fn reduce(zipper: &PZ) {
    loop {
      if !(Self::explode(zipper) || Self::split(zipper)) {
        break;
      }
    }
  }

  fn split(zipper: &PZ) -> bool {
    match zipper.borrow().content {
      ZipperContent::Number(n) if n >= 10 => {
        let n2 = n as f32 * 0.5;

        let content_left = ZipperContent::Number(n2.round() as _);
        let left = Rc::new(RefCell::new(Zipper {
          parent: Some(zipper.clone()),
          content: content_left,
        }));

        let content_right = ZipperContent::Number(n2.ceil() as _);
        let right = Rc::new(RefCell::new(Zipper {
          parent: Some(zipper.clone()),
          content: content_right,
        }));

        zipper.borrow_mut().content = ZipperContent::Pair { left, right };
        true
      }

      _ => false,
    }
  }

  fn explode(zipper: &PZ) -> bool {
    match zipper.borrow().content {
      ZipperContent::Pair {
        ref left,
        ref right,
      } => {
        match (left.borrow().content, right.borrow().content) {
          (ZipperContent::Number(left), ZipperContent::Number(right))
            if zipper.borrow().depth() == 4 =>
          {
            // here, we first replace the current zipper, so that we can go up and replace Pair only
            zipper.borrow_mut().content = ZipperContent::Number(0);

            // first find left
            let mut current = zipper.clone();
            while let Some(parent) = current.borrow().parent {
              if let ZipperContent::Pair { right, .. } = parent.borrow().content {
                if let ZipperContent::Pair { .. } = right.borrow().content {
                  current = parent;
                  break;
                }
              }

              current = parent;
            }

            // go down until we reach the number
            loop {
              match current.borrow_mut().content {
                ZipperContent::Number(n) => {
                  n = left;
                  break;
                }

                ZipperContent::Pair { right, .. } => current = right.clone(),
              }
            }

            // same shit with right
            current = zipper.clone();
            while let Some(parent) = current.borrow().parent {
              if let ZipperContent::Pair { left, .. } = parent.borrow().content {
                if let ZipperContent::Pair { .. } = left.borrow().content {
                  current = parent;
                  break;
                }
              }

              current = parent;
            }

            loop {
              match current.borrow_mut().content {
                ZipperContent::Number(n) => {
                  n = right;
                  break;
                }

                ZipperContent::Pair { left, .. } => current = left.clone(),
              }
            }

            true
          }

          _ => Self::split(left) && Self::split(right),
        }
      }

      _ => false,
    }
  }
}

fn parse_the_fucking_snailfish(input: &str) -> IResult<&str, Zipper> {
  alt((
    map(digit1, |s: &str| Zipper {
      parent: None,
      content: ZipperContent::Number(s.parse().unwrap()),
    }),
    delimited(
      tag("["),
      map(
        separated_pair(
          parse_the_fucking_snailfish,
          tag(","),
          parse_the_fucking_snailfish,
        ),
        |(a, b)| Zipper {
          parent: None,
          content: ZipperContent::Pair {
            left: Rc::new(RefCell::new(a)),
            right: Rc::new(RefCell::new(b)),
          },
        },
      ),
      tag("]"),
    ),
  ))(input)
}

fn main() {
  println!("part1");
  println!("sample: {:#?}", parse_the_fucking_snailfish("[[1,2],3]"));
}
