aoc2023::main!("../../assets/day01.txt");

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)))
        .fold(0, |acc, mut numbers| {
            let first = numbers.next().unwrap();
            let last = numbers.last().unwrap_or(first);
            acc + first * 10 + last
        })
}

fn part2(input: &str) -> u32 {
    const NUMBER_STRINGS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|l| {
            l.char_indices().filter_map(|(pos, c)| {
                if let Some(v) = c.to_digit(10) {
                    Some(v)
                } else {
                    for (value, word) in NUMBER_STRINGS.iter().enumerate() {
                        if l[pos..].starts_with(word) {
                            return Some(value as u32 + 1);
                        }
                    }
                    None
                }
            })
        })
        .fold(0, |acc, mut numbers| {
            let first = numbers.next().unwrap();
            let last = numbers.last().unwrap_or(first);
            acc + first * 10 + last
        })
}

aoc2023::test!(
    "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
    142,
    "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
    281
);
