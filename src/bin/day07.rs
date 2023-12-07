use std::collections::HashMap;

aoc2023::main!("../../assets/day07.txt");

fn part1(input: &str) -> u32 {
    const CARDS: &str = "23456789TJQKA";

    let mut hands = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            let hand = hand
                .chars()
                .map(|c| CARDS.chars().position(|x| x == c).unwrap() as u32)
                .collect::<Vec<_>>();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by(|(first_hand, _), (second_hand, _)| {
        let first_counts = occurences(first_hand);
        let second_counts = occurences(second_hand);

        let first_count = first_counts.values().max().unwrap();
        let second_count = second_counts.values().max().unwrap();

        if first_count == second_count {
            if first_counts.len() == second_counts.len() {
                for (first, second) in first_hand.iter().zip(second_hand.iter()) {
                    if first != second {
                        return first.cmp(second);
                    }
                }
                unreachable!("Equal hands")
            } else {
                // Lower count is better
                second_counts.len().cmp(&first_counts.len())
            }
        } else {
            // Higher card is better
            first_count.cmp(second_count)
        }
    });

    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| {
            let bid = bid.parse::<u32>().unwrap();
            acc + bid * (rank as u32 + 1)
        })
}

fn part2(input: &str) -> u32 {
    const CARDS: &str = "J23456789TQKA";

    let mut hands = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            let hand = hand
                .chars()
                .map(|c| CARDS.chars().position(|x| x == c).unwrap() as u32)
                .collect::<Vec<_>>();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by(|(first_hand, _), (second_hand, _)| {
        let mut first_counts = occurences(first_hand);
        let first_jokers = *first_counts.get(&0).unwrap_or(&0);
        first_counts.remove(&0); // Removes jokers

        let mut second_counts = occurences(second_hand);
        let second_jokers = *second_counts.get(&0).unwrap_or(&0);
        second_counts.remove(&0);

        let first_count = *first_counts.values().max().unwrap_or(&0) + first_jokers;
        let second_count = *second_counts.values().max().unwrap_or(&0) + second_jokers;

        if first_count == second_count {
            // If all are jokers length would be 0
            if first_counts.len().max(1) == second_counts.len().max(1) {
                for (first, second) in first_hand.iter().zip(second_hand.iter()) {
                    if first != second {
                        return first.cmp(second);
                    }
                }
                unreachable!("Equal hands")
            } else {
                // Lower count is better
                second_counts.len().cmp(&first_counts.len())
            }
        } else {
            // Higher card is better
            first_count.cmp(&second_count)
        }
    });

    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| {
            let bid = bid.parse::<u32>().unwrap();
            acc + bid * (rank as u32 + 1)
        })
}

fn occurences(hand: &[u32]) -> HashMap<&u32, i32> {
    hand.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card).or_insert(0) += 1;
        acc
    })
}

aoc2023::test!(
    "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
",
    6440,
    5905
);
