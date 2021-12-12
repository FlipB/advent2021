// mod input

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, Read};
use std::num::ParseIntError;
use std::path::Path;

pub fn read_file(file_path: impl AsRef<Path>) -> Result<impl std::io::Read> {
    Ok(File::open(file_path)?)
}

pub fn get_input_numbers<T>(f: impl std::io::Read) -> Result<(Vec<T>, usize)>
where
    T: core::str::FromStr<Err = ParseIntError>,
{
    let buf = std::io::BufReader::new(f);

    // convert lines in the file into a vector of integers.
    // We do it this way to avoid unnecessay allocations.
    // We collect into a vec rather than keeping as an iterator so that we
    // don't have to defer IO error handling to later iterations of the data.
    let numbers: (Vec<T>, usize) =
        buf.lines()
            .try_fold::<_, _, Result<_>>((vec![], 0usize), |(mut v, mut line_len), r| {
                let numstring = r?;
                let trimmed = numstring.trim();
                if !trimmed.is_empty() {
                    v.push(trimmed.parse::<T>()?);
                    line_len = trimmed.len();
                }
                Ok((v, line_len))
            })?;

    Ok(numbers)
}

pub fn get_input_number_grid<T>(f: impl std::io::Read) -> Result<(Vec<T>, usize)>
where
    T: core::convert::From<u8>,
{
    let buf = std::io::BufReader::new(f);

    // convert lines in the file into a vector of integers.
    // We do it this way to avoid unnecessay allocations.
    // We collect into a vec rather than keeping as an iterator so that we
    // don't have to defer IO error handling to later iterations of the data.
    let numbers: (Vec<T>, usize) = buf.lines().try_fold::<_, _, Result<_>>(
        (vec![], 0usize),
        |(mut v, mut line_len), line| {
            let line = line?;
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                line_len = trimmed.len();
                for c in trimmed.chars() {
                    match c as u8 {
                        c @ 48..=59 => v.push((c - 48).into()),
                        _ => return Err(anyhow::Error::msg("char is not a digit")),
                    }
                }
            }
            Ok((v, line_len))
        },
    )?;

    Ok(numbers)
}
pub fn get_input_lines(f: impl std::io::Read) -> Result<Vec<String>> {
    let buf = std::io::BufReader::new(f);

    let v = buf
        .lines()
        .collect::<std::result::Result<Vec<String>, std::io::Error>>()?;
    Ok(v)
}

pub fn lines_as_char_columns(lines: Vec<String>) -> impl Iterator<Item = Vec<char>> {
    let rows = lines
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>().into_iter())
        .collect();

    MultiZipper { iterators: rows }
}

/// MultiZipper zips multiple iterators
struct MultiZipper<I>
where
    I: Iterator,
{
    iterators: Vec<I>,
}

impl<I, T> Iterator for MultiZipper<I>
where
    I: Iterator<Item = T>,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterators
            .iter_mut()
            .map(|it| it.next())
            .collect::<Option<Vec<T>>>()
    }
}

#[test]
fn test_get_input_numbers() {
    let s = r"
        12
        14
        12
        0
        1
        ";
    let v = vec![12, 14, 12, 0, 1];
    let (iter, _): (Vec<i64>, usize) = get_input_numbers(s.as_bytes()).unwrap();

    assert!(iter.into_iter().eq(v.into_iter()));
}

#[test]
fn test_get_input_columns() {
    let s = r"12
12
12
12";
    let v = vec![vec!['1', '1', '1', '1'], vec!['2', '2', '2', '2']];
    let lines = get_input_lines(s.as_bytes()).unwrap();
    let iter = lines_as_char_columns(lines);

    assert!(iter.into_iter().eq(v.into_iter()));
}
