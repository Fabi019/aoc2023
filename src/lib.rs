#[macro_export]
macro_rules! main {
    ($input:expr) => {
        static INPUT: &str = include_str!($input);

        fn main() {
            let now = std::time::Instant::now();
            println!("Part 1: {} ({:?})", part1(INPUT), now.elapsed());

            let now = std::time::Instant::now();
            println!("Part 2: {} ({:?})", part2(INPUT), now.elapsed());
        }
    };
}

#[macro_export]
macro_rules! test {
    ($input:expr, $part1:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part1() {
                assert_eq!(part1($input), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!(part2($input), $part2);
            }
        }
    };
    ($input1:expr, $part1:expr, $input2:expr, $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part1() {
                assert_eq!(part1($input1), $part1);
            }

            #[test]
            fn test_part2() {
                assert_eq!(part2($input2), $part2);
            }
        }
    };
}
