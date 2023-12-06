aoc2023::main!("../../assets/day06.txt");

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();

    times
        .split_whitespace()
        .zip(distances.split_whitespace())
        .fold(1, |acc, (time, distance)| {
            let time: u32 = time.parse().unwrap();
            let distance: u32 = distance.parse().unwrap();

            acc * (1..time)
                .map(|t| t * (time - t))
                .filter(|&t| t > distance)
                .count() as u32
        })
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();

    let time = times.replace(' ', "").parse::<u64>().unwrap();
    let distance = distances.replace(' ', "").parse::<u64>().unwrap();

    fn binary_search(time: u64, distance: u64, start: u64, end: u64) -> u64 {
        if end - start <= 1 {
            end
        } else {
            let mid = (start + end) / 2;
            let d = mid * (time - mid);
            if d > distance {
                binary_search(time, distance, start, mid)
            } else {
                binary_search(time, distance, mid, end)
            }
        }
    }

    let first_win = binary_search(time, distance, 1, time / 2);
    time - (first_win << 1) + 1
}

aoc2023::test!(
    "\
Time:      7  15   30
Distance:  9  40  200
",
    288,
    71503
);
