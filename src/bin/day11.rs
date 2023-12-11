aoc2023::main!("../../assets/day11.txt");

fn part1(input: &str) -> u64 {
    let universe = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    calculate(universe, 2)
}

fn part2(input: &str) -> u64 {
    let universe = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let factor = if cfg!(test) { 100 } else { 1000000 };
    calculate(universe, factor)
}

fn calculate(universe: Vec<Vec<char>>, factor: usize) -> u64 {
    // Collect all galaxies
    let mut galaxies = Vec::new();
    for y in 0..universe.len() {
        for x in 0..universe[y].len() {
            if universe[y][x] == '#' {
                galaxies.push((x, y));
            }
        }
    }

    // Apply a horizontal offset when a line is empty
    for y in (0..universe.len()).rev() {
        if universe[y].iter().any(|c| *c != '.') {
            continue;
        }

        for galaxy in &mut galaxies {
            if galaxy.1 > y {
                galaxy.1 += factor - 1;
            }
        }
    }

    // Apply vertical offset if a column is empty
    for x in (0..universe[0].len()).rev() {
        if universe.iter().any(|line| line[x] != '.') {
            continue;
        }

        for galaxy in &mut galaxies {
            if galaxy.0 > x {
                galaxy.0 += factor - 1;
            }
        }
    }

    // Calculate distance between all galaxies
    let mut path_sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dx = (galaxies[i].0 as i64).abs_diff(galaxies[j].0 as i64);
            let dy = (galaxies[i].1 as i64).abs_diff(galaxies[j].1 as i64);
            path_sum += dx + dy;
        }
    }
    path_sum
}

aoc2023::test!(
    "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
",
    374,
    8410
);
