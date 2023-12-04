aoc2023::main!("../../assets/day04.txt");

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| {
            let (_, card) = l.split_once(": ").unwrap();
            let (winning, numbers) = card.split_once(" | ").unwrap();

            let winning = winning.split_whitespace().collect::<Vec<_>>();
            let matches = numbers
                .split_whitespace()
                .filter(|n| winning.contains(n))
                .count() as u32;

            if matches > 0 {
                Some(1 << (matches - 1))
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let max = input.lines().count();
    let mut counts = vec![0; max];

    input
        .lines()
        .enumerate()
        .map(|(id, l)| {
            let (_, card) = l.split_once(": ").unwrap();
            let (winning, numbers) = card.split_once(" | ").unwrap();

            let winning = winning.split_whitespace().collect::<Vec<_>>();
            let matches = numbers
                .split_whitespace()
                .filter(|n| winning.contains(n))
                .count();

            let entry = counts[id] + 1;

            #[allow(clippy::needless_range_loop)]
            for i in id..=(id + matches).min(max) {
                counts[i] += entry;
            }

            entry
        })
        .sum()
}

aoc2023::test!(
    "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    13,
    30
);
