use clap::Parser;
use std::str::FromStr;

struct CubeConfiguration {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeConfiguration {
    fn greater_than(&self, rhs: &CubeConfiguration) -> bool {
        self.red > rhs.red || self.green > rhs.green || self.blue > rhs.blue
    }

    fn max_colors(&self, rhs: &CubeConfiguration) -> Self {
        Self {
            red: self.red.max(rhs.red),
            green: self.green.max(rhs.green),
            blue: self.blue.max(rhs.blue),
        }
    }
}

impl FromStr for CubeConfiguration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = CubeConfiguration {
            red: 0,
            green: 0,
            blue: 0,
        };

        for color_amt in s.split(", ") {
            let (num, color) = color_amt.split_once(' ').unwrap();

            match color {
                "red" => {
                    ret.red = num.parse().unwrap();
                }
                "blue" => {
                    ret.blue = num.parse().unwrap();
                }
                "green" => {
                    ret.green = num.parse().unwrap();
                }
                unknown => panic!("unknown color {unknown}"),
            }
        }

        Ok(ret)
    }
}

fn part2() {
    let ret: usize = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (_game_prefix, rest) = line.split_once(": ").unwrap();

            let max_colors = rest
                .split("; ")
                .map(|colors_line| colors_line.parse::<CubeConfiguration>().unwrap())
                .reduce(|acc, e| acc.max_colors(&e))
                .unwrap();

            max_colors.red * max_colors.green * max_colors.blue
        })
        .sum();

    println!("{}", ret);
}

fn part1() {
    let total = CubeConfiguration {
        red: 12,
        green: 13,
        blue: 14,
    };

    let ret: usize = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            let (game_prefix, rest) = line.split_once(": ").unwrap();

            let (_, id_str) = game_prefix.split_once(' ').unwrap();
            let id = id_str.parse::<usize>().unwrap();

            if rest.split("; ").all(|colors_line| {
                let colors: CubeConfiguration = colors_line.parse().unwrap();

                !colors.greater_than(&total)
            }) {
                id
            } else {
                0
            }
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
