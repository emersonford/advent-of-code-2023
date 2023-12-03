fn main() {
    let ret: usize = std::io::stdin()
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.unwrap();

            let fst = line.chars().find(|chr| chr.is_digit(10)).unwrap();
            let lst = line.chars().rev().find(|chr| chr.is_digit(10)).unwrap();

            format!("{fst}{lst}").parse::<usize>().unwrap()
        })
        .sum();

    println!("{}", ret);
}
