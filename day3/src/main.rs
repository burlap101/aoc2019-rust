use day3::{part1, part2};

fn main() {
    match part1("src/input.txt") {
        Some(answer) => {
            println!("Part1: {}", answer);
        }
        None => {
            eprintln!("Part1: Failed to get an answer");
        }
    };
    match part2("src/input.txt") {
        Ok(answer) => {
            println!("Part2: {}", answer);
        }
        Err(msg) => {
            eprintln!("Part2: Failed to get an answer with error: {}", msg);
        }
    };
}
