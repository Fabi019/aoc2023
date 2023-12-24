use std::ops::Add;
use std::ops::Mul;
use z3::ast::Ast;
use z3::ast::Int;
use z3::{Config, Context, Solver};

aoc2023::main!("../../assets/day24.txt");

type Point3D = (i64, i64, i64);
type Point2D = (f64, f64);
type Line = (Point2D, Point2D);

fn prepare(input: &str) -> Vec<(Point3D, Point3D)> {
    input
        .lines()
        .map(|l| {
            let (pos, vel) = l.split_once(" @ ").unwrap();
            let mut pos = pos.split(", ").map(|s| s.trim().parse::<i64>().unwrap());
            let mut vel = vel.split(", ").map(|s| s.trim().parse::<i64>().unwrap());
            (
                (
                    pos.next().unwrap(), // px
                    pos.next().unwrap(), // py
                    pos.next().unwrap(), // pz
                ),
                (
                    vel.next().unwrap(), // vx
                    vel.next().unwrap(), // vy
                    vel.next().unwrap(), // vz
                ),
            )
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    let hails = prepare(input);

    let (min, max) = if cfg!(test) {
        (7_f64, 27_f64)
    } else {
        (200000000000000_f64, 400000000000000_f64)
    };

    let mut count = 0;

    for (i, hail) in hails.iter().enumerate() {
        for other_hail in hails.iter().skip(i + 1) {
            let (pos1, vel1) = hail;
            let (pos2, vel2) = other_hail;

            let line1 = (
                (pos1.0 as f64, pos1.1 as f64),
                (vel1.0 as f64, vel1.1 as f64),
            );
            let line2 = (
                (pos2.0 as f64, pos2.1 as f64),
                (vel2.0 as f64, vel2.1 as f64),
            );

            if let Some((ix, iy)) = intersect(line1, line2) {
                // Check if within bounds
                if ix >= min && ix <= max && iy >= min && iy <= max {
                    // Check if point is already in the past
                    if vel1.0 > 0 && ix < pos1.0 as f64
                        || vel1.0 < 0 && ix > pos1.0 as f64
                        || vel1.1 > 0 && iy < pos1.1 as f64
                        || vel1.1 < 0 && iy > pos1.1 as f64
                        || vel2.0 > 0 && ix < pos2.0 as f64
                        || vel2.0 < 0 && ix > pos2.0 as f64
                        || vel2.1 > 0 && iy < pos2.1 as f64
                        || vel2.1 < 0 && iy > pos2.1 as f64
                    {
                        continue;
                    }
                    count += 1;
                }
            }
        }
    }

    count
}

fn intersect(line1: Line, line2: Line) -> Option<Point2D> {
    let (start1, dir1) = line1;
    let (start2, dir2) = line2;

    let (x1, y1) = start1;
    let (x2, y2) = start2;
    let (v1x, v1y) = dir1;
    let (v2x, v2y) = dir2;

    let det = v1x * v2y - v1y * v2x;

    if det.abs() < f64::EPSILON {
        return None;
    }

    let t = ((x2 - x1) * v2y - (y2 - y1) * v2x) / det;
    Some((x1 + t * v1x, y1 + t * v1y))
}

fn part2(input: &str) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let hailstones = prepare(input);

    // Output variables
    let pxr = Int::new_const(&ctx, "pxr");
    let pyr = Int::new_const(&ctx, "pyr");
    let pzr = Int::new_const(&ctx, "pzr");
    let vxr = Int::new_const(&ctx, "vxr");
    let vyr = Int::new_const(&ctx, "vyr");
    let vzr = Int::new_const(&ctx, "vzr");

    // Take only the first 3 hailstones for calculation
    for (k, ((px, py, pz), (vx, vy, vz))) in hailstones.into_iter().take(3).enumerate() {
        // Different t for each hailstone
        let t = Int::new_const(&ctx, format!("t{}", k).as_str());
        solver.assert(&t.gt(&Int::from_i64(&ctx, 0))); // t > 0

        let px = Int::from_i64(&ctx, px);
        let vx = Int::from_i64(&ctx, vx);
        solver.assert(
            &pxr.clone()
                .add(&t.clone().mul(&vxr))
                ._eq(&px.add(&t.clone().mul(&vx))),
        );

        let py = Int::from_i64(&ctx, py);
        let vy = Int::from_i64(&ctx, vy);
        solver.assert(
            &pyr.clone()
                .add(&t.clone().mul(&vyr))
                ._eq(&py.add(&t.clone().mul(&vy))),
        );

        let pz = Int::from_i64(&ctx, pz);
        let vz = Int::from_i64(&ctx, vz);
        solver.assert(
            &pzr.clone()
                .add(&t.clone().mul(&vzr))
                ._eq(&pz.add(&t.clone().mul(&vz))),
        );
    }

    assert_eq!(solver.check(), z3::SatResult::Sat);

    let model = solver.get_model().unwrap();
    let total = [pxr, pyr, pzr]
        .iter()
        .map(|var| model.eval(var, true).unwrap().as_i64().unwrap())
        .sum();

    total
}

aoc2023::test!(
    "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
",
    2,
    47
);
