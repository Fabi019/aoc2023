use std::{
    collections::{HashMap, HashSet},
    hash::{DefaultHasher, Hash, Hasher},
};

aoc2023::main!("../../assets/day12.txt");

fn part1(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (springs, groups) = line.split_once(' ').unwrap();
        let groups = groups
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let springs = springs.to_owned() + ".";
        let springs = springs.chars().collect::<Vec<_>>();
        acc + check_iterative(&springs, &groups)
    })
}

fn check_iterative(groups: &[char], damaged: &[u32]) -> usize {
    let mut done = HashSet::new();
    let mut stack: Vec<(Vec<char>, usize)> = Vec::new();
    let mut result = 0;

    stack.push((groups.to_vec(), 0));

    while let Some((mut current_groups, index)) = stack.pop() {
        if index == current_groups.len() {
            let mut hasher = DefaultHasher::new();
            Hash::hash(&current_groups, &mut hasher);

            if !done.insert(hasher.finish()) {
                continue; // Already checked
            }

            if validate(&current_groups, damaged) {
                result += 1;
            }

            continue;
        }

        let next = current_groups[index];

        if next == '?' {
            current_groups[index] = '#';
            stack.push((current_groups.clone(), index + 1));

            current_groups[index] = '.';
            stack.push((current_groups.clone(), index + 1));
        } else {
            stack.push((current_groups, index + 1));
        }
    }

    result
}

fn validate(groups: &[char], damaged: &[u32]) -> bool {
    let mut valid = true;
    let mut group_lenght = 0;
    let mut group_index = 0;
    for c in groups {
        if c == &'#' {
            if group_lenght == 0 {
                group_index += 1;
            }
            group_lenght += 1;
        } else if group_lenght > 0 {
            if group_index > damaged.len() || group_lenght != damaged[group_index - 1] {
                valid = false;
                break;
            }
            group_lenght = 0;
        }
    }
    if group_index != damaged.len() {
        valid = false;
    }
    //println!("{groups:?} -> {valid}");
    valid
}

// fn get_combinations<'a>(
//     springs: &'a [char],
//     groups: &'a [usize],
//     cache: &mut HashMap<(&'a [usize], &'a [char]), usize>,
// ) -> usize {
//     if springs.is_empty() {
//         return if groups.is_empty() { 1 } else { 0 };
//     }

//     match springs[0] {
//         '.' => get_combinations(&springs[1..], groups, cache),
//         '#' => {
//             if let Some(&curr_combinations) = cache.get(&(groups, springs)) {
//                 return curr_combinations;
//             }

//             if groups.is_empty() {
//                 return 0;
//             }

//             let wanted_spring_len = groups[0];
//             if springs.len() < wanted_spring_len {
//                 return 0;
//             }

//             for spring in &springs[0..wanted_spring_len] {
//                 if spring == &'.' {
//                     return 0;
//                 }
//             }

//             if springs.len() == wanted_spring_len {
//                 if groups.len() == 1 {
//                     return 1;
//                 }
//                 return 0;
//             }

//             if springs[wanted_spring_len] == '#' {
//                 return 0;
//             }

//             let combinations = get_combinations(
//                 &springs[(wanted_spring_len + 1)..],
//                 &groups[1..],
//                 cache,
//             );

//             cache.insert((groups, springs), combinations);

//             combinations
//         }
//         '?' => {
//             let next = get_combinations(&springs[1..], groups, cache);

//             if let Some(&curr_combinations) = cache.get(&(groups, springs)) {
//                 return curr_combinations + next;
//             }

//             if groups.is_empty() {
//                 return next;
//             }

//             let target_len = groups[0];
//             if springs.len() < target_len {
//                 return next;
//             }

//             for spring in &springs[0..target_len] {
//                 if spring == &'.' {
//                     return next;
//                 }
//             }
//             if springs.len() == target_len {
//                 if groups.len() == 1 {
//                     return 1 + next;
//                 }

//                 return next;
//             }

//             if springs[target_len] == '#' {
//                 return next;
//             }

//             let combinations = get_combinations(
//                 &springs[(target_len + 1)..],
//                 &groups[1..],
//                 cache,
//             );

//             cache.insert((groups, springs), combinations);

//             combinations + next
//         }
//         _ => unreachable!(),
//     }
// }

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
