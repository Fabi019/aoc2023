aoc2023::main!("../../assets/day09.txt");

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let initial = line
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut stack = vec![initial];
            loop {
                let mut all_zero = true;
                let diffs = stack[stack.len() - 1]
                    .windows(2)
                    .map(|ab| ab[1] - ab[0])
                    .inspect(|&d| {
                        if all_zero && d != 0 {
                            all_zero = false;
                        }
                    })
                    .collect::<Vec<_>>();
                if all_zero {
                    break;
                }
                stack.push(diffs); // Don't push last chain of zeros
            }
            stack
                .into_iter()
                .rev()
                .fold(0, |acc, d| acc + d[d.len() - 1])
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let n = line
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let mut stack = vec![n];
            loop {
                let mut all_zero = true;
                let diffs = stack[stack.len() - 1]
                    .windows(2)
                    .map(|ab| ab[1] - ab[0])
                    .inspect(|&d| {
                        if all_zero && d != 0 {
                            all_zero = false;
                        }
                    })
                    .collect::<Vec<_>>();
                if all_zero {
                    break;
                }
                stack.push(diffs); // Don't push last chain of zeros
            }
            stack.into_iter().rev().fold(0, |acc, d| d[0] - acc)
        })
        .sum()
}

aoc2023::test!(
    "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
",
    114,
    2
);
