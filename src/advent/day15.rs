use std::fmt::{Display, Write};

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::vec;

use super::input;
use anyhow::Result;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let (values, line_len): (Vec<u8>, usize) = input::get_input_number_grid(input)?;
    let org_map = Map::new(values, line_len);

    let djikstra = PathFinder::new(org_map.clone());
    let (r, _path) = djikstra.dijkstras(Position(0, 0), org_map.max).unwrap();

    println!("Got Risk = {}", r);

    // part 2

    let full = add_tiles(org_map.clone());

    let djikstra = PathFinder::new(full.clone());
    let (r, _path) = djikstra.dijkstras(Position(0, 0), full.max).unwrap();

    println!("Got Risk = {} ", r);

    Ok(())
}

fn add_tiles(mut old: Map<u8>) -> Map<u8> {
    let mut org = old.clone();
    for _x in 1..5 {
        org.iter_mut().for_each(|x| {
            if *x == 9 {
                *x = 0
            }
            *x += 1
        });
        old.expand_right(&org);
    }
    let mut org = old.clone();
    for _y in 1..5 {
        org.iter_mut().for_each(|x| {
            if *x == 9 {
                *x = 0
            }
            *x += 1
        });
        old.expand_down(&org);
    }
    old
}

type Risk = u32;

struct PathFinder {
    map: Map<u8>,
}

impl PathFinder {
    fn new(map: Map<u8>) -> Self {
        Self { map }
    }

    fn adjacents<'a>(&'a self, pos: Position) -> impl Iterator<Item = (Risk, Position)> + 'a {
        pos.adjacents(self.map.max)
            .into_iter()
            .take(4)
            .filter_map(|pos| {
                if let Adjacent::Position(pos) = pos {
                    // If position has a value (it really should since we're not out-of-bounds),
                    // map it to the desired structure.
                    self.map.value(&pos).map(|&risk| (risk as Risk, pos))
                } else {
                    None // filter out out-of-bounds
                }
            })
    }

    fn dijkstras(&self, start: Position, goal: Position) -> Option<(Risk, Vec<Position>)> {
        // dijkstras algorithm using a priority queue.

        // `dist` is the distance map, tracking lowest known total risk to move from start to a position.
        let mut dist: Map<Risk> = Map {
            max: self.map.max,
            v: vec![Risk::MAX; self.map.v.len()],
        };
        *dist.value_mut(&start).unwrap() = 0; // Distance to start position is 0.

        // Create a min-heap (reverse risk) to yield lowest risk positions first.
        let mut queue: BinaryHeap<(Reverse<Risk>, Position)> = BinaryHeap::new();
        // Start searching paths from `start` with an inital total risk for the path of 0.
        queue.push((Reverse(0), start));

        // Try lowest total risk positions first (essentially breadth first search)
        while let Some((Reverse(risk), position)) = queue.pop() {
            if position == goal {
                return Some((risk, self.get_path(dist)));
            }

            // Stop search and reject path if we got to this position in a roundabout way
            if risk > *dist.value(&position).unwrap() {
                continue;
            }

            // Check if it's viable to continue along this path
            for neighbor in self.adjacents(position) {
                let (next_cost, next_pos) = (risk + neighbor.0, neighbor.1);

                // If there's no better, known path to get to the position, continue.
                if next_cost < *dist.value(&next_pos).unwrap() {
                    // Mark the new, lowest known risk to get to the new position.
                    *dist.value_mut(&next_pos).unwrap() = next_cost;
                    // Continue traversing the path through the new position.
                    queue.push((Reverse(next_cost), next_pos));
                }
            }
        }
        None
    }

    fn get_path(&self, map: Map<Risk>) -> Vec<Position> {
        let mut path = vec![];
        let mut pos = map.max;
        while pos != Position(0, 0) {
            let mut neighbors = self.adjacents(pos);
            let mut best = neighbors
                .next()
                .map(|(_, pos)| (*map.value(&pos).unwrap(), pos))
                .unwrap();
            for (_, next) in neighbors {
                let risk = *map.value(&next).unwrap();
                if risk < best.0 {
                    best = (risk, next);
                }
            }
            pos = best.1;
            path.push(pos);
        }
        path.push(map.max);
        path
    }
}

#[derive(Clone)]
struct Map<T> {
    /// max denotes the maximum values in every direction
    max: Position,
    v: Vec<T>,
}

impl<T> Map<T> {
    fn new(values: Vec<T>, line_len: usize) -> Self {
        let rows = values.len() / line_len;
        Self {
            max: Position((line_len - 1) as u32, (rows - 1) as u32),
            v: values,
        }
    }

