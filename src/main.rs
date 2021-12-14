use anyhow::Result;

mod advent;
use advent::{input::read_file, *};

fn main() -> Result<()> {
    day1::print_depth_increases(read_file("./data/day1.txt")?)?;
    day2::print_position(read_file("./data/day2.txt")?)?;
    day3::print_power_consumption_and_life_support_rating(read_file("./data/day3.txt")?)?;
    day4::print_result(read_file("./data/day4.txt")?)?;
    day5::print_overlapping_lines(read_file("./data/day5.txt")?)?;
    day6::print_fish_count_after_80_days(read_file("./data/day6.txt")?)?;
    day7::print_optimal_position_and_fuel_requirement(read_file("./data/day7.txt")?)?;
    day8::print_result(read_file("./data/day8.txt")?)?;
    day9::print_result(read_file("./data/day9.txt")?)?;
    day10::print_result(read_file("./data/day10.txt")?)?;
    day11::print_result(read_file("./data/day11.txt")?)?;
    day12::print_result(read_file("./data/day12.txt")?)?;
    day13::print_result(read_file("./data/day13.txt")?)?;
    day14::print_result(read_file("./data/day14.txt")?)?;

    Ok(())
}
