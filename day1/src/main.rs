use clap::Parser;
use regex::Regex;

fn word_to_num(word: &str) -> usize {
    match word {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        unknown => panic!("unknown number word {unknown}"),
    }
}

fn part2() {
    let fst_re = Regex::new("^.*?(one|two|three|four|five|six|seven|eight|nine|[0-9]).*$").unwrap();
    let lst_re = Regex::new("^.*(one|two|three|four|five|six|seven|eight|nine|[0-9]).*?$").unwrap();

    let ret: usize = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let fst = word_to_num(fst_re.captures(&line).unwrap().get(1).unwrap().as_str());
            let lst = word_to_num(lst_re.captures(&line).unwrap().get(1).unwrap().as_str());

            fst * 10 + lst
        })
        .sum();

    println!("{}", ret);
}

fn part1() {
    let ret: usize = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let fst = line.chars().find(|chr| chr.is_ascii_digit()).unwrap();
            let lst = line.chars().rev().find(|chr| chr.is_ascii_digit()).unwrap();

            format!("{fst}{lst}").parse::<usize>().unwrap()
        })
        .sum();

    println!("{}", ret);
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