    fn expand_right(&mut self, other: &Map<T>)
    where
        T: Clone,
    {
        if self.max.1 != other.max.1 {
            panic!("other tile must be same size in the y axis")
        }
        let mut replacement = vec![];
        let mut line_length = 0;
        for row in self.rows().zip(other.rows()) {
            replacement.extend_from_slice(row.0);
            replacement.extend_from_slice(row.1);
            line_length = row.0.len() + row.1.len();
        }
        self.v = replacement;
        self.max = Position((line_length - 1) as u32, self.max.1);
    }

    fn expand_down(&mut self, other: &Map<T>)
    where
        T: Clone,
    {
        if self.max.0 != other.max.0 {
            panic!("other tile must be same size in the x axis")
        }
        let mut replacement = vec![];
        let mut num_rows = 0;
        for row in self.rows() {
            replacement.extend_from_slice(row);
            num_rows += 1;
        }
        for row in other.rows() {
            replacement.extend_from_slice(row);
            num_rows += 1;
        }
        self.v = replacement;
        self.max = Position(self.max.0, (num_rows - 1) as u32);
    }

    fn rows<'a>(&'a self) -> impl Iterator<Item = &'a [T]> + 'a {
        (0..=self.max.1).map(|y| {
            let s = self.v.as_slice();
            let start_pos = (y * (self.max.0 + 1)) as usize;
            let end_pos = start_pos + self.max.0 as usize;
            let row = &s[start_pos..=end_pos];
            row
        })
    }

    fn value(&self, pos: &Position) -> Option<&T> {
        self.v.get(self.position_to_index(pos)?)
    }

    fn value_mut(&mut self, pos: &Position) -> Option<&mut T> {
        let i = self.position_to_index(pos)?;
        self.v.get_mut(i)
    }

    fn foreach_position_value_mut(
        &mut self,
        positions: impl Iterator<Item = Position>,
        f: impl Fn(&mut T),
    ) {
        positions.for_each(|p| {
            if let Some(v) = self.value_mut(&p) {
                f(v)
            }
        })
    }

    fn iter_positions(&self) -> impl Iterator<Item = Position> {
        let mut i = 0usize;
        let max = self.max.clone();
        std::iter::from_fn(move || {
            let pos = Self::index_to_position(&max, i);
            i += 1;
            pos
        })
    }

    fn index_to_position(max: &Position, i: usize) -> Option<Position> {
        let line_length = (max.x() + 1) as usize;
        let x = i % line_length;
        let y = i / line_length;
        let pos: Position = (x, y).into();
        if pos.le(max) {
            Some(pos)
        } else {
            None
        }
    }

    fn position_to_index(&self, pos: &Position) -> Option<usize> {
        if pos.le(&self.max) {
            let line_length = self.max.x() + 1;
            let x = pos.y();
            let y = pos.x();

            let index = (x * line_length) + y;

            Some((index) as usize)
        } else {
            None
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.v.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.v.iter_mut()
    }
}

