use shared;

/// Converts string into an integer
///
/// # Arguments
///
/// * `s` numerical string
///
/// # Returns
///
/// * converted integer
///
pub fn parse_line(s: &String) -> u64 {
    s.parse::<u64>().unwrap()
}

/// Calculates the module fuel required
///
/// # Arguments
///
/// * `mass` modules mass
///
/// # Returns
///
/// * Fuel required
///
pub fn fuel_required(mass: u64) -> u64 {
    let first = mass / 3;
    if first <= 2 {
        return 0;
    }
    first - 2
}

/// Implements all operations necessary for part1
///
/// # Arguments
///
/// * filename - path of file containing input
///
/// # Returns
///
/// * Answer
///
pub fn part1(filename: &str) -> u64 {
    let lines: Vec<String> = shared::ingest_file(filename);
    lines.iter().map(parse_line).map(fuel_required).sum()
}

/// Recursively finds total fuel mass considering the mass of fuel
/// 
/// # Arguments
///
/// * total - accumulator of total fuel mass
/// * mass - either previously found fuel mass or initial module mass
///
/// # Returns
///
/// * Total fuel mass
///
pub fn recursive_fuel_required(total: u64, mass: u64) -> u64 {
    let fuel_mass = fuel_required(mass);
    if fuel_mass == 0 {
        return total;
    }
    recursive_fuel_required(total + fuel_mass, fuel_mass)
}

/// Implements all operations necessary for part2
///
/// # Arguments
///
/// * filename - path of file containing input
///
/// # Returns
///
/// * Answer
///
pub fn part2(filename: &str) -> u64 {
    let lines: Vec<String> = shared::ingest_file(filename);
    lines.iter().map(parse_line).map(|mass| recursive_fuel_required(0, mass)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: u64,
        expected: u64,
    }

    #[test]
    fn test_parse_line() {
        let result = parse_line(&String::from("654654654"));
        let expected: u64 = 654654654;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fuel_required() {
        let tests: Vec<TestCase> = vec![
            TestCase {
                input: 12,
                expected: 2,
            },
            TestCase {
                input: 14,
                expected: 2,
            },
            TestCase {
                input: 1969,
                expected: 654,
            },
            TestCase {
                input: 100756,
                expected: 33583,
            },
        ];

        for tc in tests {
            assert_eq!(fuel_required(tc.input), tc.expected);
        }
    }

    #[test]
    fn test_recursive_fuel_required() {
        let tests: Vec<TestCase> = vec![
            TestCase {
                input: 14,
                expected: 2,
            },
            TestCase {
                input: 1969,
                expected: 966,
            },
            TestCase {
                input: 100756,
                expected: 50346,
            },
        ];
        
        for tc in tests {
            assert_eq!(recursive_fuel_required(0, tc.input), tc.expected);
        }
    }
}
