use std::collections::hash_map::HashMap;
use std::str::FromStr;

use anyhow::Result;

use super::input;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "x,y"
        let (x, y) = s.split_once(",").ok_or(anyhow::Error::msg("bad point"))?;
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Clone, Debug)]
struct Line(Point, Point);

impl std::str::FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "x,y -> x,y"
        let (s1, s2) = s.split_once(" -> ").ok_or(anyhow::Error::msg("bad line"))?;
        Ok(Self(Point::from_str(s1)?, Point::from_str(s2)?))
    }
}

impl Line {
    fn horizontal(&self) -> bool {
        self.0.x == self.1.x
    }

    fn vertical(&self) -> bool {
        self.0.y == self.1.y
    }

    fn diagonal(&self) -> bool {
        // a (45-deg.) diagonal line would have the same (absolute) difference
        // between the two points' x-coordinates as the y-coordinates.
        let x_diff = (self.0.x - self.1.x).abs();
        let y_diff = (self.0.y - self.1.y).abs();
        x_diff == y_diff
    }

    /// iter is only correct for vertical or horizontal lines
    fn iter(&self) -> impl Iterator<Item = Point> {
        let mut current_step = self.point_count();
        let line = self.clone();
        std::iter::from_fn(move || {
            if current_step == 0 {
                None
            } else {
                current_step -= 1;
                Some(line.point_at(current_step))
            }
        })
    }

    fn point_count(&self) -> u32 {
        let x_diff = (self.0.x - self.1.x).abs() as u32;
        let y_diff = (self.0.y - self.1.y).abs() as u32;
        if x_diff >= y_diff {
            x_diff + 1
        } else {
            y_diff + 1
        }
    }

    fn point_at(&self, i: u32) -> Point {
        let v_x = if self.0.x < self.1.x {
            1
        } else if self.0.x > self.1.x {
            -1
        } else {
            0
        };
        let v_y: i32 = if self.0.y < self.1.y {
            1
        } else if self.0.y > self.1.y {
            -1
        } else {
            0
        };
        let i = i as i32;
        Point {
            x: self.0.x + (v_x * i),
            y: self.0.y + (v_y * i),
        }
    }
}

pub fn print_overlapping_lines(input: impl std::io::Read) -> Result<()> {
    let input_lines = input::get_input_lines(input)?;
    // We assume all lines parse ok.
    let lines = input_lines
        .iter()
        .map(|line_string| Line::from_str(line_string.as_str()))
        .flatten()
        .filter(|l| l.horizontal() || l.vertical() || l.diagonal());

    let all_line_points = lines.map(|l| l.iter()).flatten();
    let mut point_overlap: HashMap<Point, u32> = HashMap::new();
    for point in all_line_points {
        // tabulate in hashmap
        let count = point_overlap.entry(point).or_insert(0);
        *count += 1;
    }
    let n_points_with_overlap = point_overlap.iter().filter(|(_, &n)| n >= 2).count();

    println!(
        "Number of lines overlapping a single point is {}",
        n_points_with_overlap
    );

    Ok(())
}
