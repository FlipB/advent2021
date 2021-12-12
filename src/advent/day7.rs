use anyhow::Result;

use super::input;

pub fn print_optimal_position_and_fuel_requirement(input: impl std::io::Read) -> Result<()> {
    let input_lines = input::get_input_lines(input)?;

    let positions = input_lines
        .get(0)
        .ok_or(anyhow::Error::msg("bad input"))?
        .split(",")
        .map(|s| {
            s.parse()
                .map_err(|err| anyhow::Error::from(err).context(s.to_owned()))
        })
        .collect::<Result<Vec<i32>>>()?;

    let lowest_cost_naive = lowest_cost(positions.as_slice(), |c| c);
    let lowest_cost_triangular = lowest_cost(positions.as_slice(), cost_to_move);

    println!(
        "Part 1: Best position = {}, Total Fuel Cost = {}",
        lowest_cost_naive.0, lowest_cost_naive.1,
    );

    // part 2
    println!(
        "Part 2: Best position = {}, Total Fuel Cost = {}",
        lowest_cost_triangular.0, lowest_cost_triangular.1,
    );

    Ok(())
}

fn lowest_cost(positions: &[i32], cost_calc_func: impl Fn(i32) -> i32 + Copy) -> (i32, i32) {
    if positions.len() == 0 {
        panic!("positions slice is empty")
    }
    // the range relevant of positions
    let range = positions.iter().min().unwrap().clone()..positions.iter().max().unwrap().clone();

    let mut position_costs = range.map(|pos| (pos, sum_of_offsets(positions, pos, cost_calc_func)));
    let first = position_costs.next().unwrap().clone();

    position_costs.fold(
        first,
        |lowest, cost| {
            if cost.1 < lowest.1 {
                cost
            } else {
                lowest
            }
        },
    )
}

fn sum_of_offsets(positions: &[i32], target: i32, cost_calc: impl Fn(i32) -> i32) -> i32 {
    positions
        .iter()
        .map(|&p| cost_calc((target - p).abs()))
        .sum()
}

fn cost_to_move(steps: i32) -> i32 {
    // (1..=steps).sum()
    steps * (steps + 1) / 2
}
