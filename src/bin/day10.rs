use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

aoc2023::main!("../../assets/day10.txt");

fn prepare(input: &str, map: &mut Vec<Vec<char>>) -> (usize, usize) {
    map.extend(input.lines().map(|line| line.chars().collect::<Vec<_>>()));

    // Find starting position
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    // Manually set the starting position to a pipe (depends on input)
    map[start.1][start.0] = '|'; // Real input

    start
}

fn part1(input: &str) -> u32 {
    // Parse input into a 2D map
    let mut map = Vec::new();
    let start = prepare(input, &mut map);

    if cfg!(test) {
        map[start.1][start.0] = 'F'; // Test input
    }

    bfs(start, &mut HashSet::new(), &map)
}

#[allow(clippy::needless_range_loop)]
fn part2(input: &str) -> i32 {
    // Parse input into a 2D map
    let mut map = Vec::new();
    let start = prepare(input, &mut map);

    if cfg!(test) {
        map[start.1][start.0] = '7'; // Test input
    }

    let grid_size = map.len() * map[0].len(); // Original size for later

    let mut visited = HashSet::new();
    bfs(start, &mut visited, &map);

    // Remove all cells other than loop path
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !visited.contains(&(x, y)) {
                map[y][x] = ' ';
            }
        }
    }

    // Double coordinates in the map and extend all pipes
    let mut map2 = vec![vec![' '; map[0].len() * 2 + 1]];
    for y in 0..map.len() {
        let mut line = vec![' '];
        let mut below = vec![' '];
        for x in 0..map[y].len() {
            line.push(map[y][x]);
            match map[y][x] {
                '-' | 'F' | 'L' => line.push('-'),
                _ => line.push(' '),
            }
            match map[y][x] {
                '|' | 'F' | '7' => below.push('|'),
                _ => below.push(' '),
            }
            below.push(' ');
        }
        map2.push(line);
        map2.push(below);
    }
    let map = map2;

    let mut outside = HashSet::new();
    flood_fill((0, 0), &mut outside, &map);

    // Count only odd coordinates
    let mut outside_count = 0;
    for y in (1..map.len()).step_by(2) {
        for x in (1..map[0].len()).step_by(2) {
            if outside.contains(&(x, y)) {
                outside_count += 1;
            }
        }
    }

    (grid_size - visited.len() - outside_count) as i32
}

fn bfs(start: (usize, usize), visited: &mut HashSet<(usize, usize)>, map: &[Vec<char>]) -> u32 {
    let mut max_dist = 0;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));

    while let Some((Reverse(dist), (x, y))) = queue.pop() {
        if !visited.insert((x, y)) {
            continue;
        }

        max_dist = max_dist.max(dist);
        
        let current = &map[y][x];
        let new_dist = Reverse(dist + 1);

        // No bounds check needed
        if matches!(current, '|' | 'J' | 'L') {
            queue.push((new_dist, (x, y - 1)));
        }
        if matches!(current, '|' | '7' | 'F') {
            queue.push((new_dist, (x, y + 1)));
        }
        if matches!(current, '-' | '7' | 'J') {
            queue.push((new_dist, (x - 1, y)));
        }
        if matches!(current, '-' | 'L' | 'F') {
            queue.push((new_dist, (x + 1, y)));
        }
    }

    max_dist
}

fn flood_fill(start: (usize, usize), visited: &mut HashSet<(usize, usize)>, map: &[Vec<char>]) {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((x, y)) = queue.pop_front() {
        let current = &map[y][x];

        if current != &' ' || !visited.insert((x, y)) {
            continue;
        }

        if y > 0 {
            queue.push_back((x, y - 1));
        }
        if y < map.len() - 1 {
            queue.push_back((x, y + 1));
        }
        if x > 0 {
            queue.push_back((x - 1, y));
        }
        if x < map[y].len() - 1 {
            queue.push_back((x + 1, y));
        }
    }
}

aoc2023::test!(
    "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
",
    8,
    "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
",
    10
);
