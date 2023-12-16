use std::collections::{HashSet, VecDeque};

aoc2023::main!("../../assets/day16.txt");

fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    count_visited(&bfs(&grid, (0, 0), (1, 0)))
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
        let visited = bfs(&grid, (0, y as isize), (1, 0));
        max_len = max_len.max(count_visited(&visited));

        // Right column
        let visited = bfs(
            &grid,
            (grid[0].len() as isize - 1, grid.len() as isize - 1),
            (-1, 0),
        );
        max_len = max_len.max(count_visited(&visited));
    }

    // Check vertical
    for x in 0..grid[0].len() {
        // Top row
        let visited = bfs(&grid, (x as isize, 0), (0, 1));
        max_len = max_len.max(count_visited(&visited));

        // Bottom row
        let visited = bfs(&grid, (x as isize, grid.len() as isize - 1), (0, -1));
        max_len = max_len.max(count_visited(&visited));
    }

    max_len
}

type Pos = (isize, isize);

fn bfs(grid: &[Vec<char>], start: Pos, start_direction: Pos) -> HashSet<(Pos, Pos)> {
    let mut visited = HashSet::new();
    let mut stack = VecDeque::new();

    stack.push_back((start, start_direction));

    while let Some((pos @ (x, y), direction @ (dx, dy))) = stack.pop_front() {
        if x < 0 || y < 0 || x >= grid[0].len() as isize || y >= grid.len() as isize {
            continue;
        }

        if !visited.insert((pos, direction)) {
            continue;
        }

        match grid[y as usize][x as usize] {
            '|' if dx != 0 => {
                stack.push_back(((x, y + 1), (0, 1)));
                stack.push_back(((x, y - 1), (0, -1)));
            }
            '-' if dy != 0 => {
                stack.push_back(((x - 1, y), (-1, 0)));
                stack.push_back(((x + 1, y), (1, 0)));
            }
            '.' | '|' | '-' => {
                stack.push_back(((x + dx, y + dy), direction));
            }
            c => {
                let (dx, dy) = match c {
                    '/' if dx != 0 => (0, -dx),
                    '/' => (-dy, 0),
                    '\\' if dx != 0 => (0, dx),
                    '\\' => (dy, 0),
                    _ => unreachable!(),
                };
                stack.push_back(((x + dx, y + dy), (dx, dy)));
            }
        }
    }

    visited
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
