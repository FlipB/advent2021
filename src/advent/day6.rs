use anyhow::Result;

use super::input;

pub fn print_fish_count_after_80_days(input: impl std::io::Read) -> Result<()> {
    let input_lines = input::get_input_lines(input)?;

    let fish = input_lines
        .get(0)
        .ok_or(anyhow::Error::msg("bad input"))?
        .split(",")
        .map(|s| {
            s.parse()
                .map_err(|err| anyhow::Error::from(err).context(s.to_owned()))
        })
        .collect::<Result<Vec<u32>>>()?;

    let mut fish_partitions = partition_by_age(fish.clone());

    let mut fish_iter = fish_over_time_iter(fish_partitions.as_mut_slice());
    println!(
        "Number of fish ofter 80 days = {}",
        fish_iter.nth(80).expect("working iterator")
    );
    let mut fish_partitions = partition_by_age(fish);

    let mut fish_iter = fish_over_time_iter(fish_partitions.as_mut_slice());
    println!(
        "Number of fish ofter 256 days = {}",
        fish_iter.nth(256).expect("working iterator")
    );

    Ok(())
}

fn partition_by_age(fish: Vec<u32>) -> Vec<u64> {
    let mut v: Vec<u64> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0..8 {
        v.insert(i, fish.iter().filter(|&&n| n == i as u32).count() as u64)
    }
    v
}

fn moar_fish(fish_partitions: &mut [u64]) {
    let spawning = fish_partitions[0];
    for time_to_spawn in 0..8 {
        fish_partitions[time_to_spawn] = fish_partitions[time_to_spawn + 1];
    }
    fish_partitions[6] = fish_partitions[6] + spawning;
    fish_partitions[8] = spawning;
}

fn fish_over_time_iter<'a>(mut fish_partitions: &'a mut [u64]) -> impl Iterator<Item = u64> + 'a {
    std::iter::from_fn(move || {
        let sum = fish_partitions.iter().sum();
        moar_fish(&mut fish_partitions);
        Some(sum)
    })
}
