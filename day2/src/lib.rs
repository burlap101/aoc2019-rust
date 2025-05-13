use std::cmp;

#[derive(Debug, PartialEq)]
struct IntCode {
    code: Vec<u64>,
}

impl IntCode {
    /// Constructor for a IntCode
    ///
    /// # Arguments
    ///
    /// * `input` string containing raw intcode
    ///
    /// # Returns
    ///
    /// * initialized intcode object
    ///
    pub fn new(input: String) -> IntCode {
        IntCode {
            code: input
                .split(',')
                .map(|s| s.trim())
                .map(|s| s.parse::<u64>().unwrap())
                .collect(),
        }
    }

    /// Performs next operation starting at `pos`
    ///
    /// # Arguments
    ///
    /// * `pos` within the intcode that points to an opcode
    ///
    /// # Returns
    ///
    /// * true if more operations to continue false if not
    ///
    pub fn perform(&mut self, pos: usize) -> bool {
        let mut poss: Vec<usize> = Vec::new();
        for i in &self.code[pos + 1..cmp::min(pos + 4, self.code.len())] {
            poss.push(*i as usize);
        }
        match self.code[pos] {
            1 => {
                self.add(poss[0], poss[1], poss[2]);
                true
            }
            2 => {
                self.mul(poss[0], poss[1], poss[2]);
                true
            }
            99 => false,
            _ => panic!("non opcode encountered at {pos}"),
        }
    }

    /// Performs addition operation
    ///
    /// # Arguments
    ///
    /// * `pos1` first position within self.code for operand
    /// * `pos2` second position within self.code for operand
    /// * `pos3` position in intcode to store result
    ///
    pub fn add(&mut self, pos1: usize, pos2: usize, pos3: usize) {
        let result = &self.code[pos1] + &self.code[pos2];
        self.code[pos3] = result;
    }

    /// Performs multiplication operation
    ///
    /// # Arguments
    ///
    /// * `pos1` first position within self.code for operand
    /// * `pos2` second position within self.code for operand
    /// * `pos3` position in intcode to store result
    ///
    pub fn mul(&mut self, pos1: usize, pos2: usize, pos3: usize) {
        let result = &self.code[pos1] * &self.code[pos2];
        self.code[pos3] = result;
    }

    /// Performs operations until an answer is found
    ///
    /// # Returns
    ///
    /// * value at index 0 after program completion
    ///
    pub fn execute(&mut self) -> u64 {
        let mut i: usize = 0;
        loop {
            let proceed = &self.perform(i);
            if *proceed {
                i += 4;
                continue;
            }
            break;
        }
        self.code[0]
    }
}


/// Performs all parts necessary for part1
///
/// # Returns
///
/// * value at index 0 after program completion
///
pub fn part1(filename: &str) -> u64 {
    let mut input = shared::ingest_file(filename);
    let mut ic = IntCode::new(input.pop().unwrap());
    ic.code[1] = 12;
    ic.code[2] = 2;
    let result = ic.execute();
    result
}

/// Performs all parts necessary for part2
/// Main objective is to find the values of *noun* and *verb* which
/// are the combination of values in index 1 and 2 respectively that 
/// create 19690720 at position 0 when the intcode is executed
///
/// # Returns
///
/// * 100 * noun + verb
///
pub fn part2(filename: &str) -> u64 {
    let input = shared::ingest_file(filename);
    let mut noun: Option<u64> = None;
    let mut verb: Option<u64> = None;
    let mut found = false;
    for i in 0..100 {
        for j in 0..100 {
            let mut ic = IntCode::new(input[0].clone());
            ic.code[1] = i;
            ic.code[2] = j;
            let result = ic.execute();
            if result == 19690720 {
                found = true;
                noun = Some(i as u64);
                verb = Some(j as u64);
                break;
            }
            if found {
                break;
            }
        };
    };
    match  (noun, verb) {
        (Some(n), Some(v)) => 100 * n + v,
        _ => panic!("encountered some error: Noun = {:?}, Verb = {:?}", noun, verb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("src/test.txt");
        assert_eq!(result, 3500);
    }
}
#[cfg(test)]
mod tests_intcode {
    use super::*;

    #[test]
    fn new_produces_object() {
        let s = String::from("1, 5, 9, 4");
        let expected = IntCode {
            code: vec![1, 5, 9, 4],
        };
        let result = IntCode::new(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn can_manipulate_vals() {
        let s = String::from("1, 5, 9, 4");
        let mut ic = IntCode::new(s);
        ic.code[2] = 0;
        assert_eq!(ic.code, vec![1, 5, 0, 4]);
    }

    #[test]
    fn add_single_op() {
        let mut ic = IntCode {
            code: vec![1, 2, 2, 0],
        };
        ic.add(2, 2, 0);
        assert_eq!(ic.code, vec![4, 2, 2, 0]);
    }

    #[test]
    fn perform_for_single_set_add() {
        let mut ic = IntCode {
            code: vec![1, 2, 2, 0],
        };
        ic.perform(0);
        assert_eq!(ic.code, vec![4, 2, 2, 0])
    }

    #[test]
    fn perform_for_single_set_mul() {
        let mut ic = IntCode {
            code: vec![2, 3, 3, 3],
        };
        ic.perform(0);
        assert_eq!(ic.code, vec![2, 3, 3, 9])
    }

    #[test]
    fn perform_part1_test_step1() {
        let mut ic = IntCode {
            code: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        };
        ic.perform(0);
        assert_eq!(ic.code, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn vector_slicing() {
        let input = vec![5, 6, 7, 8, 9];
        assert_eq!(input[1..3], vec![6, 7]);
    }

    #[test]
    fn few_more_small_programs() {
        struct TestCase {
            input: Vec<u64>,
            expected: Vec<u64>,
        }
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: vec![1, 0, 0, 0, 99],
                expected: vec![2, 0, 0, 0, 99],
            },
            TestCase {
                input: vec![2, 3, 0, 3, 99],
                expected: vec![2, 3, 0, 6, 99],
            },
            TestCase {
                input: vec![2, 4, 4, 5, 99, 0],
                expected: vec![2, 4, 4, 5, 99, 9801],
            },
            TestCase {
                input: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                expected: vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            },
        ];
        for tc in test_cases {
            let mut ic = IntCode {
                code: tc.input
            };
            let result = ic.execute();
            assert_eq!(ic.code, tc.expected);
        }
    }
}
