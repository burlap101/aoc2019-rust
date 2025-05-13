use std::str::FromStr;
use std::cmp;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, PartialEq)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Orientation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Orientation::Up),
            "D" => Ok(Orientation::Down),
            "L" => Ok(Orientation::Left),
            "R" => Ok(Orientation::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Direction {
    orientation: Orientation,
    distance: u64,
}

impl Direction {
    fn new(input: &str) -> Direction {
        let mut ch_iter = input.chars();
        let orientation = ch_iter.next().unwrap().to_string().parse::<Orientation>().unwrap();
        let distance = ch_iter.next().unwrap().to_string().parse::<u64>().unwrap();
        Direction {
            orientation,
            distance
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Coord {
    m: i64,
    n: i64
}

impl Coord {
    fn distance(&self) -> u64 {
        (self.m.abs() + self.n.abs()) as u64
    }
}

#[derive(Debug, PartialEq)]
pub struct Wire {
    directions: Vec<Direction>
}

impl Wire {
    fn new(input: String) -> Wire {
        Wire {
            directions: input.split(",").map(|x| Direction::new(x)).collect()
        }
    }

    fn trace(&self) -> impl Iterator<Item = Coord> {
        let mut cur_pos = Coord { m: 0, n: 0 };
        let mv = move |dir: &Direction| -> Vec<Coord> {
            let mut result: Vec<Coord> = Vec::new();
            match dir.orientation {
                Orientation::Up => {
                    let j = cur_pos.n;
                    for i in cur_pos.m..=(cur_pos.m + dir.distance as i64) {
                        result.push(Coord {m: i, n: j});
                    }
                    cur_pos.m += dir.distance as i64;
                },
                Orientation::Down => {
                    let j = cur_pos.n;
                    for i in ((cur_pos.m - dir.distance as i64)..=cur_pos.m).rev() {
                        result.push(Coord {m: i, n: j});
                    }
                    cur_pos.m -= dir.distance as i64;
                },
                Orientation::Left => {
                    let i = cur_pos.m;
                    for j in ((cur_pos.n - dir.distance as i64)..=cur_pos.n).rev() {
                        result.push(Coord {m: i, n: j});
                    }
                    cur_pos.n -= dir.distance as i64;
                },
                Orientation::Right => {
                    let i = cur_pos.m;
                    for j in cur_pos.n..=(cur_pos.n + dir.distance as i64) {
                        result.push(Coord {m: i, n: j});
                    }
                    cur_pos.n += dir.distance as i64;
                }
            }
            result
        };
        self.directions.iter().flat_map(mv)
    }
}

#[derive(Debug, PartialEq)]
pub struct Panel {
    wire_one: Wire,
    wire_two: Wire
}

impl Panel {
    fn determine_cross_overs(&self) -> impl Iterator<Item = Coord> {
        let trace_one: Vec<Coord> = self.wire_one.trace().collect();
        self.wire_two.trace().filter(move |c| trace_one.contains(c) && (c.m != 0 && c.n != 0))
    }
}

pub fn part1(filename: &str) -> Option<i64> {
    let lines = shared::ingest_file(filename);
    let wire_one = Wire::new(lines[0].clone());
    let wire_two = Wire::new(lines[1].clone());

    let panel = Panel { wire_one, wire_two };

    panel.determine_cross_overs().map(|co| co.m + co.n).reduce(|acc, v| cmp::min(v, acc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = part1("src/test.txt").unwrap();
        assert_eq!(actual, 159);
    }
}

#[cfg(test)]
mod tests_wire {
    use super::*;

    #[test]
    fn new_wire_constructed_correctly() {
        struct TestCase {
            input: String,
            expected: Wire,
        }

        let tests: Vec<TestCase> = vec![
            TestCase {
                input: String::from("R8,U5,L5,D3"),
                expected: Wire { directions: vec![
                    Direction {
                        distance: 8,
                        orientation: Orientation::Right,
                    },
                    Direction {
                        distance: 5,
                        orientation: Orientation::Up,
                    },
                    Direction {
                        distance: 5,
                        orientation: Orientation::Left,
                    },
                   Direction {
                        distance: 3,
                        orientation: Orientation::Down,
                    },
                ]},
            },
        ];

        for tc in tests {
            let actual = Wire::new(tc.input);
            assert_eq!(actual, tc.expected);
        }
    }

    #[test]
    fn trace_up_returns_correctly() {
        let dir = Direction {
            distance: 5,
            orientation: Orientation::Up,
        };
        let wire = Wire {
            directions: vec![dir]
        };
        let j = 0;
        let expected: Vec<Coord> = (0..=5).map(|i| Coord {m: i, n: j }).collect();
        let result: Vec<Coord> = wire.trace().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn trace_down_returns_correctly() {
        let dir = Direction {
            distance: 5,
            orientation: Orientation::Down,
        };
        let wire = Wire {
            directions: vec![dir]
        };
        let expected: Vec<Coord> = {
            let mut res: Vec<Coord> = Vec::new();
            let j = 0;
            for i in (-5..=0).rev() {
              res.push(Coord {m: i, n: j});  
            };
            res
        };
        let result: Vec<Coord> = wire.trace().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn trace_left_returns_correctly() {
        let dir = Direction {
            distance: 5,
            orientation: Orientation::Left,
        };
        let wire = Wire {
            directions: vec![dir]
        };
        let expected: Vec<Coord> = {
            let mut res: Vec<Coord> = Vec::new();
            let i = 0;
            for j in (-5..=0).rev() {
              res.push(Coord {m: i, n: j});  
            };
            res
        };
        let result: Vec<Coord> = wire.trace().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn trace_right_returns_correctly() {
        let dir = Direction {
            distance: 5,
            orientation: Orientation::Right,
        };
        let wire = Wire {
            directions: vec![dir]
        };
        let expected: Vec<Coord> = {
            let mut res: Vec<Coord> = Vec::new();
            let i = 0;
            for j in 0..=5 {
              res.push(Coord {m: i, n: j});  
            };
            res
        };
        let result: Vec<Coord> = wire.trace().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn trace_works_with_test_input() {
        let lines = shared::ingest_file("src/test.txt");
        let wire_one = Wire::new(lines[0].clone());
        let trace_one: Vec<Coord> = wire_one.trace().collect();
        assert_eq!(trace_one, vec![Coord{m: 5, n: 6}])
    }
}

#[cfg(test)]
mod tests_panel {
    use super::*;

    #[test]
    fn determine_cross_overs_works() {
        let wire_one = Wire::new(String::from("R8,U5,L5,D3"));
        let wire_two = Wire::new(String::from("U7,R6,D4,L4"));

        let expected = vec![Coord {m: 5, n: 6}, Coord {m: 3, n: 3}];

        let panel = Panel {wire_one, wire_two};
        let actual: Vec<Coord> = panel.determine_cross_overs().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn determine_cross_overs_with_test_data() {
        let lines = shared::ingest_file("src/test.txt");
        let wire_one = Wire::new(lines[0].clone());
        let wire_two = Wire::new(lines[1].clone());
    
        let panel = Panel { wire_one, wire_two };
        let distances: Vec<u64> = panel.determine_cross_overs().map(|co| co.distance()).collect();
        assert_eq!(distances, vec![0]);
    }
}
