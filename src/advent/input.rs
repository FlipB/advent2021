// mod input

use anyhow::Result;
use std::io::BufRead;

pub fn get_input_numbers(f: impl std::io::Read) -> Result<Vec<i64>> {
    let buf = std::io::BufReader::new(f);

    // convert lines in the file into a vector of integers.
    // We do it this way to avoid unnecessary allocations.
    // We collect into a vec rather than keeping as an iterator so that we
    // don't have to defer IO error handling to later iterations of the data.
    let numbers: Vec<i64> = buf
        .lines()
        .try_fold::<_, _, Result<_>>(vec![], |mut v, r| {
            let numstring = r?;
            let trimmed = numstring.trim();
            if !trimmed.is_empty() {
                v.push(trimmed.parse::<i64>()?);
            }
            Ok(v)
        })?;

    Ok(numbers)
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
    let iter = get_input_numbers(s.as_bytes()).unwrap();

    assert!(iter.into_iter().eq(v.into_iter()));
}
