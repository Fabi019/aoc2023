use std::collections::VecDeque;

aoc2023::main!("../../assets/day03.txt");

const OFFSETS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

fn part1(input: &str) -> u32 {
    // Convert input into a 2d grid
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    grid.iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter().enumerate().fold(
                (false, 0, 0),
                |(mut symbol, mut current, mut line_sum), (pos, c)| {
                    if let Some(d) = c.to_digit(10) {
                        current = current * 10 + d;

                        // Check if we have found a symbol yet
                        if !symbol {
                            // Check for a symbol around the digit
                            for (x, y) in OFFSETS {
                                if let Some(c) = grid
                                    .get((i as i32 + y) as usize)
                                    .and_then(|l| l.get((pos as i32 + x) as usize))
                                {
                                    if c != &'.' && !c.is_ascii_digit() {
                                        symbol = true;
                                        break;
                                    }
                                }
                            }
                        }

                        // Check if this is the last digit in the line
                        if symbol && pos == line.len() - 1 {
                            line_sum += current;
                        }
                    } else {
                        if symbol {
                            line_sum += current;
                            symbol = false;
                        }
                        current = 0;
                    }

                    (symbol, current, line_sum)
                },
            )
        })
        .map(|(_, _, line_sum)| line_sum)
        .sum()
}

fn part2(input: &str) -> u32 {
    // Convert input into a 2d grid
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|c| c.1 == &'*')
                .fold(0, |acc, (x, _)| {
                    // List of nearby numbers
                    let mut numbers = Vec::with_capacity(2);

                    // Check around the pixel for a number
                    for (dx, dy) in OFFSETS {
                        if let Some(d) = grid
                            .get((y as i32 + dy) as usize)
                            .and_then(|l| l.get((x as i32 + dx) as usize))
                            .and_then(|c| c.to_digit(10))
                        {
                            let mut number = VecDeque::with_capacity(3);
                            number.push_back(d);

                            // Check to the left and right side of the number
                            for dir in [-1, 1] {
                                let mut cx = x as i32 + dx + dir;
                                while let Some(c) = grid
                                    .get((y as i32 + dy) as usize)
                                    .and_then(|l| l.get(cx as usize))
                                    .and_then(|c| c.to_digit(10))
                                {
                                    match dir {
                                        -1 => number.push_front(c),
                                        1 => number.push_back(c),
                                        _ => unreachable!(),
                                    }
                                    cx += dir;
                                }
                            }

                            // Parse the number
                            let number = number.into_iter().fold(0, |acc, d| acc * 10 + d);

                            if !numbers.contains(&number) {
                                numbers.push(number);
                            }

                            // Exit early if we have found two numbers
                            if numbers.len() == 2 {
                                break;
                            }
                        }
                    }

                    // Calculate gear score
                    if numbers.len() == 2 {
                        acc + numbers[0] * numbers[1]
                    } else {
                        acc
                    }
                })
        })
        .sum()
}

aoc2023::test!(
    "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    4361,
    467835
);
