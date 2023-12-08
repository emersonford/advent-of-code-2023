use std::collections::{HashMap, HashSet};

use clap::Parser;
use regex::Regex;

fn part2() {
    let num_re = Regex::new("([0-9]+)").unwrap();

    let lines = std::io::stdin()
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let num_lines = lines.len();

    let mut gear_locations = lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            let row = row;

            line.chars()
                .enumerate()
                .flat_map(move |(col, char)| (char == '*').then(|| ((row, col), Vec::new())))
        })
        .collect::<HashMap<(usize, usize), Vec<usize>>>();

    for (line_num, line) in lines.iter().enumerate() {
        for cap in num_re.captures_iter(line) {
            let cap = cap.get(0).unwrap();

            let num = cap.as_str().parse::<usize>().unwrap();

            // Technically this goes 1 column over but it's fine because there's no symbols out
            // there.
            let end = cap.end();

            for row_col in [
                (line_num > 0).then(|| line_num - 1),
                (line_num + 1 < num_lines).then(|| line_num + 1),
            ]
            .into_iter()
            .flatten()
            .flat_map(|l| {
                ((cap.start().max(1) - 1)..=end).map({
                    let l = l;
                    move |col| (l, col)
                })
            })
            .chain(
                [
                    (cap.start() > 0).then(|| (line_num, cap.start() - 1)),
                    Some((line_num, end)),
                ]
                .into_iter()
                .flatten(),
            ) {
                gear_locations.entry(row_col).and_modify(|e| e.push(num));
            }
        }
    }

    println!(
        "{}",
        gear_locations
            .into_values()
            .map(|val| {
                if let &[one, two] = val.as_slice() {
                    one * two
                } else {
                    0
                }
            })
            .sum::<usize>()
    );
}

fn part1() {
    let num_re = Regex::new("([0-9]+)").unwrap();

    let lines = std::io::stdin()
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let num_lines = lines.len();

    let symbol_locations = lines
        .iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .flat_map(|(idx, char)| (!char.is_digit(10) && char != '.').then_some(idx))
                .collect()
        })
        .collect::<Vec<HashSet<usize>>>();

    let mut sum = 0;

    for (line_num, line) in lines.iter().enumerate() {
        for cap in num_re.captures_iter(line) {
            let cap = cap.get(0).unwrap();

            let num = cap.as_str().parse::<usize>().unwrap();

            // Technically this goes 1 column over but it's fine because there's no symbols out
            // there.
            let end = cap.end();

            if [
                (line_num > 0).then(|| line_num - 1),
                (line_num + 1 < num_lines).then(|| line_num + 1),
            ]
            .into_iter()
            .flatten()
            .flat_map(|l| {
                ((cap.start().max(1) - 1)..=end).map({
                    let l = l;
                    move |col| (l, col)
                })
            })
            .chain(
                [
                    (cap.start() > 0).then(|| (line_num, cap.start() - 1)),
                    Some((line_num, end)),
                ]
                .into_iter()
                .flatten(),
            )
            .any(|(row, col)| symbol_locations[row].contains(&col))
            {
                sum += num;
            }
        }
    }

    println!("{sum}");
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
