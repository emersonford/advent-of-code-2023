use std::{cmp::Ordering, collections::HashMap};

use clap::Parser;

#[derive(Debug)]
struct MapTuple {
    fst_start: usize,
    snd_start: usize,
    length: usize,
}

fn part2() {}

fn part1() {
    let mut maps: HashMap<String, (String, Vec<MapTuple>)> = HashMap::new();

    let lines = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let mut chunks = lines.split(|line| line.is_empty());

    let seeds = {
        let seeds_str = chunks.next().unwrap().first().unwrap();

        seeds_str
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|seed| seed.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    for chunk in chunks {
        let (from, to) = chunk
            .first()
            .unwrap()
            .strip_suffix(" map:")
            .unwrap()
            .split_once("-to-")
            .unwrap();

        let mut tuples = chunk
            .into_iter()
            .skip(1)
            .map(|line| {
                let mut split = line.split(' ').map(|x| x.parse::<usize>().unwrap());

                MapTuple {
                    snd_start: split.next().unwrap(),
                    fst_start: split.next().unwrap(),
                    length: split.next().unwrap(),
                }
            })
            .collect::<Vec<_>>();

        tuples.sort_by_key(|val| val.fst_start);

        maps.insert(from.to_string(), (to.to_string(), tuples));
    }

    let ret = seeds
        .into_iter()
        .map(|seed| {
            let mut curr = ("seed", seed);

            while curr.0 != "location" {
                let entry = maps.get(curr.0).unwrap();

                let new_val = if let Ok(idx) = entry.1.binary_search_by(|val| {
                    if val.fst_start <= curr.1 && curr.1 < val.fst_start + val.length {
                        return Ordering::Equal;
                    }

                    // curr.1.cmp(&val.fst_start)
                    val.fst_start.cmp(&curr.1)
                }) {
                    let tuple_entry = entry.1.get(idx).unwrap();

                    tuple_entry.snd_start + (curr.1 - tuple_entry.fst_start)
                } else {
                    curr.1
                };

                curr = (&entry.0, new_val);
            }

            curr.1
        })
        .min()
        .unwrap();

    println!("{ret}");
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
