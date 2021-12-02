use anyhow::Result;

use super::input;

pub fn print_depth_increases(input: impl std::io::Read) -> Result<()> {
    let values = input::get_input_numbers(input)?;

    println!(
        "Depth increases: {}",
        get_depth_increases(values.clone().into_iter())
    );

    println!(
        "Windowed depth increases: {}",
        get_depth_increases(summed_windows(values.as_slice()))
    );
    Ok(())
}

/// get_depth_increases returns number of depth increases in the input file for day1 found in input path.
pub fn get_depth_increases(v: impl Iterator<Item = i64>) -> i64 {
    let mut current_depth: Option<i64> = None;

    v.fold(0i64, |mut t, c| -> i64 {
        if let Some(depth) = current_depth {
            if depth < c {
                t = t + 1;
            }
        }
        current_depth = Some(c);
        t
    })
}

pub fn summed_windows<'a>(v: &'a [i64]) -> impl Iterator<Item = i64> + 'a {
    v.windows(3).map(|x| x.iter().sum())
}

#[test]
fn test_summed_windows() {
    let v = vec![3, 3, 1, 0, 4, 0, 1, 1];
    let iter = summed_windows(v.as_slice());

    assert!(&[7, 4, 5, 4, 5, 2].into_iter().eq(iter));
}

#[test]
fn test_get_depth_increases() {
    let v = vec![3, 3, 1, 0, 4, 0, 1, 1];
    let n = get_depth_increases(v.into_iter());

    assert_eq!(n, 2);
}
