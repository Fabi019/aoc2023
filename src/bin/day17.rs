use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

aoc2023::main!("../../assets/day17.txt");

fn part1(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    dijkstra2(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1), 1, 3) as u32
}

fn part2(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    dijkstra2(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1), 4, 10) as u32
}

fn dijkstra(
    grid: &[Vec<u32>],
    start: (usize, usize),
    end: (usize, usize),
    min: isize,
    max: isize,
) -> u32 {
    let mut costs = HashMap::new();
    let mut queue = BinaryHeap::new();

    queue.push(Reverse((0, start, vec![start], (0, 0))));
    while let Some(Reverse((c, (x, y), path, dir))) = queue.pop() {
        if x >= grid[0].len() || y >= grid.len() {
            continue;
        }

        if (x, y) == end {
            // Visualize path in grid
            let grid = grid.to_vec();
            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    if path.contains(&(x, y)) {
                        print!("*");
                    } else {
                        print!("{}", grid[y][x]);
                    }
                }
                println!();
            }

            return c;
        }

        // Check for shorter path to this position
        if let Some(&cc) = costs.get(&((x, y), dir)) {
            if c > cc {
                continue;
            }
        }
        //costs.insert(((x, y), dir), c);

        //println!("Current: {}, {}, cost {c}", x, y);

        for d @ (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // Prevent going back and in the same direction
            if
            /*d == dir || */
            (-dx, -dy) == dir {
                continue;
            }

            if x == 0 && dx == -1 || y == 0 && dy == -1 {
                continue;
            }

            let allowed = path.len() < 4
                || !path.iter().rev().take(4).all(
                    |(lx, ly)| {
                        if dx != 0 {
                            ly == &y
                        } else {
                            lx == &x
                        }
                    },
                );

            if !allowed {
                continue;
            }

            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx >= grid[0].len() || ny >= grid.len() {
                continue;
            }

            let next_cost = c + grid[ny][nx];
            let key = (nx, ny);
            if next_cost < *costs.get(&(key, d)).unwrap_or(&u32::MAX) {
                //println!("{} {:?}", next_cost, key);
                costs.insert((key, d), next_cost);
                let mut path = path.clone();
                path.push(key);
                queue.push(Reverse((next_cost, key, path, d)));
            }
        }

        //     let key @ (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
        //     if nx >= grid[0].len() || ny >= grid.len() {
        //         continue;
        //     }

        //     //let allowed = true;
        //     let allowed = path.len() < 4
        //         || !path.iter().rev().take(4).all(
        //             |(lx, ly)| {
        //                 if dx != 0 {
        //                     ly == &y
        //                 } else {
        //                     lx == &x
        //                 }
        //             },
        //         );

        //     println!("{dx} {dy}: {allowed}");

        //     if allowed {
        //         let next_cost = c + grid[ny][nx];

        //         if next_cost < *costs.get(&(key, d)).unwrap_or(&10000000) {
        //             //println!("{} {:?}", next_cost, key);
        //             costs.insert((key, d), next_cost);
        //             let mut path = path.clone();
        //             path.push(key);
        //             queue.push(Reverse((next_cost, key, path, d)));
        //         }
        //     }
        // }

        // if path.len() < 4 || !path.iter().rev().take(4).all(|(_, ly)| ly == &y) {
        //     if x > 0 && dir != (1, 0) {
        //         let mut path = path.clone();
        //         path.push((x - 1, y));
        //         queue.push(Reverse((c + grid[y][x - 1], (x - 1, y), path, (-1, 0))));
        //     }
        //     if x < grid[0].len() - 1 && dir != (-1, 0) {
        //         let mut path = path.clone();
        //         path.push((x + 1, y));
        //         queue.push(Reverse((c + grid[y][x + 1], (x + 1, y), path, (1, 0))));
        //     }
        // }
        // if path.len() < 4 || !path.iter().rev().take(4).all(|(lx, _)| lx == &x) {
        //     if y > 0 && dir != (0, 1) {
        //         let mut path = path.clone();
        //         path.push((x, y - 1));
        //         queue.push(Reverse((c + grid[y - 1][x], (x, y - 1), path, (0, -1))));
        //     }
        //     if y < grid.len() - 1 && dir != (0, -1) {
        //         let mut path = path.clone();
        //         path.push((x, y + 1));
        //         queue.push(Reverse((c + grid[y + 1][x], (x, y + 1), path, (0, 1))));
        //     }
        // }
    }

    unreachable!("No  path found");
}

// fn shortest_path(grid: &[Vec<u32>], minstep: isize, maxstep: isize, end: (usize, usize)) -> i64 {
//     let mut dists = HashMap::new();
//     let mut q = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);
//     while let Some((cost, (r, c, d))) = q.pop() {
//         if (r, c) == end {
//             return -cost;
//         }
//         if let Some(&c) = dists.get(&(r, c, d)) {
//             if -cost > c {
//                 continue;
//             }
//         }
//         for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
//             if (dr, dc) == d || (-dr, -dc) == d {
//                 continue;
//             }
//             let mut extra_cost = 0;
//             for dist in 1..=maxstep {
//                 let rr = (r as isize + dr * dist) as usize;
//                 let cc = (c as isize + dc * dist) as usize;
//                 if rr >= grid.len() || cc >= grid[0].len() {
//                     continue;
//                 }
//                 extra_cost += grid[rr][cc] as i64;
//                 if dist < minstep {
//                     continue;
//                 }
//                 let next_cost = -cost + extra_cost;
//                 let key = (rr, cc, (dr, dc));
//                 if next_cost < *dists.get(&key).unwrap_or(&10000000) {
//                     dists.insert(key, next_cost);
//                     q.push((-next_cost, key));
//                 }
//             }
//         }
//     }
//     unreachable!()
// }

fn dijkstra2(
    grid: &[Vec<u32>],
    start: (usize, usize),
    end: (usize, usize),
    min: isize,
    max: isize,
) -> u32 {
    let mut costs = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start, (0, 0)));

    while let Some((Reverse(cost), pos @ (x, y), dir)) = queue.pop() {
        if pos == end {
            return cost;
        }

        if !costs.insert((pos, dir)) {
            continue;
        }

        for d @ &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
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
