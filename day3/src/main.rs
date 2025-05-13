use day3::part1;

fn main() {
    match part1("src/input.txt") {
        Some(answer) => {
            println!("Part1: {}", answer);
        }
        None => {
            eprintln!("Failed to get an answer");
        }
    }
}
