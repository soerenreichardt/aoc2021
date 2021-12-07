use std::collections::{HashMap};
use std::ops::{Sub};

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

impl From<&str> for Point {
    fn from(point_str: &str) -> Self {
        match point_str.split(",").collect::<Vec<&str>>().as_slice() {
            [x, y] => Point { x: x.parse().unwrap(), y: y.parse().unwrap() },
            _ => panic!("Expected to read 2 coordinates")
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
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

struct LineIntoIterator {
    distance: Point,
    direction: Point,
    last_point: Point
}

fn diagonal_lines() -> bool {
    false
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        let direction_vector = self.end - self.start;

        if !diagonal_lines() && direction_vector.x != 0 && direction_vector.y != 0 {
            return LineIntoIterator {
                distance: Point::new(0, 0),
                direction: Point::new(0, 0),
                last_point: Point::new(0, 0)
            }
        }

        let distance_x = direction_vector.x.abs() + 1;
        let distance_y = direction_vector.y.abs() + 1;

        let distance = Point::new(distance_x, distance_y);

        let step_x = if self.start.x == self.end.x { 0 } else { 1 };
        let step_y = if self.start.y == self.end.y { 0 } else { 1 };

        let direction_x = direction_vector.x.signum();
        let direction_y = direction_vector.y.signum();

        let direction = Point::new(step_x * direction_x, step_y * direction_y);

        LineIntoIterator {
            distance,
            direction,
            last_point: self.start
        }
    }
}

impl Iterator for LineIntoIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.distance == Point::new(0, 0) {
            return None
        }

        let interpolated_point = Some(self.last_point);

        let mut x = self.last_point.x;
        let mut y = self.last_point.y;
        if self.distance.x > 0 {
            x += self.direction.x;
            self.distance.x -= 1;
        }
        if self.distance.y > 0 {
            y += self.direction.y;
            self.distance.y -= 1;
        }

        self.last_point = Point::new(x, y);
        interpolated_point
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

        let line = Line { start: Point::new(4, 3), end: Point::new(1, 3) };
        let points: Vec<Point> = line.into_iter().collect();

        assert_eq!(
            points,
            vec![
                Point::new(4, 3),
                Point::new(3, 3),
                Point::new(2, 3),
                Point::new(1, 3),
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

        if diagonal_lines() {
            assert_eq!(find_hydrothermal_vents(input), 12);
        } else {
            assert_eq!(find_hydrothermal_vents(input), 5);
        }
    }
}
