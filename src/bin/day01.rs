use std::collections::BTreeMap;

static INPUT: &str = include_str!("../../assets/day01.txt");

fn main() {
    let now = std::time::Instant::now();
    println!("Part 1: {}", part1(INPUT));
    println!("Took: {:?}", now.elapsed());

    let now = std::time::Instant::now();
    println!("Part 2: {}", part2(INPUT));
    println!("Took: {:?}", now.elapsed());
}

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
    const NUMBER_STRINGS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input.lines().fold(0, |acc, line| {
        let mut numbers = BTreeMap::new();

        // Insert digits as they are
        for (pos, c) in line.char_indices() {
            if let Some(v) = c.to_digit(10) {
                numbers.insert(pos, v);
            }
        }

        // Convert words to digits
        for (value, word) in NUMBER_STRINGS.iter().enumerate() {
            for (pos, _) in line.match_indices(word) {
                numbers.insert(pos, value as u32);
            }
        }

        let numbers = numbers.values().cloned().collect::<Vec<_>>();
        let (first, last) = (numbers[0], numbers[numbers.len() - 1]);
        acc + first * 10 + last
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE1), 142);
    }

    const EXAMPLE2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE2), 281);
    }
}
