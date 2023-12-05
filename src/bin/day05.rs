#![feature(test)]

aoc2023::main!("../../assets/day05.txt");

fn part1(input: &str) -> u64 {
    let mut input = input.split("\n\n");
    let mut seeds = input.next().unwrap()[7..]
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for map in input {
        let mut mapping = map.lines();

        println!("{}", mapping.next().unwrap());

        let ranges = mapping
            .map(|l| {
                let mut range = l.split_whitespace();
                let target = range.next().unwrap().parse::<u64>().unwrap();
                let lower = range.next().unwrap().parse::<u64>().unwrap();
                let lenght = range.next().unwrap().parse::<u64>().unwrap();
                (target, lower, lower + lenght)
            })
            .collect::<Vec<_>>();

        seeds.iter_mut().for_each(|seed| {
            for &(target, lower, upper) in &ranges {
                if *seed >= lower && *seed < upper {
                    *seed += target - lower;
                    break;
                }
            }
        });
    }

    *seeds.iter().min().unwrap()
}

fn part2(input: &str) -> u64 {
    let mut input = input.split("\n\n");
    let mut seeds = input.next().unwrap()[7..]
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut new_seeds = Vec::new();
    for range in seeds.chunks_exact(2).clone() {
        let start = range[0];
        let length = range[1];
        new_seeds.extend(start..start + length);
    }
    seeds = new_seeds;

    for map in input {
        let mut mapping = map.lines();

        println!("{}", mapping.next().unwrap());

        let ranges = mapping
            .map(|l| {
                let mut range = l.split_whitespace();
                let target = range.next().unwrap().parse::<u64>().unwrap();
                let lower = range.next().unwrap().parse::<u64>().unwrap();
                let lenght = range.next().unwrap().parse::<u64>().unwrap();
                (target, lower, lower + lenght)
            })
            .collect::<Vec<_>>();

        seeds.iter_mut().for_each(|seed| {
            for &(target, lower, upper) in &ranges {
                if *seed >= lower && *seed < upper {
                    *seed = *seed - lower + target;
                    break;
                }
            }
        });
    }

    *seeds.iter().min().unwrap()
}

aoc2023::test!(
    "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
    35,
    46
);
