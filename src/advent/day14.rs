use std::{collections::HashMap, hash::Hash};

use anyhow::Result;

use super::input;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let (polymer_string, mapping) = parse_input(input).unwrap();

    let mut polymer = polymer_string.chars().collect::<Vec<char>>();
    for _ in 0..10 {
        polymer = process_polymer(&mapping, &polymer);
        println!("{}", polymer.iter().collect::<String>());
    }
    println!(
        "Part 1: {}",
        calculate_the_thing(count_occurrance(&polymer).into_iter())
    );

    let mut polymer: HashMap<PolymerPair, u64> = HashMap::new();
    polymer_string
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(slice_into_pair)
        .for_each(|pair| *polymer.entry(pair).or_insert(0) += 1);

    let mut map: HashMap<char, u64> = HashMap::new();
    polymer_string
        .chars()
        .for_each(|c| *map.entry(c).or_insert(0) += 1);
    for _ in 0..40 {
        process_polymer2(&mapping, &mut polymer, &mut map);
    }
    println!("Part 2: {}", calculate_the_thing(map.into_iter()));

    Ok(())
}

fn calculate_the_thing(it: impl Iterator<Item = (char, u64)>) -> u64 {
    let mut v: Vec<(char, u64)> = it.collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    v.first().unwrap().1 - v.last().unwrap().1
}

fn parse_input(input: impl std::io::Read) -> Result<(String, HashMap<PolymerPair, PolymerTriple>)> {
    let lines = input::get_input_lines(input)?;
    let polymer = lines.get(0).unwrap().clone();
    let mut mapping: HashMap<PolymerPair, PolymerTriple> = HashMap::new();
    for line in lines.iter().skip(2) {
        let (from, insert) = line.split_once(" -> ").unwrap();
        let from_chars = PolymerPair(from.chars().next().unwrap(), from.chars().last().unwrap());
        let to_chars = PolymerTriple(
            from.chars().next().unwrap(),
            insert.chars().next().unwrap(),
            from.chars().last().unwrap(),
        );
        mapping.insert(from_chars, to_chars);
    }

    Ok((polymer, mapping))
}

fn process_polymer(mapping: &HashMap<PolymerPair, PolymerTriple>, polymer: &[char]) -> Vec<char> {
    flatten(
        polymer
            .windows(2)
            .map(slice_into_pair)
            .map(|pair| mapping.get(&pair).expect("all pairs have mappings").iter()),
    )
}

fn flatten(triples: impl Iterator<Item = impl Iterator<Item = char>>) -> Vec<char> {
    let mut vec = vec![];
    for mut t in triples {
        if !vec.is_empty() {
            // discard first char in triple
            let _c = t.next().unwrap();
        }
        for c in t {
            vec.push(c)
        }
    }
    vec
}

fn count_occurrance(polymer: &[char]) -> HashMap<char, u64> {
    let mut map: HashMap<char, u64> = HashMap::new();
    for c in polymer {
        *map.entry(*c).or_insert(0) += 1;
    }
    map
}

fn process_polymer2(
    mapping: &HashMap<PolymerPair, PolymerTriple>,
    polymer: &mut HashMap<PolymerPair, u64>,
    acc: &mut HashMap<char, u64>,
) {
    for (pair, n) in polymer.clone() {
        let triple = mapping.get(&pair).expect("all pairs have mappings");
        // track count per element
        *acc.entry(triple.introduced()).or_insert(0) += n;

        // update polymer length
        let (p1, p2) = triple.pairs();
        *polymer.entry(p1).or_insert(0) += n;
        *polymer.entry(p2).or_insert(0) += n;
        *polymer.entry(pair).or_insert(0) -= n;
    }
}

fn slice_into_pair(pair: &[char]) -> PolymerPair {
    PolymerPair(pair[0], pair[1])
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PolymerPair(char, char);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PolymerTriple(char, char, char);

impl PolymerTriple {
    /// certainly there must be some tuple iterator somewhere already??
    fn iter(&self) -> impl Iterator<Item = char> {
        let mut i = 0;
        let tup = *self;
        std::iter::from_fn(move || {
            let c = match i {
                0 => Some(tup.0),
                1 => Some(tup.1),
                2 => Some(tup.2),
                _ => None,
            };
            i += 1;
            c
        })
    }

    fn pairs(&self) -> (PolymerPair, PolymerPair) {
        (PolymerPair(self.0, self.1), PolymerPair(self.1, self.2))
    }

    /// introduced retuns the new, "middle" element
    fn introduced(&self) -> char {
        self.1
    }
}
