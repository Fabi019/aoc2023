use std::collections::{HashMap, VecDeque};

aoc2023::main!("../../assets/day15.txt");

fn part1(input: &str) -> u32 {
    let mut hash_sum = 0;
    for s in input.lines().next().unwrap().split(',') {
        let mut hash = 0;
        for c in s.chars() {
            hash += (c as u8) as u32;
            hash *= 17;
            hash %= 256;
        }
        hash_sum += hash;
    }
    hash_sum
}

fn part2(input: &str) -> u32 {
    let mut boxes = HashMap::new();
    for s in input.lines().next().unwrap().split(',') {
        if s.ends_with('-') {
            let label = s.strip_suffix('-').unwrap();
            let hash = dohash(label);
            boxes
                .entry(hash)
                .and_modify(|v: &mut VecDeque<(&str, u32)>| {
                    v.retain(|(k, _)| k != &label);
                });
        } else if s.contains('=') {
            let (label, focal) = s.split_once('=').unwrap();
            let hash = dohash(label);

            let vec = boxes.entry(hash).or_insert(VecDeque::new());

            let mut found = false;
            for (k, v) in vec.iter_mut() {
                if k == &label {
                    *v = focal.parse().unwrap();
                    found = true;
                }
            }
            if !found {
                vec.push_back((label, focal.parse().unwrap()));
            }
        }
    }
    let mut focal_sum = 0;
    for (hash, v) in boxes {
        for (i, (_, v)) in v.iter().enumerate() {
            let slot = i + 1;
            focal_sum += (hash + 1) * slot as u32 * v;
        }
    }
    focal_sum
}

fn dohash(s: &str) -> u32 {
    let mut hash = 0;
    for c in s.chars() {
        hash += (c as u8) as u32;
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
