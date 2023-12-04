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
    let total = CubeConfiguration {
        red: 12,
        green: 13,
        blue: 14,
    };

    let ret: usize = std::io::stdin()
        .lines()
        .into_iter()
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
