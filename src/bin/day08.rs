use std::collections::HashMap;

aoc2023::main!("../../assets/day08.txt");

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();

    let map = lines.skip(1).fold(HashMap::new(), |mut map, line| {
        let (key, value) = line.split_once(" = ").unwrap();
        let (left, right) = value.split_once(", ").unwrap();
        map.insert(key, (&left[1..], &right[..3]));
        map
    });

    let mut current = "AAA";
    1 + instructions
        .take_while(|next| {
            let (left, right) = map[current];
            current = match next {
                'L' => left,
                'R' => right,
                _ => unreachable!(),
            };
            current != "ZZZ"
        })
        .count() as u32
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle();

    let map = lines.skip(1).fold(HashMap::new(), |mut map, line| {
        let (key, value) = line.split_once(" = ").unwrap();
        let (left, right) = value.split_once(", ").unwrap();
        map.insert(key, (&left[1..], &right[..3]));
        map
    });

    map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|&current| {
            let mut current = current;
            1 + instructions
                .clone()
                .take_while(|next| {
                    let (left, right) = map[current];
                    current = match next {
                        'L' => left,
                        'R' => right,
                        _ => unreachable!(),
                    };
                    !current.ends_with('Z')
                })
                .count() as u64
        })
        .fold(1, lcm)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

aoc2023::test!(
    "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
",
    6,
    "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
",
    6
);
