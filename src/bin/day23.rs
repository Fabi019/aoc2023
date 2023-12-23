use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

aoc2023::main!("../../assets/day23.txt");

fn part1(input: &str) -> usize {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    max_steps_recursive(&map, (1, 0), &HashSet::new())
}

thread_local! {
    static CACHE: RefCell<HashMap<(usize, usize), usize>> = RefCell::new(HashMap::new());
}

fn max_steps_recursive(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    visited: &HashSet<(usize, usize)>,
) -> usize {
    let mut visited = visited.clone();
    let mut steps = 0;
    let (mut x, mut y) = start;

    let end = (map[0].len() - 2, map.len() - 1);

    // Loop until we find a junction
    loop {
        if (x, y) == end {
            return steps;
        }

        visited.insert((x, y));

        match map[y][x] {
            '#' => panic!("Should not happen"),
            // < and ^ don't exist in the input
            '>' => x += 1,
            'v' => y += 1,
            _ => {
                let mut neighbors = Vec::with_capacity(4);

                for &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let x = x as isize + dx;
                    let y = y as isize + dy;

                    if x < 0 || y < 0 || x >= map[0].len() as isize || y >= map.len() as isize {
                        continue;
                    }

                    if map[y as usize][x as usize] == '#'
                        || visited.contains(&(x as usize, y as usize))
                    {
                        continue;
                    }

                    neighbors.push((x as usize, y as usize));
                }

                if neighbors.len() == 1 {
                    let (nx, ny) = neighbors[0];
                    x = nx;
                    y = ny;
                } else {
                    // Split up path at junction
                    let max_path = CACHE.with(|cache| cache.borrow().get(&(x, y)).copied());

                    return steps
                        + max_path.unwrap_or_else(|| {
                            let mut max_path = 0;

                            for neighbor in neighbors {
                                max_path =
                                    max_path.max(max_steps_recursive(map, neighbor, &visited));
                            }

                            CACHE.with(|cache| {
                                cache.borrow_mut().insert((x, y), max_path);
                            });

                            max_path
                        });
                }
            }
        }

        steps += 1;
    }
}

fn part2(input: &str) -> i32 {
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut graph = HashMap::new();
    let mut nodes = HashSet::new();

    let start = (1, 0);
    let end = (map[0].len() - 2, map.len() - 1);

    nodes.insert(start);
    nodes.insert(end);

    // Collects all nodes and their neighbors
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c != &'#' {
                let mut neighbors = Vec::with_capacity(4);

                for &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let x = x as isize + dx;
                    let y = y as isize + dy;

                    if x < 0
                        || y < 0
                        || x >= map[0].len() as isize
                        || y >= map.len() as isize
                        || map[y as usize][x as usize] == '#'
                    {
                        continue;
                    }

                    neighbors.push((x as usize, y as usize));
                }

                if neighbors.len() > 2 {
                    nodes.insert((x, y));
                }

                graph.insert((x, y), neighbors);
            }
        }
    }

    let mut adj = HashMap::new();

    // For each node, find the distance to the next nodes
    for node in &nodes {
        adj.insert(*node, HashMap::new());

        let neighbors = graph.get(node).unwrap();

        // Initial neighbors
        for nb in neighbors {
            let mut steps = 0;
            let mut prev = *node;

            let (mut x, mut y) = *nb;

            // Loop until we find a node
            loop {
                if nodes.contains(&(x, y)) {
                    // Found a node, add the distance to the list
                    adj.get_mut(node).unwrap().insert((x, y), steps);
                    break;
                } else {
                    let neighbors = graph.get(&(x, y)).unwrap();

                    for &nb @ (nx, ny) in neighbors {
                        if nb == prev {
                            continue;
                        }

                        prev = (x, y);

                        x = nx;
                        y = ny;
                        break;
                    }
                }
                steps += 1;
            }
        }
    }

    // let mut max_path = 0;
    // let mut queue = BinaryHeap::new();
    // queue.push((0, start, BTreeSet::new()));

    // // Find the longest path between start and end
    // while let Some((steps, node, mut visited)) = queue.pop() {
    //     if node == end {
    //         max_path = max_path.max(steps);
    //         continue;
    //     }

    //     // Visit each node only once
    //     if !visited.insert(node) {
    //         continue;
    //     }

    //     for (neighbor, d) in &adj[&node] {
    //         queue.push((steps + d + 1, *neighbor, visited.clone()));
    //     }
    // }

    //max_path

    find_recursive(&adj, 0, start, end, &mut HashSet::new())
}

type Adj = HashMap<(usize, usize), HashMap<(usize, usize), i32>>;

fn find_recursive(
    adj: &Adj,
    steps: i32,
    node: (usize, usize),
    end: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> i32 {
    if node == end {
        return steps;
    }

    if !visited.insert(node) {
        return 0;
    }

    let mut max_steps = 0;

    for (neighbor, d) in &adj[&node] {
        let len = find_recursive(adj, steps + d + 1, *neighbor, end, visited);
        max_steps = max_steps.max(len);
    }

    visited.remove(&node);

    max_steps
}

aoc2023::test!(
    "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
    94,
    154
);