impl Display for Map<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max.1 {
            for x in 0..=self.max.0 {
                let i = self.position_to_index(&(x, y).into()).unwrap();
                let val = self.v[i];
                let c = if val == 10 { '*' } else { (val + 48u8) as char };
                f.write_char(c)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position(u32, u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Adjacent {
    Position(Position),
    OutOfBounds,
}

impl Position {
    #[inline]
    fn x(&self) -> u32 {
        self.0
    }

    #[inline]
    fn y(&self) -> u32 {
        self.1
    }

    /// le returns true if self is less than or equal to the other position,
    /// in every direction.
    fn le(&self, other_pos: &Position) -> bool {
        self.1 <= other_pos.1 && self.0 <= other_pos.0
    }

    /// adjacents may include positions that are out of bounds
    fn adjacents(&self, max: Position) -> [Adjacent; 8] {
        let mut adj = [Adjacent::OutOfBounds; 8];
        let (x, y) = (self.0, self.1);

        for i in 0..8 {
            // Position indexes:
            //	4 3 5
            //	2   0
            //	6 1 7
            //
            // So the first 4 positions yielded are the '+' neighbors.
            // The next 4 are the diagonal neighbors.
            let mut pos = match i {
                0 => Adjacent::Position((x + 1, y).into()), // right
                1 => Adjacent::Position((x, y + 1).into()), // bottom
                2 => {
                    // left
                    if x == 0 {
                        Adjacent::OutOfBounds
                    } else {
                        Adjacent::Position((x - 1, y).into())
                    }
                }
                3 => {
                    // top
                    if y == 0 {
                        Adjacent::OutOfBounds
                    } else {
                        Adjacent::Position((x, y - 1).into())
                    }
                }
                4 => {
                    // top left
                    if x == 0 || y == 0 {
                        Adjacent::OutOfBounds
                    } else {
                        Adjacent::Position((x - 1, y - 1).into())
                    }
                }
                5 => {
                    // top right
                    if y == 0 {
                        Adjacent::OutOfBounds
                    } else {
                        Adjacent::Position((x + 1, y - 1).into())
                    }
                }
                6 => {
                    // bottom left
                    if x == 0 {
                        Adjacent::OutOfBounds
                    } else {
                        Adjacent::Position((x - 1, y + 1).into())
                    }
                }
                7 => Adjacent::Position((x + 1, y + 1).into()), // bottom right
                _ => unreachable!(),
            };
            if let Adjacent::Position(apos) = pos {
                if !apos.le(&max) {
                    pos = Adjacent::OutOfBounds
                }
            }
            adj[i] = pos;
        }
        adj
    }
}

impl From<(usize, usize)> for Position {
    fn from(tup: (usize, usize)) -> Self {
        Position(tup.0 as u32, tup.1 as u32)
    }
}

impl From<(u32, u32)> for Position {
    fn from(tup: (u32, u32)) -> Self {
        Position(tup.0, tup.1)
    }
}

#[test]
fn test_map() {
    let m = Map::new(vec![0, 0, 0, 0], 2);
    // 00
    // 00

    // 0,1,2,3
    // func(0,0) -> 0
    // func(1,0) -> 1
    // func(0,1) -> 2
    // func(1,1) -> 3

    assert_eq!(
        m.iter_positions()
            .map(|p| {
                println!("{:?}", p);
                p
            })
            .count(),
        4
    );
    assert_eq!(m.position_to_index(&Position(0, 0)), Some(0));
    assert_eq!(m.position_to_index(&Position(1, 0)), Some(1));
    assert_eq!(m.position_to_index(&Position(0, 1)), Some(2));
    assert_eq!(m.position_to_index(&Position(1, 1)), Some(3));
    assert_eq!(m.position_to_index(&Position(2, 0)), None);
    assert_eq!(m.position_to_index(&Position(2, 1)), None);
    assert_eq!(m.position_to_index(&Position(1, 2)), None);

    let m = Map::new(vec![0, 0, 0, 0, 0, 0], 3);

    assert_eq!(
        m.iter_positions()
            .map(|p| {
                println!("{:?}", p);
                p
            })
            .count(),
        6
    );
}

#[test]
fn test_map2() {
    let m = Map::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0], 3);
    // 00
    // 00

    // 0,1,2,3
    // func(0,0) -> 0
    // func(1,0) -> 1
    // func(0,1) -> 2
    // func(1,1) -> 3

    assert_eq!(
        m.iter_positions()
            .map(|p| {
                println!("{:?}", p);
                p
            })
            .count(),
        9
    );
    assert_eq!(m.position_to_index(&Position(0, 0)), Some(0));
    assert_eq!(m.position_to_index(&Position(1, 0)), Some(1));
    assert_eq!(m.position_to_index(&Position(2, 0)), Some(2));
    assert_eq!(m.position_to_index(&Position(3, 0)), None);
    assert_eq!(m.position_to_index(&Position(0, 1)), Some(3));
    assert_eq!(m.position_to_index(&Position(1, 1)), Some(4));
    assert_eq!(m.position_to_index(&Position(2, 1)), Some(5));
    assert_eq!(m.position_to_index(&Position(3, 1)), None);
    assert_eq!(m.position_to_index(&Position(0, 2)), Some(6));
    assert_eq!(m.position_to_index(&Position(1, 2)), Some(7));
    assert_eq!(m.position_to_index(&Position(2, 2)), Some(8));
    assert_eq!(m.position_to_index(&Position(3, 2)), None);
    assert_eq!(m.position_to_index(&Position(3, 3)), None);

    let m = Map::new(vec![0, 0, 0, 0, 0, 0], 3);

    assert_eq!(
        m.iter_positions()
            .map(|p| {
                println!("{:?}", p);
                p
            })
            .count(),
        6
    );
}

#[test]
fn test_map_rows() {
    let m = Map::new(vec![0, 0, 0, 1, 1, 1, 2, 2, 2], 3);

    for r in m.rows() {
        for p in r {
            print!("{}", p);
        }
        println!("");
    }

    let mut rows = m.rows();
    assert_eq!(
        [
            rows.next().unwrap(),
            rows.next().unwrap(),
            rows.next().unwrap()
        ],
        [[0, 0, 0], [1, 1, 1], [2, 2, 2]]
    );
    assert_eq!(rows.next(), None);
}
