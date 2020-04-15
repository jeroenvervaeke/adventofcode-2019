use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_03/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let directions_1 = parse_direction_str(line.trim()).map_err(|_| "path 1 is incorrect")?;

    line.clear();
    reader.read_line(&mut line)?;
    let directions_2 = parse_direction_str(line.trim()).map_err(|_| "path 2 is incorrect")?;

    let distance = closest_intersection_distance(&directions_1, &directions_2, manhattan);
    println!("Manhattan distance: {}", distance);

    let distance = closest_intersection_distance(&directions_1, &directions_2, signal_distance);
    println!("Signal delay: {}", distance);

    Ok(())
}

fn manhattan(coordinate: &Coordinate, _: &Vec<Line>, _: &Vec<Line>) -> i32 {
    coordinate.x.abs() + coordinate.y.abs()
}

fn signal_distance(coordinate: &Coordinate, line_1: &Vec<Line>, line_2: &Vec<Line>) -> i32 {
    line_1.signal_distance(coordinate) + line_2.signal_distance(coordinate)
}

fn closest_intersection_distance<F>(
    directions_path_1: &Vec<Direction>,
    directions_path_2: &Vec<Direction>,
    distance_function: F,
) -> i32
where
    F: Fn(&Coordinate, &Vec<Line>, &Vec<Line>) -> i32,
{
    let path_coordinates_1 = to_coordinates(&directions_path_1);
    let path_coordinates_2 = to_coordinates(&directions_path_2);

    let path_1 = to_lines(&path_coordinates_1);
    let path_2 = to_lines(&path_coordinates_2);

    let mut intersections = Vec::new();
    for line_1 in &path_1 {
        for line_2 in &path_2 {
            if let Some(intersection) = line_1
                .intersection(&line_2)
                .filter(|c| c.x != 0 || c.y != 0)
            {
                intersections.push(intersection);
            }
        }
    }

    intersections.sort_by_cached_key(|coordinate| distance_function(coordinate, &path_1, &path_2));

    let closest_intersection = intersections
        .first()
        .expect("There is always at least one intersection");
    let manhattan_distance = distance_function(closest_intersection, &path_1, &path_2);

    manhattan_distance
}

fn to_coordinates(moves: &Vec<Direction>) -> Vec<Coordinate> {
    let mut coordinates: Vec<Coordinate> = moves
        .iter()
        .scan(Coordinate::new(0, 0), |coordinate, direction| {
            coordinate.move_in_direction(direction);
            Some(coordinate.clone())
        })
        .collect();

    coordinates.insert(0, Coordinate::new(0, 0));
    coordinates
}

fn to_lines(coordinates: &Vec<Coordinate>) -> Vec<Line> {
    coordinates
        .windows(2)
        .map(|c| Line::new(c[0].clone(), c[1].clone()))
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn move_in_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Down(step) => self.y -= *step,
            Direction::Left(step) => self.x -= *step,
            Direction::Right(step) => self.x += *step,
            Direction::Up(step) => self.y += *step,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    p1: Coordinate,
    p2: Coordinate,
}

#[derive(Debug, PartialEq)]
enum Alignment {
    Horizontal,
    Vertical,
}

impl Line {
    fn new(p1: Coordinate, p2: Coordinate) -> Self {
        Self { p1, p2 }
    }

    fn alignment(&self) -> Alignment {
        if self.p1.y == self.p2.y {
            Alignment::Horizontal
        } else {
            Alignment::Vertical
        }
    }

