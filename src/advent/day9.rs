use anyhow::Result;

use std::io::{BufRead, Read};

pub fn print_result(input: impl Read) -> Result<()> {
    let height_map = HeightMap::parse(input)?;

    let tot: u32 = height_map
        .low_points()
        .map(|p| (height_map.position_depth(p).unwrap() + 1) as u32)
        .sum();

    // part 1
    println!("Part 1: {}", tot);

    let mut basins = height_map
        .basins()
        .iter()
        .map(|b| b.len())
        .collect::<Vec<usize>>();
    basins.sort();

    let basin_size_product = basins.iter().rev().take(3).fold(1, |mut prod, &size| {
        prod *= size;
        prod
    });

    println!("Part 2: Basin size product = {}", basin_size_product);

    Ok(())
}

struct HeightMap(Vec<Vec<i8>>);

impl HeightMap {
    pub fn parse(reader: impl std::io::Read) -> Result<HeightMap> {
        let buf = std::io::BufReader::new(reader);

        let decimal_heighmap: Result<Vec<Vec<i8>>> = buf
            .lines()
            .map(|line| {
                if let Ok(s) = line {
                    Ok(s.chars()
                        .filter_map(|c| match c {
                            c @ '0'..='9' => Some(c as i8 - 48),
                            _ => None,
                        })
                        .collect::<Vec<i8>>())
                } else {
                    Err(anyhow::Error::from(line.unwrap_err()))
                }
            })
            .collect();

        Ok(HeightMap(decimal_heighmap?))
    }

    pub fn position_depth(&self, (x, y): (isize, isize)) -> Option<i8> {
        let row = self.0.get(y as usize)?;
        Some(row.get(x as usize)?.clone())
    }

    fn dimensions(&self) -> (isize, isize) {
        let x = self.0.get(0).map(|line| line.len()).unwrap_or(0);
        let y = self.0.len();
        (x as isize, y as isize)
    }

    pub fn low_points<'a>(&'a self) -> impl Iterator<Item = (isize, isize)> + 'a {
        self.iter_positions().filter(|&p| self.is_low_point(p))
    }

    pub fn basins(&self) -> Vec<Vec<(isize, isize)>> {
        self.iter_positions().fold(vec![], |mut basins, pos| {
            self.try_to_add_to_basin(&mut basins, pos);
            basins
        })
    }

    fn is_low_point(&self, pos: (isize, isize)) -> bool {
        let pos_height = self.position_depth(pos).unwrap();
        let adjacents = self
            .adjacent_positions(pos)
            .into_iter()
            .filter_map(|pos| self.position_depth(pos));

        for adjacent in adjacents {
            if adjacent <= pos_height {
                return false;
            }
        }
        true
    }

    fn iter_positions<'a>(&'a self) -> impl Iterator<Item = (isize, isize)> + 'a {
        MapPosition::new(self.dimensions())
    }

    fn adjacent_positions(&self, (x, y): (isize, isize)) -> Vec<(isize, isize)> {
        let mut v = vec![];
        if y != 0 {
            v.push((x, y - 1))
        }
        if x != 0 {
            v.push((x - 1, y))
        }
        let (max_x, max_y) = self.dimensions();
        if y <= max_y {
            v.push((x + 1, y))
        }
        if x <= max_x {
            v.push((x, y + 1))
        }

        v
    }

    fn try_to_add_to_basin(&self, basins: &mut Vec<Vec<(isize, isize)>>, pos: (isize, isize)) {
        if let Some(depth) = self.position_depth(pos) {
            if depth == 9 {
                return; // not a basin
            }
        }

        // check if position is in a known basin
        for basin in basins.iter() {
            if basin.contains(&pos) {
                return;
            }
        }

        // New basin discovered - expore it completely
        let mut new_basin = vec![];
        self.find_all_points_in_basin(&mut new_basin, pos);
        basins.push(new_basin)
    }

    fn find_all_points_in_basin(&self, basin: &mut Vec<(isize, isize)>, pos: (isize, isize)) {
        if let Some(depth) = self.position_depth(pos) {
            if depth >= 9 {
                return; // not a basin
            }
        } else {
            return;
        }

        if basin.contains(&pos) {
            return;
        }
        // pos is part of a basin.
        basin.push(pos);

        // add adjacent positions that can be considered part of the same basin
        self.adjacent_positions(pos)
            .iter()
            .for_each(|&p| self.find_all_points_in_basin(basin, p));
    }
}

struct MapPosition {
    dimensions: (isize, isize),
    current: Option<(isize, isize)>,
}

impl MapPosition {
    fn new(dimensions: (isize, isize)) -> Self {
        Self {
            dimensions,
            current: None,
        }
    }

    fn next_position(&mut self) -> Option<(isize, isize)> {
        let pos = match self.current {
            None => Some((0, 0)),
            Some((mut x, mut y)) => {
                x += 1;
                if x >= self.dimensions.0 {
                    x = 0;
                    y += 1;
                }
                if y >= self.dimensions.1 {
                    y = 0;
                }
                if x == 0 && y == 0 {
                    None
                } else {
                    Some((x, y))
                }
            }
        };
        if pos != None {
            self.current = pos;
        }
        pos
    }
}

impl Iterator for MapPosition {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_position()
    }
}
