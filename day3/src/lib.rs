use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if *self == Orientation::Vertical {
                "Vertical"
            } else {
                "Horizontal"
            }
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
struct CornerPair(Coord, Coord);

impl Display for CornerPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl CornerPair {
    /// Determines the intersecting point of two intervals
    ///
    /// # Arguments
    ///
    /// * other - the other CornerPair to compare against
    ///
    /// # Returns
    ///
    /// * point of intersection or none
    fn intersection(&self, other: CornerPair) -> Option<Coord> {
        match (self.orientation(), other.orientation()) {
            (Orientation::Horizontal, Orientation::Vertical) => {
                let (x1, x2) = (self.0.x, self.1.x);
                let (y1, y2) = (other.0.y, other.1.y);
                if ((x1.min(x2) + 1)..x1.max(x2)).contains(&other.0.x)
                    && ((y1.min(y2) + 1)..y1.max(y2)).contains(&self.0.y)
                {
                    return Some(Coord {
                        x: other.0.x,
                        y: self.0.y,
                    });
                }
                None
            }
            (Orientation::Vertical, Orientation::Horizontal) => {
                let (y1, y2) = (self.0.y, self.1.y);
                let (x1, x2) = (other.0.x, other.1.x);
                if ((y1.min(y2) + 1)..y1.max(y2)).contains(&other.0.y)
                    && ((x1.min(x2) + 1)..x1.max(x2)).contains(&self.0.x)
                {
                    return Some(Coord {
                        x: self.0.x,
                        y: other.0.y,
                    });
                }
                None
            }
            _ => None,
        }
    }

    fn orientation(&self) -> Orientation {
        let CornerPair(c1, c2) = self;
        if c1.x == c2.x {
            return Orientation::Vertical;
        }
        return Orientation::Horizontal;
    }

    pub fn on_interval(&self, point: Coord) -> bool {
        match self.orientation() {
            Orientation::Vertical => {
                let miny = self.0.y.min(self.1.y);
                let maxy = self.0.y.max(self.1.y);
                self.0.x == point.x && (miny..=maxy).contains(&point.y)
            }
            Orientation::Horizontal => {
                let minx = self.0.x.min(self.1.x);
                let maxx = self.0.x.max(self.1.x);
                self.0.y == point.y && (minx..=maxx).contains(&point.x)
            }
        }
    }

    /// Takes a point and returns char representation
    ///
    /// # Arguments
    ///
    /// * point - location
    ///
    /// # Returns
    ///
    /// * either - | or +
    pub fn char_point(&self, point: Coord, curr: Option<char>) -> char {
        if self.0 == point || self.1 == point {
            return '+';
        }

        match (self.on_interval(point), self.orientation(), curr) {
            (true, Orientation::Vertical, Some('.')) => '|',
            (true, Orientation::Horizontal, Some('.')) => '=',
            (true, Orientation::Vertical, None) => '|',
            (true, Orientation::Horizontal, None) => '=',
            (false, _, Some(c)) => c,
            _ => '.',
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct Command {
    dir: Direction,
    count: u32,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.dir, self.count)
    }
}

impl Command {
    pub fn new(cmd_s: &str) -> Self {
        let (dir_s, count_s) = cmd_s.split_at(1);
        let dir = match dir_s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => {
                panic!("unknown direction char {}", dir_s)
            }
        };
        let count = count_s.parse::<u32>().unwrap();

        Command { dir, count }
    }

    /// Returns the coords of all points when carrying out the command
    ///
    /// Excludes the start coordinate
    ///
    /// # Arguments
    ///
    /// * `start` - Starting location
    ///
    /// # Returns
    ///
    /// iterator of all coords visited, excluding start.
    pub fn coords(&self, start: Coord) -> Box<dyn Iterator<Item = Coord>> {
        match self.dir {
            Direction::Up => Box::new(
                (start.y + 1..=start.y + self.count as i64).map(move |y| Coord { x: start.x, y }),
            ),

            Direction::Down => Box::new(
                (start.y - self.count as i64..start.y)
                    .rev()
                    .map(move |y| Coord { x: start.x, y }),
            ),

            Direction::Left => Box::new(
                (start.x - self.count as i64..start.x)
                    .rev()
                    .map(move |x| Coord { x, y: start.y }),
            ),

            Direction::Right => Box::new(
                (start.x + 1..=start.x + self.count as i64).map(move |x| Coord { x, y: start.y }),
            ),
        }
    }

