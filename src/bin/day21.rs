use std::collections::{HashSet, VecDeque};

aoc2023::main!("../../assets/day21.txt");

fn part1(input: &str) -> usize {
    let mut map = Vec::new();
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.char_indices() {
            if c == 'S' {
                start = (x, y);
                row.push('.');
            } else {
                row.push(c);
            }
        }
        map.push(row);
    }

    let max_steps = if cfg!(test) { 6 } else { 64 };

    make_steps(&map, start, max_steps, false)
}

fn make_steps(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    max_steps: usize,
    infinite_map: bool,
) -> usize {
    let start = (start.0 as isize, start.1 as isize);

    let mut queue = VecDeque::new();
    queue.push_back(vec![start]);

    let mut step = 0;
    let mut last_count = 0;

    // Iterate through all steps
    while let Some(positions) = queue.pop_front() {
        if step > max_steps {
            break;
        }

        let mut visited = HashSet::new();
        let mut new_positions = Vec::new();

        for pos in positions {
            let (x, y) = pos;

            let mut ny = y % map.len() as isize;
            let mut nx = x % map[0].len() as isize;

            if ny < 0 {
                ny += map.len() as isize;
            }

            if nx < 0 {
                nx += map[0].len() as isize;
            }

            if map[ny as usize][nx as usize] == '#' {
                continue;
            }

            if !visited.insert(pos) {
                continue;
            }

            if infinite_map || x > 0 {
                new_positions.push((x - 1, y));
            }
            if infinite_map || y > 0 {
                new_positions.push((x, y - 1));
            }
            if infinite_map || x < map[y as usize].len() as isize - 1 {
                new_positions.push((x + 1, y));
            }
            if infinite_map || y < map.len() as isize - 1 {
                new_positions.push((x, y + 1));
            }
        }

        //println!("Step {}: {}", step, visited.len());
        last_count = visited.len();

        queue.push_back(new_positions);
        step += 1;
    }

    last_count
}

fn part2(input: &str) -> usize {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.char_indices() {
            if c == 'S' {
                start = (x, y);
                row.push('.');
            } else {
                row.push(c);
            }
        }
        map.push(row);
    }

    // Does not work in test mode
    if cfg!(test) {
        return 0;
    }

    let max_steps = 26501365;

    let size = map.len();
    let half = size / 2;

    println!("Size: {size}, Half: {half}");

    let points = [0, 1, 2].map(|x| {
        (
            x as f64,
            make_steps(&map, start, half + size * x, true) as f64,
        )
    });
    let target = (max_steps - half) / size;

    println!("Points: {:?}", points);
    println!("Target: {}", target);

    interpolate(&points, target as f64) as usize
}

fn interpolate(points: &[(f64, f64); 3], x: f64) -> f64 {
    let (x1, y1) = points[0];
    let (x2, y2) = points[1];
    let (x3, y3) = points[2];

    // Calculate coefficients a, b, c
    let a = ((y3 - y2) / ((x3 - x2) * (x3 - x1))) - ((y2 - y1) / ((x2 - x1) * (x3 - x1)));
    let b = (y2 - y1) / (x2 - x1) - a * (x1 + x2);
    let c = y1 - a * x1 * x1 - b * x1;

    // Calculate the interpolated value
    a * x * x + b * x + c
}

aoc2023::test!(
    "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
",
    16,
    0
);
