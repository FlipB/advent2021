use anyhow::Result;

mod advent;
use advent::{input::read_file, *};

fn main() -> Result<()> {
    day1::print_depth_increases(read_file("./data/day1.txt")?)?;
    day2::print_position(read_file("./data/day2.txt")?)?;

    Ok(())
}
