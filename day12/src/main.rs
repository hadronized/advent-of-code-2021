use std::collections::{HashMap, HashSet};

const SAMPLE: &str = include_str!("sample.txt");
const INPUT: &str = include_str!("input.txt");

type Node = String;
type Adj = HashMap<Node, Vec<Node>>;

fn parse(input: &str) -> Adj {
  let edges = input.split_whitespace().map(|line| {
    let parts: Vec<_> = line.split('-').collect();
    (parts[0].to_owned(), parts[1].to_owned())
  });

  let mut adj: Adj = HashMap::new();
  for (a, b) in edges {
    adj.entry(a.clone()).or_default().push(b.clone());
    adj.entry(b.clone()).or_default().push(a.clone());
  }

  adj
}

fn can_visit(visitable: &HashMap<Node, usize>, node: &Node) -> bool {
  if *node == node.to_uppercase() {
    return true;
  }

  visitable.get(node).copied().unwrap_or(0) > 0
}

fn visit(
  adj: &Adj,
  visitable: &mut HashMap<Node, usize>,
  mut path: Vec<Node>,
  node: &Node,
  f: &mut impl FnMut(&Vec<Node>),
) {
  path.push(node.clone());

  if node == "end" {
    f(&path);
    return;
  }

  if *node == node.to_lowercase() {
    for v in visitable.get_mut(node) {
      *v -= 1;
    }
  }

  for node in adj.get(node).iter().flat_map(|nodes| nodes.iter()) {
    if can_visit(visitable, node) {
      let mut visitable = visitable.clone();
      visit(adj, &mut visitable, path.clone(), node, f);
    }
  }
}

fn visit_start(adj: &Adj, visitable: HashMap<Node, usize>, f: &mut impl FnMut(&Vec<Node>)) {
  for node in adj.get("start").iter().flat_map(|nodes| nodes.iter()) {
    let path = vec!["start".to_owned()];

    if can_visit(&visitable, node) {
      let mut visitable = visitable.clone();
      visit(adj, &mut visitable, path, node, f);
    }
  }
}

fn solve1(input: &str) -> u32 {
  let adj = parse(input);
  let mut count = 0;

  let mut visitable = HashMap::new();

  for node in adj.keys().filter(|n| **n == n.to_lowercase()) {
    visitable.insert(node.clone(), 1);
  }

  *visitable.entry("start".to_owned()).or_default() = 0;

  visit_start(&adj, visitable, &mut |_| {
    count += 1;
  });

  count
}

fn solve2(input: &str) -> u32 {
  let adj = parse(input);

  let mut long_live_dogs = HashSet::new();
  for low_node_twice in adj
    .keys()
    .filter(|n| **n == n.to_lowercase() && *n != "start" && *n != "end")
  {
    let mut visitable = HashMap::new();

    for node in adj.keys().filter(|n| **n == n.to_lowercase()) {
      visitable.insert(node.clone(), 1);
    }

    *visitable.entry(low_node_twice.clone()).or_default() = 2;
    *visitable.entry("start".to_owned()).or_default() = 0;

    visit_start(&adj, visitable, &mut |path| {
      long_live_dogs.insert(path.clone());
    });
  }

  long_live_dogs.len() as _
}

fn main() {
  println!("part1");
  println!("sample: {}", solve1(SAMPLE));
  println!("input: {}", solve1(INPUT));

  println!("part2");
  println!("sample: {}", solve2(SAMPLE));
  println!("input: {}", solve2(INPUT));
}
