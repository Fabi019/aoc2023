aoc2023::main!("../../assets/day02.txt");

fn part1(input: &str) -> u32 {
    input.lines().enumerate().fold(0, |acc, (game_n, l)| {
        let (_, draws) = l.split_once(": ").unwrap();

        for draw in draws.split("; ") {
            for cube in draw.split(", ") {
                let (number, color) = cube.split_once(' ').unwrap();
                let number = number.parse::<u32>().unwrap();

                match color {
                    "red" if number > 12 => return acc,
                    "blue" if number > 14 => return acc,
                    "green" if number > 13 => return acc,
                    _ => {}
                }
            }
        }

        acc + game_n as u32 + 1
    })
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |acc, l| {
        let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);
        let (_, draws) = l.split_once(": ").unwrap();

        draws.split("; ").flat_map(|d| d.split(", ")).for_each(|f| {
            let (number, color) = f.split_once(' ').unwrap();
            let number = number.parse::<u32>().unwrap();

            match color {
                "red" if number > max_red => max_red = number,
                "blue" if number > max_blue => max_blue = number,
                "green" if number > max_green => max_green = number,
                _ => {}
            }
        });

        acc + max_red * max_blue * max_green
    })
}

aoc2023::test!(
    "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    8,
    2286
);
