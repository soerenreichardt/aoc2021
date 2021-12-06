use std::array::IntoIter;
use std::collections::{Bound, HashMap};
use std::ops::Bound::Included;
use std::ops::RangeBounds;
use crate::day5::IterationDirection::NONE;

#[derive(Eq, PartialEq, Debug, Hash)]
struct Point {
    x: u32,
    y: u32
}

impl From<&str> for Point {
    fn from(point_str: &str) -> Self {
        match point_str.split(",").collect::<Vec<&str>>().as_slice() {
            [x, y] => Point { x: x.parse().unwrap(), y: y.parse().unwrap() },
            _ => panic!("Expected to read 2 coordinates")
        }
    }
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

struct Line {
    start: Point,
    end: Point
}

impl From<(Point, Point)> for Line {
    fn from(points: (Point, Point)) -> Self {
        Line { start: points.0, end: points.1 }
    }
}

enum IterationDirection {
    X, Y, NONE
}

struct LineIntoIterator {
    line: Line,
    iteration_direction: IterationDirection,
    iteration_value: u32,
    upper_bound: u32
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        if self.start.x != self.end.x && self.start.y != self.end.y {
            return LineIntoIterator {
                line: self,
                iteration_direction: IterationDirection::NONE,
                iteration_value: 0,
                upper_bound: 0
            }
        }

        let iteration_direction = match (self.start.x == self.end.x, self.start.y == self.end.y) {
            (true, false) => IterationDirection::Y,
            (false, true) => IterationDirection::X,
            (true, true) => IterationDirection::NONE,
            (false, false) => panic!("This should not happen")
        };

        let iteration_value = match iteration_direction {
            IterationDirection::X => self.start.x,
            IterationDirection::Y => self.start.y,
            IterationDirection::NONE => self.start.x
        };

        let upper_bound = match iteration_direction {
            IterationDirection::X => self.end.x,
            IterationDirection::Y => self.end.y,
            IterationDirection::NONE => self.start.x
        };

        LineIntoIterator {
            line: self,
            iteration_direction,
            iteration_value,
            upper_bound
        }
    }
}

impl Iterator for LineIntoIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration_value <= self.upper_bound {
            let next_point = match self.iteration_direction {
                IterationDirection::X => Point { x: self.iteration_value, y: self.line.start.y },
                IterationDirection::Y => Point { x: self.line.end.x, y: self.iteration_value },
                IterationDirection::NONE => Point { x: self.line.start.x, y: self.line.start.y }
            };
            self.iteration_value += 1;
            Some(next_point)
        } else {
            None
        }
    }
}

pub fn find_hydrothermal_vents(data: String) -> u32 {
    let data_rows = data.lines().collect();
    let point_tuples = point_tuples_from_str_rows(data_rows);
    let lines: Vec<Line> = point_tuples.into_iter().map(|point_tuple| Line::from(point_tuple)).collect();

    lines
        .into_iter()
        .flat_map(|line| line.into_iter())
        .fold(HashMap::new(), |mut vent_field: HashMap<Point, u32>, point| {
            *vent_field.entry(point).or_insert(0) += 1;
            vent_field
        })
        .values()
        .into_iter()
        .filter(|count| **count > 1)
        .count() as u32
}

fn point_tuples_from_str_rows(rows: Vec<&str>) -> Vec<(Point, Point)> {
    rows.iter().map(|row| {
        match row.split(" -> ").collect::<Vec<&str>>().as_slice() {
            [p1, p2] => (Point::from(*p1), Point::from(*p2)),
            _ => panic!("Expected to read 2 coordinates per line")
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn should_interpolate_lines() {
        let line = Line { start: Point::new(1, 3), end: Point::new(4, 3) };
        let points: Vec<Point> = line.into_iter().collect();

        assert_eq!(
            points,
            vec![
                Point::new(1, 3),
                Point::new(2, 3),
                Point::new(3, 3),
                Point::new(4, 3),
            ]
        );
    }

    #[test]
    fn should_interpolate_point() {
        let line = Line { start: Point::new(1, 3), end: Point::new(1, 3) };
        let points: Vec<Point> = line.into_iter().collect();

        assert_eq!(points, vec![Point::new(1, 3)])
    }

    #[test]
    fn should_pass_test() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2".to_string();

        assert_eq!(find_hydrothermal_vents(input), 5);
    }
}
