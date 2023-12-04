use std::str::FromStr;

struct CubeConfiguration {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeConfiguration {
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

        return Ok(ret);
    }
}

fn main() {
    let ret: usize = std::io::stdin()
        .lines()
        .into_iter()
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
