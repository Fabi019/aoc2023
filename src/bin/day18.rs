aoc2023::main!("../../assets/day18.txt");

fn part1(input: &str) -> i64 {
    let mut edges = Vec::new();
    let mut map = Vec::new();
    let (mut x, mut y) = (0, 0);

    for line in input.lines() {
        let mut inst = line.split(' ');
        let dir = inst.next().unwrap();
        let dist = inst.next().unwrap();

        for _ in 0..dist.parse::<u32>().unwrap() {
            match dir {
                "R" => x += 1,
                "L" => x -= 1,
                "U" => y += 1,
                "D" => y -= 1,
                _ => panic!("Unknown direction"),
            }
            map.push((x, y));
        }
        edges.push((x, y));
    }

    calculate_polygon_area(&edges) + 1 + map.len() as i64 / 2
}

fn calculate_polygon_area(points: &Vec<(i32, i32)>) -> i64 {
    let n = points.len();

    let mut area: i64 = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += points[i].0 as i64 * points[j].1 as i64 - points[j].0 as i64 * points[i].1 as i64;
    }

    area.abs() / 2
}

fn part2(input: &str) -> i64 {
    let mut edges = Vec::new();
    let mut map = Vec::new();
    let (mut x, mut y) = (0, 0);

    for line in input.lines() {
        let color = line.split(' ').nth(2).unwrap();
        let color = &color[2..color.len() - 1]; // Remove the parentheses and #

        let dist = u64::from_str_radix(&color[..color.len() - 1], 16).unwrap();
        let dir = &color[color.len() - 1..];

        for _ in 0..dist {
            match dir {
                "0" => x += 1,
                "2" => x -= 1,
                "3" => y += 1,
                "1" => y -= 1,
                _ => panic!("Unknown direction"),
            }
            map.push((x, y));
        }
        edges.push((x, y));
    }

    calculate_polygon_area(&edges) + 1 + map.len() as i64 / 2
}

aoc2023::test!(
    "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
",
    62,
    952408144115
);
