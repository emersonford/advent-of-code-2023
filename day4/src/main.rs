use std::collections::{HashSet, VecDeque};

use clap::Parser;

fn part2() {
    let mut card_counts = VecDeque::new();

    let res = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (_, after_card) = line.split_once(": ").unwrap();

            let (winning_numbers_str, numbers_on_card_str) = after_card.split_once(" | ").unwrap();

            let winning_numbers = winning_numbers_str
                .split_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<HashSet<_>>();

            let numbers_on_card = numbers_on_card_str
                .split_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<HashSet<_>>();

            let card_count = card_counts.pop_front().unwrap_or(0);
            let num_wins = winning_numbers.intersection(&numbers_on_card).count();

            if num_wins > card_counts.len() {
                card_counts.extend(std::iter::repeat(0).take(num_wins - card_counts.len()));
            }

            for i in 0..num_wins {
                *card_counts.get_mut(i).unwrap() += 1 + card_count;
            }

            card_count + 1
        })
        .sum::<usize>();

    println!("{res}");
}

fn part1() {
    let res = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (_, after_card) = line.split_once(": ").unwrap();

            let (winning_numbers_str, numbers_on_card_str) = after_card.split_once(" | ").unwrap();

            let winning_numbers = winning_numbers_str
                .split_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<HashSet<_>>();

            let numbers_on_card = numbers_on_card_str
                .split_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<HashSet<_>>();

            match winning_numbers.intersection(&numbers_on_card).count() {
                0 => 0,
                more_than_zero => 2_usize.pow((more_than_zero - 1).try_into().unwrap()),
            }
        })
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
