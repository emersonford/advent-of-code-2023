use std::collections::HashMap;

use clap::Parser;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    hand: [u32; 5],
}

impl Hand {
    fn from_str(s: &str, j_is_joker: bool) -> Self {
        let hand: [u32; 5] = s
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if j_is_joker {
                        1
                    } else {
                        11
                    }
                }
                'T' => 10,
                other => other.to_digit(10).unwrap(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let (mut hand_counts, num_jokers) = {
            let mut counts = hand.iter().fold(HashMap::new(), |mut acc, x| {
                *acc.entry(*x).or_insert(0) += 1;

                acc
            });

            let num_jokers = counts.remove(&1).unwrap_or(0);

            (counts.into_values().collect::<Vec<_>>(), num_jokers)
        };

        hand_counts.sort_by(|a, b| b.cmp(a));

        if hand_counts.is_empty() {
            hand_counts.push(0);
        }

        hand_counts[0] += num_jokers;

        let hand_type = match *hand_counts {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Hand { hand_type, hand }
    }
}

fn part2() {
    let mut hands = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            (Hand::from_str(hand, true), bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort();

    // dbg!(&hands);

    let res = hands
        .into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum::<usize>();

    println!("{res}");
}

fn part1() {
    let mut hands = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            (Hand::from_str(hand, false), bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort();

    // dbg!(hands);

    let res = hands
        .into_iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum::<usize>();

    println!("{res}");
}

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    part2: bool,
}

fn main() {
    let args = Cli::parse();

    if args.part2 {
        part2();
    } else {
        part1();
    }
}
