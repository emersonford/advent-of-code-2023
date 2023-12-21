use clap::Parser;

#[derive(Copy, Clone)]
enum Direction {
    FirstCol,
    LastCol,
}

fn calc_next_col(row: &[i64], direction: Direction) -> Vec<i64> {
    if row.iter().all(|val| *val == 0) {
        return vec![0];
    }

    let diffs = row
        .windows(2)
        .map(|vals| {
            let &[fst, snd] = vals else {
                panic!("not possible");
            };

            snd - fst
        })
        .collect::<Vec<_>>();

    let mut recurse = calc_next_col(&diffs, direction);
    match direction {
        Direction::LastCol => {
            recurse.push(*row.last().unwrap() + *recurse.last().unwrap());
        }
        Direction::FirstCol => {
            recurse.insert(0, *row.first().unwrap() - *recurse.first().unwrap());
        }
    }

    recurse
}

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    part2: bool,
}

fn main() {
    let args = Cli::parse();

    let res = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|line| {
            let line_res = calc_next_col(
                &line,
                if args.part2 {
                    Direction::FirstCol
                } else {
                    Direction::LastCol
                },
            );

            eprintln!("{line_res:?}");

            if args.part2 {
                *line_res.first().unwrap()
            } else {
                *line_res.last().unwrap()
            }
        })
        .sum::<i64>();

    println!("{res}");
}
