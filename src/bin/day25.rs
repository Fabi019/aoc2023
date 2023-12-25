use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

aoc2023::main!("../../assets/day25.txt");

fn part1(input: &str) -> usize {
    let mut nodes = input
        .lines()
        .map(|line| {
            let (node, children) = line.split_once(": ").unwrap();
            let children = children.split_whitespace().collect::<HashSet<_>>();
            (node, children)
        })
        .collect::<HashMap<_, _>>();

    // Add reverse connections to each node
    for (node, childs) in nodes.clone() {
        for child in childs {
            nodes.entry(child).or_default().insert(node);
        }
    }

    let mut done = HashSet::new();
    let mut all_steps = BTreeSet::new();

    for (node, childs) in &nodes {
        for child in childs {
            if !done.insert((node, child)) || !done.insert((child, node)) {
                continue;
            }

            // Find path between node and child without the direct connection
            let mut queue =
                VecDeque::from_iter(childs.iter().filter(|c| *c != child).map(|c| (1, *c)));

            let mut visited = HashSet::new();
            visited.insert(*node);

            let mut steps = 0;

            'outer: while let Some((s, node)) = queue.pop_front() {
                if !visited.insert(node) {
                    continue;
                }

                for c in &nodes[node] {
                    if c == child {
                        steps = s + 1;
                        break 'outer;
                    }
                    queue.push_back((s + 1, c));
                }
            }

            all_steps.insert((steps, node, child));
        }
    }

    // Remove the 3 connections with the longest distance
    let removed = all_steps.into_iter().rev().take(3).collect::<Vec<_>>();
    //println!("Removed: {:?}", removed);

    // Start anywhere and count the visited nodes without the connections that were removed
    let mut queue = VecDeque::new();
    queue.push_back(nodes.keys().next().unwrap());

    let mut visited = HashSet::new();

    while let Some(node) = queue.pop_front() {
        if !visited.insert(node) {
            continue;
        }

        for c in &nodes[node] {
            if removed
                .iter()
                .any(|(_, n, cc)| (*n == node && *cc == c) || (*n == c && *cc == node))
            {
                continue;
            }
            queue.push_back(c);
        }
    }

    //println!("Visited: {}", visited.len());

    visited.len() * (nodes.len() - visited.len())
}

fn part2(_input: &str) -> u32 {
    0
}

aoc2023::test!(
    "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
",
    54,
    0
);