    // Simple intersect, since lines are always straight
    fn intersection(&self, other: &Line) -> Option<Coordinate> {
        let alignment_self = self.alignment();
        let alignment_other = other.alignment();

        if alignment_self == alignment_other {
            return None;
        }

        let (horizontal, vertical) = if alignment_self == Alignment::Horizontal {
            (self, other)
        } else {
            (other, self)
        };

        let horizontal_y = horizontal.p1.y;
        let (horizontal_x_min, horizontal_x_max) = if horizontal.p1.x < horizontal.p2.x {
            (horizontal.p1.x, horizontal.p2.x)
        } else {
            (horizontal.p2.x, horizontal.p1.x)
        };

        let vertical_x = vertical.p1.x;
        let (vertical_y_min, vertical_y_max) = if vertical.p1.y < vertical.p2.y {
            (vertical.p1.y, vertical.p2.y)
        } else {
            (vertical.p2.y, vertical.p1.y)
        };

        if horizontal_y <= vertical_y_max
            && horizontal_y >= vertical_y_min
            && vertical_x <= horizontal_x_max
            && vertical_x >= horizontal_x_min
        {
            Some(Coordinate::new(vertical_x, horizontal_y))
        } else {
            None
        }
    }

    fn contains(&self, coordinate: &Coordinate) -> bool {
        let (x_min, x_max) = if self.p1.x < self.p2.x {
            (self.p1.x, self.p2.x)
        } else {
            (self.p2.x, self.p1.x)
        };

        let (y_min, y_max) = if self.p1.y < self.p2.y {
            (self.p1.y, self.p2.y)
        } else {
            (self.p2.y, self.p1.y)
        };

        match self.alignment() {
            Alignment::Horizontal => {
                coordinate.x <= x_max && coordinate.x >= x_min && coordinate.y == y_min
            }
            Alignment::Vertical => {
                coordinate.y <= y_max && coordinate.y >= y_min && coordinate.x == x_min
            }
        }
    }

    fn len(&self) -> i32 {
        match self.alignment() {
            Alignment::Horizontal => {
                let (min, max) = min_max(self.p1.x, self.p2.x);
                max - min
            }
            Alignment::Vertical => {
                let (min, max) = min_max(self.p1.y, self.p2.y);
                max - min
            }
        }
    }
}

fn min_max(v1: i32, v2: i32) -> (i32, i32) {
    if v1 < v2 {
        (v1, v2)
    } else {
        (v2, v1)
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Down(i32),
    Left(i32),
    Right(i32),
    Up(i32),
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<_> = s.chars().collect();
        if chars.len() < 2 {
            return Err(());
        }

        let (direction, amount_chars) = chars.split_at(1);
        let amount = amount_chars
            .iter()
            .cloned()
            .collect::<String>()
            .parse()
            .map_err(|_| ())?;

        match direction {
            ['D'] => Ok(Direction::Down(amount)),
            ['L'] => Ok(Direction::Left(amount)),
            ['R'] => Ok(Direction::Right(amount)),
            ['U'] => Ok(Direction::Up(amount)),
            _ => Err(()),
        }
    }
}

fn parse_direction_str(s: &str) -> Result<Vec<Direction>, ()> {
    let mut result = Vec::new();
    for part in s.split(',') {
        if let Ok(direction) = part.parse() {
            result.push(direction);
        } else {
            return Err(());
        }
    }

    Ok(result)
}

trait SignalDistance {
    fn signal_distance(&self, coordinate: &Coordinate) -> i32;
}

