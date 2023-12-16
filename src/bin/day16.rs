use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

aoc2023::main!("../../assets/day16.txt");

fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    dfs(&grid, (0, 0), (1, 0), &mut visited);
    count_visited(&visited)
}

fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut max_len = 0;

    // Check horizontal
    for y in 0..grid.len() {
        // Left column
        let mut visited = HashSet::new();
        dfs(&grid, (0, y as isize), (1, 0), &mut visited);
        max_len = max_len.max(count_visited(&visited));

        // Right column
        let mut visited = HashSet::new();
        dfs(
            &grid,
            (grid[0].len() as isize - 1, grid.len() as isize - 1),
            (-1, 0),
            &mut visited,
        );
        max_len = max_len.max(count_visited(&visited));
    }

    // Check vertical
    for x in 0..grid[0].len() {
        // Top row
        let mut visited = HashSet::new();
        dfs(&grid, (x as isize, 0), (0, 1), &mut visited);
        max_len = max_len.max(count_visited(&visited));

        // Bottom row
        let mut visited = HashSet::new();
        dfs(
            &grid,
            (x as isize, grid.len() as isize - 1),
            (0, -1),
            &mut visited,
        );
        max_len = max_len.max(count_visited(&visited));
    }

    max_len
}

type Pos = (isize, isize);

fn dfs(
    grid: &[Vec<char>],
    position: (isize, isize),
    direction: (isize, isize),
    visited: &mut HashSet<(Pos, Pos)>,
) {
    let pos @ (x, y) = position;
    let (dx, dy) = direction;

    if x < 0 || y < 0 || x >= grid[0].len() as isize || y >= grid.len() as isize {
        return;
    }

    if !visited.insert((pos, direction)) {
        return;
    }

    match grid[y as usize][x as usize] {
        '|' if dx != 0 => {
            dfs(grid, (x, y + 1), (0, 1), visited);
            dfs(grid, (x, y - 1), (0, -1), visited);
        }
        '-' if dy != 0 => {
            dfs(grid, (x - 1, y), (-1, 0), visited);
            dfs(grid, (x + 1, y), (1, 0), visited);
        }
        '.' | '|' | '-' => dfs(grid, (x + dx, y + dy), direction, visited),
        c => {
            let (dx, dy) = match c {
                '/' if dx != 0 => (0, -dx),
                '/' => (-dy, 0),
                '\\' if dx != 0 => (0, dx),
                '\\' => (dy, 0),
                _ => unreachable!(),
            };
            dfs(grid, (x + dx, y + dy), (dx, dy), visited)
        }
    }
}

fn count_visited(visited: &HashSet<(Pos, Pos)>) -> usize {
    let visited = visited.iter().map(|(p, _)| p).collect::<HashSet<_>>();
    visited.len()
}

aoc2023::test!(
    r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#,
    46,
    51
);
