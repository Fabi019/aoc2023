use std::collections::HashMap;

aoc2023::main!("../../assets/day12.txt");

fn part1(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (springs, groups) = line.split_once(' ').unwrap();
        let groups = groups
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let springs = springs.to_owned() + ".";
        let springs = springs.chars().collect::<Vec<_>>();
        acc + get_combinations(&springs, &groups, &mut HashMap::new())
    })
}

fn part2(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (springs, groups) = line.split_once(' ').unwrap();
        let groups = groups
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let groups = groups.repeat(5);
        let springs = (springs.to_owned() + "?").repeat(4) + springs + ".";
        let springs = springs.chars().collect::<Vec<_>>();
        acc + get_combinations(&springs, &groups, &mut HashMap::new())
    })
}

fn get_combinations<'a>(
    springs: &'a [char],
    groups: &'a [usize],
    cache: &mut HashMap<(&'a [usize], &'a [char]), usize>,
) -> usize {
    if groups.is_empty() {
        return if springs.iter().all(|&s| s != '#') {
            1
        } else {
            0
        };
    } else if springs.len() < groups.iter().sum() {
        return 0;
    }

    match springs[0] {
        '.' => get_combinations(&springs[1..], groups, cache),
        c => {
            let current = if let Some(&curr_combinations) = cache.get(&(groups, springs)) {
                curr_combinations
            } else {
                let g = groups[0];
                if springs[..g].iter().all(|&s| s != '.')
                    && (springs.len() > g && springs[g] != '#' || springs.len() <= g)
                {
                    get_combinations(&springs[g + 1..], &groups[1..], cache)
                } else {
                    0
                }
            };

            cache.insert((groups, springs), current);

            if c == '?' {
                current + get_combinations(&springs[1..], groups, cache)
            } else {
                current
            }
        }
    }
}

aoc2023::test!(
    "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
",
    21,
    525152
);