impl SignalDistance for Vec<Line> {
    fn signal_distance(&self, coordinate: &Coordinate) -> i32 {
        let mut sum = 0;

        for line in self {
            if line.contains(coordinate) {
                let partial = Line::new(line.p1.clone(), coordinate.clone());
                sum += partial.len();
                break;
            } else {
                sum += line.len();
            }
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_direction_down() {
        assert_eq!("D1".parse(), Ok(Direction::Down(1)));
    }

    #[test]
    fn parse_direction_left() {
        assert_eq!("L123".parse(), Ok(Direction::Left(123)));
    }

    #[test]
    fn parse_direction_right() {
        assert_eq!("R3".parse(), Ok(Direction::Right(3)));
    }

    #[test]
    fn parse_direction_up() {
        assert_eq!("U999".parse(), Ok(Direction::Up(999)));
    }

    #[test]
    fn parse_direction_empty() {
        assert_eq!("".parse::<Direction>(), Err(()));
    }

    #[test]
    fn parse_direction_partial() {
        assert_eq!("D".parse::<Direction>(), Err(()));
    }

    #[test]
    fn path_to_coordinates_simple() {
        let directions = vec![
            Direction::Up(1),
            Direction::Left(1),
            Direction::Down(1),
            Direction::Right(1),
        ];
        let expected_result = vec![
            Coordinate::new(0, 0),
            Coordinate::new(0, 1),
            Coordinate::new(-1, 1),
            Coordinate::new(-1, 0),
            Coordinate::new(0, 0),
        ];

        let result = to_coordinates(&directions);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn coordinates_to_line() {
        let coordinates = vec![
            Coordinate::new(0, 1),
            Coordinate::new(-1, 1),
            Coordinate::new(-1, 0),
            Coordinate::new(0, 0),
        ];
        let expected_result = vec![
            Line::new(Coordinate::new(0, 1), Coordinate::new(-1, 1)),
            Line::new(Coordinate::new(-1, 1), Coordinate::new(-1, 0)),
            Line::new(Coordinate::new(-1, 0), Coordinate::new(0, 0)),
        ];

        let result = to_lines(&coordinates);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_direction_string() {
        let directions_str = "U1,L1,D1,R1";
        let expected_result = vec![
            Direction::Up(1),
            Direction::Left(1),
            Direction::Down(1),
            Direction::Right(1),
        ];

        let result = parse_direction_str(directions_str);

        assert_eq!(result, Ok(expected_result));
    }

    #[test]
    fn intersect_ok() {
        let line_1 = Line::new(Coordinate::new(0, 0), Coordinate::new(10, 0));
        let line_2 = Line::new(Coordinate::new(4, -10), Coordinate::new(4, 10));

        let expected_result = Coordinate::new(4, 0);

        let result = line_1.intersection(&line_2);

        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn example_main() {
        let directions_1 = parse_direction_str("R8,U5,L5,D3")
            .expect("directions_1 contains an invalid direction!");
        let directions_2 = parse_direction_str("U7,R6,D4,L4")
            .expect("directions_2 contains an invalid direction!");

        let calculated_distance =
            closest_intersection_distance(&directions_1, &directions_2, manhattan);

        assert_eq!(calculated_distance, 6);
    }

    #[test]
    fn example_1() {
        let directions_1 = parse_direction_str("R75,D30,R83,U83,L12,D49,R71,U7,L72")
            .expect("directions_1 contains an invalid direction!");
        let directions_2 = parse_direction_str("U62,R66,U55,R34,D71,R55,D58,R83")
            .expect("directions_2 contains an invalid direction!");

        let calculated_distance =
            closest_intersection_distance(&directions_1, &directions_2, manhattan);

        assert_eq!(calculated_distance, 159);
    }

    #[test]
    fn example_2() {
        let directions_1 = parse_direction_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
            .expect("directions_1 contains an invalid direction!");
        let directions_2 = parse_direction_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            .expect("directions_2 contains an invalid direction!");

        let calculated_distance =
            closest_intersection_distance(&directions_1, &directions_2, manhattan);

        assert_eq!(calculated_distance, 135);
    }

    #[test]
    fn part2_example_1() {
        let directions_1 = parse_direction_str("R75,D30,R83,U83,L12,D49,R71,U7,L72")
            .expect("directions_1 contains an invalid direction!");
        let directions_2 = parse_direction_str("U62,R66,U55,R34,D71,R55,D58,R83")
            .expect("directions_2 contains an invalid direction!");

        let calculated_distance =
            closest_intersection_distance(&directions_1, &directions_2, signal_distance);

        assert_eq!(calculated_distance, 610);
    }

    #[test]
    fn part2_example_2() {
        let directions_1 = parse_direction_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")
            .expect("directions_1 contains an invalid direction!");
        let directions_2 = parse_direction_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            .expect("directions_2 contains an invalid direction!");

        let calculated_distance =
            closest_intersection_distance(&directions_1, &directions_2, signal_distance);

        assert_eq!(calculated_distance, 410);
    }
}
