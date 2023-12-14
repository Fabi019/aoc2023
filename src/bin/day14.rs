use std::collections::HashMap;

aoc2023::main!("../../assets/day14.txt");

fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' {
                let mut y = y;
                grid[y][x] = '.';
                while y > 0 && grid[y - 1][x] == '.' {
                    y -= 1;
                }
                grid[y][x] = 'O';
            }
        }
    }

    grid.iter().enumerate().fold(0, |acc, (i, line)| {
        let rocks = line.iter().filter(|&&c| c == 'O');
        acc + (grid.len() - i) * rocks.count()
    })
}

fn part2(input: &str) -> usize {
    const TOTAL: i32 = 1000000000;

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut seen = HashMap::new();

    for c in 1..=TOTAL {
        cycle_board(&mut grid);

        if let Some(start) = seen.insert(grid.clone(), c) {
            let dist = c - start;
            let last = start + (TOTAL - start) % dist;
            grid = seen
                .into_iter()
                .find_map(|(k, v)| if v == last { Some(k) } else { None })
                .unwrap();
            break;
        }
    }

    grid.iter().enumerate().fold(0, |acc, (i, line)| {
        let rocks = line.iter().filter(|&&c| c == 'O');
        acc + (grid.len() - i) * rocks.count()
    })
}

fn cycle_board(grid: &mut Vec<Vec<char>>) {
    // Push rocks to the north
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' {
                let mut y = y;
                grid[y][x] = '.';
                while y > 0 && grid[y - 1][x] == '.' {
                    y -= 1;
                }
                grid[y][x] = 'O';
            }
        }
    }
    // Push to the west iterate columns from left to right
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                let mut x = x;
                grid[y][x] = '.';
                while x > 0 && grid[y][x - 1] == '.' {
                    x -= 1;
                }
                grid[y][x] = 'O';
            }
        }
    }
    // Push to the south iterate from bottom to top
    for y in (0..grid.len()).rev() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'O' {
                let mut y = y;
                grid[y][x] = '.';
                while y < grid.len() - 1 && grid[y + 1][x] == '.' {
                    y += 1;
                }
                grid[y][x] = 'O';
            }
        }
    }
    // Push to the east iterate from right to left
    for x in (0..grid[0].len()).rev() {
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                let mut x = x;
                grid[y][x] = '.';
                while x < grid[0].len() - 1 && grid[y][x + 1] == '.' {
                    x += 1;
                }
                grid[y][x] = 'O';
            }
        }
    }
}

// Alternative implementation (shorter, but 2x slower)
// fn cycle_board(grid: &mut Vec<Vec<char>>) {
//     for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
//         let mut moved = true;
//         while moved {
//             moved = false;

//             for y in 0..grid.len() {
//                 for x in 0..grid[0].len() {
//                     if grid[y][x] == 'O' {
//                         grid[y][x] = '.';

//                         let mut y = y as i32;
//                         let mut x = x as i32;

//                         while let Some(c) = grid
//                             .get((y + dy) as usize)
//                             .and_then(|l| l.get((x + dx) as usize))
//                         {
//                             if *c != '.' {
//                                 break;
//                             }
//                             y += dy;
//                             x += dx;
//                             moved = true;
//                         }

//                         grid[y as usize][x as usize] = 'O';
//                     }
//                 }
//             }
//         }
//     }
// }

aoc2023::test!(
    "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
",
    136,
    64
);
