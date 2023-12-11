aoc2023::main!("../../assets/day11.txt");

fn part1(input: &str) -> usize {
    let universe = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    calculate(&universe, 2)
}

fn part2(input: &str) -> usize {
    let universe = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let factor = if cfg!(test) { 100 } else { 1000000 };
    calculate(&universe, factor)
}

fn calculate(universe: &[Vec<char>], factor: usize) -> usize {
    // Collect all galaxies and apply horizontal offset if a line is empty
    let mut galaxies = Vec::new();
    let mut offset = 0;
    #[allow(clippy::needless_range_loop)]
    for y in 0..universe.len() {
        let mut galaxy_in_line = false;
        for x in 0..universe[y].len() {
            if universe[y][x] == '#' {
                galaxies.push((x, y + offset));
                galaxy_in_line = true;
            }
        }
        if !galaxy_in_line {
            offset += factor - 1;
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
            let dx = galaxies[i].0.abs_diff(galaxies[j].0);
            let dy = galaxies[i].1.abs_diff(galaxies[j].1);
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
