use std::vec;

use super::input;
use anyhow::Result;

pub fn print_result(f: impl std::io::Read) -> Result<()> {
    let lines = input::get_input_lines(f)?;
    let numbers: Vec<Vec<Number>> = lines.iter().map(parse_snailfish_numbers).collect();

    println!(
        "Snail number magnitude = {}",
        calc_magnitude(&add_lines(&numbers))
    );

    println!("Largest magnitude = {}", find_biggest_magnitude(&numbers));

    Ok(())
}

fn parse_snailfish_numbers(input: &String) -> Vec<Number> {
    let mut values = vec![];
    let mut depth = 0;
    for c in input.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => {}
            c => {
                values.push(Number {
                    value: c.to_digit(10).unwrap() as u32,
                    depth: depth as usize,
                });
            }
        }
    }

    return values;
}

fn find_biggest_magnitude(lines: &Vec<Vec<Number>>) -> u32 {
    let mut max = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            let number_lines = [lines[i].clone(), lines[j].clone()];
            let magnitude = calc_magnitude(&add_lines(&number_lines));
            if magnitude > max {
                max = magnitude
            }
        }
    }
    return max;
}

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    depth: usize,
}

impl Number {
    fn new(v: u32, d: usize) -> Self {
        Number { value: v, depth: d }
    }
}

/// perform snailfish addition on lines of numbers
fn add_lines(lines: &[Vec<Number>]) -> Vec<Number> {
    let mut num = lines[0].clone();

    for next in &lines[1..lines.len()] {
        let mut sum = add(&num, &next);
        while reduce(&mut sum) {}
        num = sum;
    }

    num
}

fn calc_magnitude(input: &Vec<Number>) -> u32 {
    let mut curr = input.clone();

    loop {
        if curr.len() == 1 {
            break curr[0].value;
        }
        let depth = curr.iter().map(|v| v.depth).max().unwrap();
        inner_magnitude(&mut curr, depth)
    }
}

fn inner_magnitude(input: &mut Vec<Number>, depth: usize) {
    for i in 0..input.len() {
        if input[i].depth != depth {
            continue;
        }

        input[i] = Number {
            value: (3 * input[i].value) + (2 * input[i + 1].value),
            depth: depth - 1,
        };
        input.remove(i + 1);
        break;
    }
}

fn reduce(input: &mut Vec<Number>) -> bool {
    for i in 0..input.len() {
        if explode(input, i) {
            return true;
        }
    }
    for i in 0..input.len() {
        if split(input, i) {
            return true;
        }
    }
    false
}

fn explode(input: &mut Vec<Number>, at_index: usize) -> bool {
    let Number { value, depth } = input[at_index];
    if depth <= 4 {
        return false;
    }
    if at_index > 0 {
        input[at_index - 1].value += value;
    }
    if at_index + 2 < input.len() {
        input[at_index + 2].value += input[at_index + 1].value;
    }
    input.remove(at_index);
    input[at_index] = Number {
        value: 0,
        depth: depth - 1,
    };

    true
}

fn split(input: &mut Vec<Number>, at_index: usize) -> bool {
    let Number { value, depth } = input[at_index];
    if value < 10 {
        return false;
    }

    let f = value as f64;
    input[at_index] = Number {
        value: (f / 2.0).floor() as u32,
        depth: depth + 1,
    };
    input.insert(
        at_index + 1,
        Number {
            value: (f / 2.0).ceil() as u32,
            depth: depth + 1,
        },
    );

    true
}

fn add(a: &Vec<Number>, b: &Vec<Number>) -> Vec<Number> {
    a.iter()
        .chain(b.iter())
        .map(|v| Number {
            value: v.value,
            depth: v.depth + 1,
        })
        .collect()
}
