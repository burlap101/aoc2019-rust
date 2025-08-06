use day3::{part1, printer};

fn main() {
    match part1("src/input.txt") {
        Some(answer) => {
            println!("Part1: {}", answer);
        }
        None => {
            eprintln!("Failed to get an answer");
        }
    }
    //printer("src/test.txt");
}
