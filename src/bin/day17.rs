use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

aoc2023::main!("../../assets/day17.txt");

fn prepare(inpute: &str) -> Vec<Vec<u32>> {
    inpute
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> u32 {
    dijkstra(&prepare(input), 1, 3)
}

fn part2(input: &str) -> u32 {
    dijkstra(&prepare(input), 4, 10)
}

fn dijkstra(grid: &[Vec<u32>], min: isize, max: isize) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();

    let start = (0, 0);
    let end = (grid.len() - 1, grid[0].len() - 1);
    queue.push((Reverse(0), start, (0, 0)));

    while let Some((Reverse(cost), pos @ (x, y), dir)) = queue.pop() {
        if pos == end {
            return cost;
        }

        if !visited.insert((pos, dir)) {
            continue;
        }

        for d @ &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            // Prevent going in the same direction or back
            if d == &dir || (-dx, -dy) == dir {
                continue;
            }

            let mut new_cost = cost;

            for step in 1..=max {
                let x = (x as isize + dx * step) as usize;
                let y = (y as isize + dy * step) as usize;

                if y < grid.len() && x < grid[0].len() {
                    new_cost += grid[y][x];
                    if step >= min {
                        queue.push((Reverse(new_cost), (x, y), (dx, dy)));
                    }
                }
            }
        }
    }

    unreachable!("No path found");
}

aoc2023::test!(
    "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
",
    102,
    94
);
