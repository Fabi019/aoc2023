aoc2023::main!("../../assets/day13.txt");

type Grid = Vec<Vec<bool>>;
fn prepare(input: &str) -> Vec<(Grid, Grid)> {
    input
        .split("\n\n")
        .map(|tile| {
            let rows = tile
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let cols = (0..rows[0].len())
                .map(|x| rows.iter().map(|row| row[x]).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            (rows, cols)
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    let grids = prepare(input);

    grids.into_iter().fold(0, |acc, (rows, cols)| {
        acc + check_symmetry(&rows, 0) * 100 + check_symmetry(&cols, 0)
    })
}

fn part2(input: &str) -> usize {
    let grids = prepare(input);

    grids.into_iter().fold(0, |acc, (rows, cols)| {
        acc + check_symmetry(&rows, 1) * 100 + check_symmetry(&cols, 1)
    })
}

fn check_symmetry(grid: &[Vec<bool>], max_diff: usize) -> usize {
    for i in 0..grid.len() - 1 {
        let mut diffs = 0;

        for (exp, actual) in grid[..i + 1].iter().rev().zip(&grid[i + 1..]) {
            for (a, b) in exp.iter().zip(actual.iter()) {
                if a != b {
                    diffs += 1;
                    if diffs > max_diff {
                        break;
                    }
                }
            }
        }

        if diffs == max_diff {
            return i + 1;
        }
    }
    
    0
}

aoc2023::test!(
    "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
",
    405,
    400
);
