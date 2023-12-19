use clap::Parser;

// x - how long you hold
// y - how long you have
// z - how far you go
// formula: z = (y - x) * x = xy - x^2
//
// by quadratic formula: x = (y ± sqrt(y^2 - 4z)) / 2
//
// or (y - sqrt(y^2 - 4z)) / 2 < x < (y + sqrt(y^2 - 4z)) / 2
// for z < xy - x^2
fn num_options(race_length: usize, distance_to_beat: usize) -> usize {
    let y = race_length as f64;
    let z = distance_to_beat as f64;

    let min_float = (y - f64::sqrt(y.powi(2) - 4.0 * z)) / 2.0;
    let max_float = (y + f64::sqrt(y.powi(2) - 4.0 * z)) / 2.0;

    let min = {
        let ceil = min_float.ceil();

        (ceil as usize) + if ceil == min_float { 1 } else { 0 }
    };

    let max = {
        let floor = max_float.floor();

        (floor as usize) - if floor == max_float { 1 } else { 0 }
    };

    max - min + 1
}

fn part2() {
    let lines = std::io::stdin()
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let &[times_line, distances_line] = &lines.as_slice() else {
        panic!("invalid input");
    };

    let time = times_line
        .strip_prefix("Time: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let distance = distances_line
        .strip_prefix("Distance: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    println!("{}", num_options(time, distance));
}

fn part1() {
    let lines = std::io::stdin()
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    let &[times_line, distances_line] = &lines.as_slice() else {
        panic!("invalid input");
    };

    let times = times_line
        .strip_prefix("Time: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|val| val.parse::<usize>().unwrap());
    let distances = distances_line
        .strip_prefix("Distance: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|val| val.parse::<usize>().unwrap());

    let res = times
        .zip(distances)
        .fold(1, |acc, (time, distance)| acc * num_options(time, distance));

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
