use std::{collections::HashSet, fmt::Write};

use anyhow::Result;

use super::input;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let (dots, folds) = parse_input(input)?;

    for dot in dots.iter() {
        println!("{}, {}", dot.0, dot.1);
    }
    println!("\n");

    let mut first = dots.clone();
    let first_fold = folds.first().unwrap();
    first = first
        .into_iter()
        .filter_map(|d| first_fold.update_dot_position(d))
        .collect();
    for dot in first.iter() {
        println!("{}, {}", dot.0, dot.1);
    }
    println!("Part 1: Dots after first fold = {}", first.len());

    let mut second = dots.clone();
    for fold in folds.iter() {
        second = second
            .into_iter()
            .filter_map(|d| fold.update_dot_position(d))
            .collect();
    }
    render(second);

    Ok(())
}

type Dot = (i16, i16);
enum Fold {
    X(i16),
    Y(i16),
}

impl Fold {
    fn update_dot_position(&self, (dx, dy): Dot) -> Option<Dot> {
        match &self {
            Fold::X(x) => {
                if *x == dx {
                    None
                } else if dx > *x {
                    Some((x - (dx - x), dy))
                } else {
                    Some((dx, dy))
                }
            }
            Fold::Y(y) => {
                if *y == dy {
                    None
                } else if dy > *y {
                    Some((dx, y - (dy - y)))
                } else {
                    Some((dx, dy))
                }
            }
        }
    }
}

fn parse_input(input: impl std::io::Read) -> Result<(HashSet<Dot>, Vec<Fold>)> {
    let lines = input::get_input_lines(input)?;
    let mut dots: HashSet<Dot> = HashSet::new();
    let mut folds: Vec<Fold> = vec![];
    let mut read_folds = false;
    for line in lines {
        if line == "" {
            read_folds = true;
            continue;
        }
        if !read_folds {
            let dot: Dot = line
                .split_once(',')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap();
            dots.insert(dot);
        } else {
            parse_fold(&mut folds, line);
        }
    }

    Ok((dots, folds))
}

fn parse_fold(folds: &mut Vec<Fold>, line: String) {
    match line.split_once("=") {
        Some(("fold along x", x)) => folds.push(Fold::X(x.parse().unwrap())),
        Some(("fold along y", y)) => folds.push(Fold::Y(y.parse().unwrap())),
        None => return,
        _ => return,
    }
}

fn render(dots: HashSet<Dot>) {
    let mut max: Dot = (0, 0);
    for &dot in dots.iter() {
        if dot.0 > max.0 {
            max.0 = dot.0
        }
        if dot.1 > max.1 {
            max.1 = dot.1
        }
    }
    let mut s = String::new();
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            if dots.contains(&(x, y)) {
                s.write_char('#').unwrap();
            } else {
                s.write_char(' ').unwrap();
            }
        }
        s.write_char('\n').unwrap();
    }
    println!("{}", s)
}
