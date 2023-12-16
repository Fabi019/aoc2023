use std::collections::HashMap;

aoc2023::main!("../../assets/day15.txt");

fn part1(input: &str) -> u32 {
    input.lines().next().unwrap().split(',').map(dohash).sum()
}

fn part2(input: &str) -> u32 {
    let mut boxes = HashMap::new();

    for s in input.lines().next().unwrap().split(',') {
        if s.ends_with('-') {
            let label = s.strip_suffix('-').unwrap();
            let hash = dohash(label);
            // Remove if it is present
            boxes.entry(hash).and_modify(|v: &mut Vec<(&str, u32)>| {
                if let Some(i) = v.iter().position(|(k, _)| k == &label) {
                    v.remove(i);
                }
            });
        } else if s.contains('=') {
            let (label, focal) = s.split_once('=').unwrap();
            let hash = dohash(label);
            // Using a hashmap is not possible because the order is important
            let vec = boxes.entry(hash).or_insert(Vec::new());
            // Update if it is present
            if let Some(i) = vec.iter().position(|(k, _)| k == &label) {
                vec[i].1 = focal.parse().unwrap();
            } else {
                vec.push((label, focal.parse().unwrap()));
            }
        }
    }

    boxes
        .into_iter()
        .flat_map(|(hash, v)| {
            v.into_iter()
                .enumerate()
                .map(move |(i, (_, v))| (hash + 1) * (i + 1) as u32 * v)
        })
        .sum()
}

fn dohash(s: &str) -> u32 {
    let mut hash = 0;
    for c in s.chars() {
        hash += c as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

aoc2023::test!(
    "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
",
    1320,
    145
);
