use shared;

/// Determines if a number is valid password
///
/// # Arguments
///
/// * num - candidate under test
///
/// # Returns
///
/// * validity result
pub fn is_valid(num: u64) -> bool {
    let nstr: Vec<char> = num.to_string().chars().collect();
    let mut has_repeat = false;
    let mut current = '0';
    for i in 0..nstr.len() {
        if nstr[i] < current {
            return false;
        }
        current = nstr[i];
        if i != nstr.len() - 1 && nstr[i] == nstr[i+1] {
            has_repeat = true;
        }
    }
    has_repeat
}

pub fn is_valid_double_pair(num: u64) -> bool {
    let nstr: Vec<char> = num.to_string().chars().collect();
    let mut has_repeat = false;
    let mut current = '0';
    let mut dup_run: u64 = 0;
    for i in 0..nstr.len() {
        if nstr[i] < current {
            return false;
        }
        current = nstr[i];
        if i != nstr.len() - 1 && nstr[i] == nstr[i+1] {
            if dup_run == 0 {
                dup_run = 2
            } else {
                dup_run += 1;
            }
        } else if dup_run == 2 {
            has_repeat = true;
        } else {
            dup_run = 0;
        }
    }
    has_repeat
}

pub fn part1(start: u64, end: u64) -> u64 {
    (start..=end).filter(|&n| is_valid(n)).count().try_into().unwrap()
}

pub fn part2(start: u64, end: u64) -> u64 {
    (start..=end).filter(|&n| is_valid_double_pair(n)).count().try_into().unwrap()
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn is_valid_works() {
        let input: Vec<(u64, bool)> = vec![
            (12345, false),
            (123444, true),
            (33445, true),
            (443444, false),
        ];

        for tc in input {
            assert_eq!(is_valid(tc.0), tc.1);
        }
    }

    #[test]
    fn is_valid_double_pair_works() {
        let input: Vec<(u64, bool)> = vec![
            (12345, false),
            (123444, false),
            (33445, true),
            (44344, false),
            (122333, true),
        ];

        for tc in input {
            assert_eq!(is_valid_double_pair(tc.0), tc.1, "failed with input: {}", tc.0);
        }
    }
}
