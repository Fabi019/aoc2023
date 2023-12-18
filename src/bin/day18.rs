aoc2023::main!("../../assets/day18.txt");

fn part1(input: &str) -> u64 {
    let mut edges = Vec::new();
    let mut lenght = 0;
    let (mut x, mut y) = (0_i64, 0_i64);

    for line in input.lines() {
        let mut inst = line.split(' ');

        let dir = inst.next().unwrap();
        let dist = inst.next().unwrap();
        let dist = dist.parse::<i64>().unwrap();

        match dir {
            "R" => x += dist,
            "L" => x -= dist,
            "U" => y += dist,
            "D" => y -= dist,
            _ => panic!("Unknown direction"),
        }

        lenght += dist as u64;
        edges.push((x, y));
    }

    shoelace(&edges) + 1 + (lenght >> 1)
}

fn part2(input: &str) -> u64 {
    let mut edges = Vec::new();
    let mut lenght = 0;
    let (mut x, mut y) = (0, 0);

    for line in input.lines() {
        let color = line.split(' ').nth(2).unwrap();
        let color = &color[2..color.len() - 1]; // Remove the parentheses and #

        let dist = i64::from_str_radix(&color[..color.len() - 1], 16).unwrap();
        let dir = &color[color.len() - 1..];

        match dir {
            "0" => x += dist,
            "2" => x -= dist,
            "3" => y += dist,
            "1" => y -= dist,
            _ => panic!("Unknown direction"),
        }

        lenght += dist as u64;
        edges.push((x, y));
    }

    shoelace(&edges) + 1 + (lenght >> 1)
}

fn shoelace(points: &[(i64, i64)]) -> u64 {
    let n = points.len();

    let mut area = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += points[i].0 * points[j].1 - points[j].0 * points[i].1;
    }

    area.unsigned_abs() / 2
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
