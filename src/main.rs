use anyhow::Result;

mod advent;

fn main() -> Result<()> {
    let input_path = std::path::Path::new("./data/");

    advent::day1::print_depth_increases(input_path)?;

    Ok(())
}
