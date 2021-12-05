use anyhow::Result;

mod advent;
use advent::{input::read_file, *};

fn main() -> Result<()> {
    day1::print_depth_increases(read_file("./data/day1.txt")?)?;
    day2::print_position(read_file("./data/day2.txt")?)?;
    day3::print_power_consumption_and_life_support_rating(read_file("./data/day3.txt")?)?;
    day4::print_result(read_file("./data/day4.txt")?)?;
    day5::print_overlapping_lines(read_file("./data/day5.txt")?)?;

    Ok(())
}