    pub fn last_coord(&self, start: Coord) -> Coord {
        match self.dir {
            Direction::Up => {
                return Coord {
                    x: start.x,
                    y: start.y + self.count as i64,
                };
            }
            Direction::Down => {
                return Coord {
                    x: start.x,
                    y: start.y - self.count as i64,
                };
            }
            Direction::Left => {
                return Coord {
                    x: start.x - self.count as i64,
                    y: start.y,
                };
            }
            Direction::Right => {
                return Coord {
                    x: start.x + self.count as i64,
                    y: start.y,
                };
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Wire {
    cmds: Vec<Command>,
}

impl Wire {
    pub fn new(cmds_s: &str) -> Self {
        let dirs = cmds_s.split(',').map(Command::new).collect();
        Wire { cmds: dirs }
    }

    /// Takes a collection of cmds and returns all coordinates
    ///
    /// # Arguments
    ///
    /// * `start` - starting coordinate of the trace.
    ///
    /// # Returns
    ///
    /// All coordinates visited
    pub fn trace(&self, start: Coord) -> Vec<Coord> {
        let mut cmd_coords: Vec<Coord> = Vec::new();
        let mut current = start;
        for cmd in &self.cmds {
            let new_coords: Vec<Coord> = cmd.coords(current).collect();
            cmd_coords.extend(&new_coords);
            if let Some(last) = new_coords.last() {
                current = *last;
            }
        }
        cmd_coords
    }

    /// Takes a collection of cmds and returns all corner coordinates
    ///
    /// # Arguments
    ///
    /// * start - starting coordinate of the trace.
    ///
    /// # Returns
    ///
    /// corner coordinates visited
    pub fn trace_corners(&self, start: Coord) -> Vec<CornerPair> {
        let mut cnr_coords: Vec<CornerPair> = Vec::new();
        let mut current = start;
        for cmd in &self.cmds {
            let new_coord: Coord = cmd.last_coord(current);
            cnr_coords.push(CornerPair(current, new_coord));
            current = new_coord;
        }
        cnr_coords
    }

    /// Determines all crossovers with another wire
    ///
    /// # Arguments
    ///
    /// * other - the wire to compare with
    ///
    /// # Returns
    ///
    /// all crossover coordinates
    pub fn crossovers(&self, other: &Wire) -> Vec<Coord> {
        let this_trace_corners: Vec<CornerPair> = self.trace_corners(Coord { x: 0, y: 0 });
        let other_trace_corners: Vec<CornerPair> = other.trace_corners(Coord { x: 0, y: 0 });
        let mut all_crossovers: Vec<Coord> = Vec::new();
        for cpi in this_trace_corners {
            for cpj in &other_trace_corners {
                if let Some(coord) = cpi.intersection(*cpj) {
                    if (cpi.0.y == 0 && cpi.1.y == 0) || (cpj.0.y == 0 && cpj.1.y == 0) {
                        println!("cpi: {}; cpj: {}; coord: {}", cpi, cpj, coord);
                    }
                    all_crossovers.push(coord);
                }
            }
        }
        all_crossovers
    }

    /// Determines the amount of steps taken to reach the given crossover
    ///
    /// # Arguments
    ///
    /// * other - the wire that crossover occurs with
    /// * point - the particular crossover to consider
    ///
    /// # Returns
    ///
    /// steps that it takes the trace to reach that point
    pub fn steps_to_crossover(&self, other: &Wire, point: Coord) -> Result<u64, String> {
        if !self.crossovers(other).contains(&point) {
            return Err(format!("point not a crossover; {}", point));
        }
        let mut current = Coord { x: 0, y: 0 };
        let mut count: u64 = 0;
        for cmd in &self.cmds {
            for coord in cmd.coords(current) {
                count += 1;
                if coord == (point) {
                    return Ok(count);
                }
            }
            current = match cmd.coords(current).last() {
                Some(c) => c,
                None => unreachable!("a last coordinate wasn't returned"),
            };
        }
        
        Err(String::from("the crossover was never reached"))
    }
}

struct Panel(Wire, Wire);

impl Panel {
    fn generate(&self) -> (HashMap<(i64, i64), char>, Coord, Coord) {
        let mut cnrs: Vec<Coord> = self
            .0
            .trace_corners(Coord { x: 0, y: 0 })
            .iter()
            .map(|x| vec![x.0, x.1])
            .flatten()
            .collect();
        cnrs.extend(
            self.1
                .trace_corners(Coord { x: 0, y: 0 })
                .iter()
                .map(|x| vec![x.0, x.1])
                .flatten(),
        );
        let mut min_bounds = Coord { x: 0, y: 0 };
        let mut max_bounds = Coord { x: 0, y: 0 };

        let crossovers = self.0.crossovers(&self.1);

        // Determine bounds of trace
        for cnr in cnrs {
            min_bounds.x = min_bounds.x.min(cnr.x);
            min_bounds.y = min_bounds.y.min(cnr.y);
            max_bounds.x = max_bounds.x.max(cnr.x);
            max_bounds.y = max_bounds.y.max(cnr.y);
        }

        let mut intervals: Vec<CornerPair> = self.0.trace_corners(Coord { x: 0, y: 0 });
        intervals.extend(self.1.trace_corners(Coord { x: 0, y: 0 }));

        let mut display: HashMap<(i64, i64), char> = HashMap::new();
        for i in min_bounds.y..=max_bounds.y {
            for j in min_bounds.x..=max_bounds.x {
                let coord = Coord { x: j, y: i };
                let on_interval = intervals.iter().any(|intvl| intvl.on_interval(coord));
                let ch = if crossovers.contains(&coord) {
                    Some('X')
                } else if on_interval {
                    Some('5')
                } else {
                    Some('.')
                };
                match ch {
                    Some(c) => {
                        display.insert((j, i), c);
                    }
                    None => unreachable!("Found a siuatuion where character wasn't returned"),
                }
            }
        }
        (display, min_bounds, max_bounds)
    }

    pub fn generate_from_trace(&self) -> (HashMap<(i64, i64), char>, Coord, Coord) {
        let mut all_trace: Vec<Coord> = self.1.trace(Coord { x: 0, y: 0 });
        all_trace.extend(self.0.trace(Coord { x: 0, y: 0 }));
        let (_, min_bounds, max_bounds) = self.generate();
        let mut display: HashMap<(i64, i64), char> = HashMap::new();
        let cos = self.0.crossovers(&self.1);
        for i in min_bounds.y..=max_bounds.y {
            for j in min_bounds.x..=max_bounds.x {
                let cd = Coord { x: j, y: i };
                let ch = if all_trace.contains(&cd) {
                    Some('5')
                } else if cos.contains(&cd) {
                    Some('X')
                } else {
                    Some('.')
                };
                match ch {
                    Some(c) => {
                        const ORIGIN: Coord = Coord { x: 0, y: 0 };
                        if cd == ORIGIN {
                            display.insert((j, i), 'O');
                        } else {
                            display.insert((j, i), c);
                        }
                    }
                    None => unreachable!("Found a siuatuion where character wasn't returned"),
                }
            }
        }
        (display, min_bounds, max_bounds)
    }

    pub fn print_panel(&self) {
        let (disp, min_bounds, max_bounds) = self.generate();
        const COL_WIDTH: usize = 1;
        let mut first_row = String::from(" ".repeat(6));
        println!("{} -> {}", min_bounds, max_bounds);
        for j in min_bounds.x..=max_bounds.x {
            first_row = format!("{}{:^width$}", first_row, j % 10, width = COL_WIDTH);
        }
        let mut lines: Vec<String> = Vec::new();
        println!("{}", first_row);
        for i in min_bounds.y..=max_bounds.y {
            let mut line = String::from(format!("{:>5} ", i));
            for j in min_bounds.x..=max_bounds.x {
                let k = (j, i);
                if let Some(ch) = disp.get(&k) {
                    line = format!("{}{:^width$}", line, ch, width = COL_WIDTH);
                } else {
                    unreachable!("character wasn't generated");
                }
            }
            lines.push(line);
        }
        for line in lines.iter().rev() {
            println!("{}", line)
        }
        println!("{}", first_row);
    }
}

pub fn part1(filename: &str) -> Option<i64> {
    let input = shared::ingest_file(filename);
    let wire_one = Wire::new(&input[0]);
    let wire_two = Wire::new(&input[1]);
    wire_one
        .crossovers(&wire_two)
        .into_iter()
        .map(|c| c.x.abs() + c.y.abs())
        .min()
}

/// Performs all operations necessary for part2
///
/// # Arguments
///
/// * filename - name of input file
///
/// # Returns
///
/// * count of steps taken to crossover if successful, or
/// * error message
pub fn part2(filename: &str) -> Result<u64, String> {
    let input = shared::ingest_file(filename);
    let wire_one = Wire::new(&input[0]);
    let wire_two = Wire::new(&input[1]);
    let crossovers = wire_one.crossovers(&wire_two);
    let distances_one = crossovers.iter().map(|co| wire_one.steps_to_crossover(&wire_two, *co));
    let distances_two = crossovers.iter().map(|co| wire_two.steps_to_crossover(&wire_one, *co));

    let res = distances_one.zip(distances_two).map(|(d1, d2)| d1.unwrap() + d2.unwrap()).min();
    
    match res {
        Some(d) => Ok(d),
        None => Err(String::from("no distance returned"))
    }
}

pub fn printer(filename: &str) {
    let input = shared::ingest_file(filename);
    let wire_one = Wire::new(&input[0]);
    let wire_two = Wire::new(&input[1]);
    let panel = Panel(wire_one, wire_two);
    panel.print_panel();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let actual = part1("src/test.txt").unwrap();
        assert_eq!(actual, 159);
    }

    #[test]
    fn part1_works_second_test() {
        let actual = part1("src/test2.txt").unwrap();
        assert_eq!(actual, 135);
    }

    #[test]
    fn part2_works() {
        let actual = part2("src/test.txt").unwrap();
        assert_eq!(actual, 610)
    }

    #[test]
    fn part2_works_second_test() {
        let actual = part2("src/test2.txt").unwrap();
        assert_eq!(actual, 410)
    }
}

#[cfg(test)]
mod test_command {
    use super::*;

    #[test]
    fn command_constructor_works() {
        let input = "U32";
        let expected = Command {
            dir: Direction::Up,
            count: 32,
        };
        let actual = Command::new(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn coords_up_works() {
        let input = Command {
            dir: Direction::Up,
            count: 10,
        };
        let mut expected: Vec<Coord> = Vec::new();
        for i in 1..=10 {
            expected.push(Coord { x: 5, y: 5 + i });
        }
        let actual: Vec<Coord> = input.coords(Coord { x: 5, y: 5 }).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn coords_down_works() {
        let input = Command {
            dir: Direction::Down,
            count: 10,
        };
        let mut expected: Vec<Coord> = Vec::new();
        for i in 1..=10 {
            expected.push(Coord { x: 5, y: 5 - i });
        }
        let mut actual: Vec<Coord> = input.coords(Coord { x: 5, y: 5 }).collect();
        assert_eq!(actual.sort(), expected.sort());
    }

    #[test]
    fn coords_right_works() {
        let input = Command {
            dir: Direction::Right,
            count: 71,
        };
        let mut expected: Vec<Coord> = Vec::new();
        for i in 1..=71 {
            expected.push(Coord { y: 4, x: 146 + i });
        }
        let actual: Vec<Coord> = input.coords(Coord { x: 146, y: 4 }).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn coords_left_works() {
        let input = Command {
            dir: Direction::Left,
            count: 10,
        };
        let mut expected: Vec<Coord> = Vec::new();
        for i in 1..=10 {
            expected.push(Coord { y: 5, x: 5 - i });
        }
        let mut actual: Vec<Coord> = input.coords(Coord { x: 5, y: 5 }).collect();
        assert_eq!(actual.sort(), expected.sort());
    }
}

#[cfg(test)]
mod test_wire {
    use super::*;

    #[test]
    fn wire_constructor_works() {
        let input = "U32,D15,L16,R240";
        let expected = Wire {
            cmds: vec![
                Command {
                    dir: Direction::Up,
                    count: 32,
                },
                Command {
                    dir: Direction::Down,
                    count: 15,
                },
                Command {
                    dir: Direction::Left,
                    count: 16,
                },
                Command {
                    dir: Direction::Right,
                    count: 240,
                },
            ],
        };

        let actual = Wire::new(input);
        assert_eq!(actual, expected)
    }

    #[test]
    fn trace_works() {
        let input: Wire = Wire {
            cmds: vec![
                Command {
                    dir: Direction::Up,
                    count: 7,
                },
                Command {
                    dir: Direction::Left,
                    count: 3,
                },
            ],
        };
        let mut expected: Vec<Coord> = vec![
            Coord { x: 0, y: 1 },
            Coord { x: 0, y: 2 },
            Coord { x: 0, y: 3 },
            Coord { x: 0, y: 4 },
            Coord { x: 0, y: 5 },
            Coord { x: 0, y: 6 },
            Coord { x: 0, y: 7 },
            Coord { x: -1, y: 7 },
            Coord { x: -2, y: 7 },
            Coord { x: -3, y: 7 },
        ];

        let mut actual = input.trace(Coord { x: 0, y: 0 });

        assert_eq!(actual.sort(), expected.sort())
    }

    #[test]
    fn crossovers_works() {
        let wire_one = Wire {
            cmds: vec![
                Command {
                    dir: Direction::Up,
                    count: 7,
                },
                Command {
                    dir: Direction::Left,
                    count: 3,
                },
            ],
        };
        let wire_two = Wire {
            cmds: vec![
                Command {
                    dir: Direction::Left,
                    count: 2,
                },
                Command {
                    dir: Direction::Up,
                    count: 8,
                },
            ],
        };
        let expected: Vec<Coord> = vec![Coord { x: -2, y: 7 }];
        let actual: Vec<Coord> = wire_one.crossovers(&wire_two);

        assert_eq!(actual, expected);
    }

    #[test]
    fn trace_corners_works() {
        let input: Wire = Wire {
            cmds: vec![
                Command {
                    dir: Direction::Up,
                    count: 7,
                },
                Command {
                    dir: Direction::Left,
                    count: 3,
                },
                Command {
                    dir: Direction::Down,
                    count: 14,
                },
            ],
        };
        let mut expected = vec![
            Coord { x: 0, y: 7 },
            Coord { x: 3, y: 7 },
            Coord { x: 3, y: -7 },
        ];

        let mut actual = input.trace_corners(Coord { x: 0, y: 0 });
        assert_eq!(actual.sort(), expected.sort());
    }

    #[test]
    fn trace_corners_works_test_input() {
        let input = shared::ingest_file("src/test.txt");
        let wire_one = Wire::new(&input[0]);
        let actual = wire_one.trace_corners(Coord { x: 0, y: 0 });
        let expected = vec![
            CornerPair(Coord { x: 0, y: 0 }, Coord { x: 75, y: 0 }),
            CornerPair(Coord { x: 75, y: 0 }, Coord { x: 75, y: -30 }),
            CornerPair(Coord { x: 75, y: -30 }, Coord { x: 158, y: -30 }),
            CornerPair(Coord { x: 158, y: -30 }, Coord { x: 158, y: 53 }),
            CornerPair(Coord { x: 158, y: 53 }, Coord { x: 146, y: 53 }),
            CornerPair(Coord { x: 146, y: 53 }, Coord { x: 146, y: 4 }),
            CornerPair(Coord { x: 146, y: 4 }, Coord { x: 217, y: 4 }),
            CornerPair(Coord { x: 217, y: 4 }, Coord { x: 217, y: 11 }),
            CornerPair(Coord { x: 217, y: 11 }, Coord { x: 145, y: 11 }),
        ];
        assert_eq!(actual.len(), 9);
        for (p1, p2) in actual.into_iter().zip(expected) {
            assert_eq!(p1, p2);
        }
    }

    #[test]
    fn steps_to_crossover_works() {
        let wire_one = Wire {
            cmds: vec![
                Command {
                    dir: Direction::Up,
                    count: 7,
                },
                Command {
                    dir: Direction::Left,
                    count: 3,
                },
            ],
        };
        let wire_two = Wire {
            cmds: vec![
                Command {
                    dir: Direction::Left,
                    count: 1,
                },
                Command {
                    dir: Direction::Down,
                    count: 5,
                },
                Command {
                    dir: Direction::Left,
                    count: 1,
                },
                Command {
                    dir: Direction::Up,
                    count: 15,
                },
            ],
        };
        let crossover: Coord = Coord { x: -2, y: 7 };
        let expected_one: Result<u64, String> = Ok(9);
        let expected_two: Result<u64, String> = Ok(19);
        let actual_one = wire_one.steps_to_crossover(&wire_two, crossover);
        let actual_two = wire_two.steps_to_crossover(&wire_one, crossover);

        assert_eq!(actual_one, expected_one);
        assert_eq!(actual_two, expected_two);
    }
}

#[cfg(test)]
mod test_corner_pair {
    use super::*;

    #[test]
    fn orientation_works() {
        let pair = CornerPair(Coord { x: 0, y: 0 }, Coord { x: 0, y: 7 });
        assert_eq!(pair.orientation(), Orientation::Vertical)
    }

    #[test]
    fn intersection_works_actual() {
        let first = CornerPair(Coord { x: 5, y: -7 }, Coord { x: -10, y: -7 });
        let second = CornerPair(Coord { x: -3, y: 3 }, Coord { x: -3, y: -10 });
        let expected = Coord { x: -3, y: -7 };
        let actual = first.intersection(second);

        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn intersection_works_none() {
        let first = CornerPair(Coord { x: 0, y: 0 }, Coord { x: 75, y: 0 });
        let second = CornerPair(Coord { x: 66, y: 62 }, Coord { x: 66, y: 117 });
        let actual = first.intersection(second);

        assert_eq!(actual, None);
    }
}
