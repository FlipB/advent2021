use std::fmt::{Display, Write};

use super::input;
use anyhow::Result;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let (map, line_len): (Vec<u8>, usize) = input::get_input_number_grid(input)?;

    let mut flasher = OctopusFlasher::new(Map::new(map.clone(), line_len));
    flasher.simulate(100);

    println!(
        "Part 1: Flash count after 100 ticks = {}",
        flasher.flash_count
    );

    let mut flasher = OctopusFlasher::new(Map::new(map, line_len));
    let tick_count = flasher.simulate_until_synchronize();
    println!("Part 2: Ticks until flashing synchronizes = {}", tick_count);

    Ok(())
}

struct OctopusFlasher {
    map: Map<u8>,
    flash_count: u64,
}

impl OctopusFlasher {
    fn new(map: Map<u8>) -> Self {
        Self {
            map,
            flash_count: 0,
        }
    }

    fn simulate(&mut self, num_ticks: u32) {
        for _n in 0..num_ticks {
            self.map.iter_positions().for_each(|oct| {
                self.tick_octopus(oct);
            });

            self.map.iter_positions().for_each(|oct| {
                self.reset_flashed(oct);
            });
        }
    }

    fn simulate_until_synchronize(&mut self) -> i32 {
        let octopus_count = (self.map.max.0 + 1) * (self.map.max.1 + 1);

        for n in 1.. {
            self.map.iter_positions().for_each(|oct| {
                self.tick_octopus(oct);
            });

            let reset_count = self.map.iter_positions().fold(0, |mut reset_count, oct| {
                if self.reset_flashed(oct) {
                    reset_count += 1;
                }
                reset_count
            });
            if reset_count == octopus_count {
                return n;
            }
        }
        panic!("end of func")
    }

    fn tick_octopus(&mut self, octopus_pos: Position) {
        let power = self.map.value(&octopus_pos);
        if power.is_none() {
            return;
        }
        let power = match *power.unwrap() {
            10.. => return, // already flashed this tick
            power @ 0..=9 => power + 1,
        };

        *self.map.value_mut(&octopus_pos).unwrap() = power;
        if power != 10 {
            return;
        }
        // octopus reached power level 10 - it will flash and increment it's neighbors
        self.flash_count += 1;

        octopus_pos.adjacents().for_each(|oct| {
            self.tick_octopus(oct);
        })
    }

    fn reset_flashed(&mut self, octopus_pos: Position) -> bool {
        if let Some(&power) = self.map.value(&octopus_pos) {
            if power == 10 {
                *self.map.value_mut(&octopus_pos).unwrap() = 0;
                return true;
            }
        }
        return false;
    }
}

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

/*
impl<T> Display for Map<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.max.1 {
            for x in 0..self.max.0 {
                let i = self.position_to_index(&(x, y).into()).unwrap();
                let val = &self.v[i];
                f.write_fmt(format_args!("{}", val))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
*/

impl Display for Map<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.max.1 {
            for x in 0..self.max.0 {
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

#[derive(Clone, Debug)]
struct Position(u32, u32);

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
    fn adjacents(&self) -> impl Iterator<Item = Position> {
        let (x, y) = (self.0, self.1);

        let mut i = 0u8;

        std::iter::from_fn(move || {
            loop {
                i += 1;
                if i > 9 {
                    break None;
                }
                // Position indexes:
                //	5 4 6
                //	2   1
                //	7 3 8
                //
                // So the first 4 positions yielded are the '+' neighbors.
                // The next 4 are the diagonal neighbors.
                let pos = match i {
                    1 => Some((x + 1, y).into()), // right
                    2 => {
                        // left
                        if x == 0 {
                            None
                        } else {
                            Some((x - 1, y).into())
                        }
                    }
                    3 => Some((x, y + 1).into()), // bottom
                    4 => {
                        // top
                        if y == 0 {
                            None
                        } else {
                            Some((x, y - 1).into())
                        }
                    }
                    5 => {
                        // top left
                        if x == 0 || y == 0 {
                            None
                        } else {
                            Some((x - 1, y - 1).into())
                        }
                    }
                    6 => {
                        // top right
                        if y == 0 {
                            None
                        } else {
                            Some((x + 1, y - 1).into())
                        }
                    }
                    7 => {
                        // bottom left
                        if x == 0 {
                            None
                        } else {
                            Some((x - 1, y + 1).into())
                        }
                    }
                    8 => Some((x + 1, y + 1).into()), // bottom right
                    _ => None,
                };
                if pos.is_some() {
                    break pos;
                }
            }
        })
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
