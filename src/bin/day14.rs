use std::collections::HashMap;

aoc2023::main!("../../assets/day14.txt");

fn part1(input: &str) -> u32 {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Go through each row and push the round rocks as far north as possible
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

    let mut total_load = 0;
    for (i, line) in grid.iter().enumerate() {
        let rock_count = line.iter().filter(|&&c| c == 'O').count();
        let load = (grid.len() - i) * rock_count;
        total_load += load;
    }

    total_load as u32
}

fn part2(input: &str) -> u32 {
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

    let mut total_load = 0;
    for (i, line) in grid.iter().enumerate() {
        let rock_count = line.iter().filter(|&&c| c == 'O').count();
        let load = (grid.len() - i) * rock_count;
        total_load += load;
    }

    total_load as u32
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
