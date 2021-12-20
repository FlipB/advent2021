use std::collections::HashMap;

use anyhow::Result;

use super::input;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let input_lines = input::get_input_lines(input)?;

    let result_part_1 = input_lines
        .iter()
        .filter_map(|line| line.split_once(" | "))
        .map(|(_input_values, output_values)| {
            output_values
                .split_terminator(" ")
                .filter_map(|s| match s.len() {
                    i @ 7 | i @ 4 | i @ 3 | i @ 2 => Some(i),
                    _ => None,
                })
        })
        .flatten()
        .count();

    // part 1
    println!("Part 1: {}", result_part_1);

    //

    let mut sum = 0;
    for (i, line) in input_lines.iter().enumerate() {
        println!("line {}", i);
        let (signals, outputs) = line.split_once(" | ").unwrap();
        let signals = signals.split_terminator(" ").collect::<Vec<&str>>();
        let mut s = Sleuth {
            known: [None; 10],
            buf: ['\0'; 7],
        };
        let mapping = s.infer(signals);

        let mut acc = 0;
        let mut mul = 1000 as i64;
        for word in outputs.split(" ") {
            let mut mapped_word = String::new();
            for c in word.chars() {
                mapped_word.push(*mapping.get(&c).unwrap())
            }

            println!("Mapped: {} = {}", word, mapped_word.as_str());
            // abcefg
            acc += to_number(mapped_word.as_str()).unwrap() as i64 * mul;
            mul /= 10;
        }
        println!("{} = {}", outputs, acc);
        sum += acc;
    }
    println!("Sum = {}", sum);

    Ok(())
}

struct Sleuth<'a> {
    known: [Option<&'a str>; 10],
    buf: [char; 7],
}

impl<'a> Sleuth<'a> {
    fn infer(&mut self, signals: Vec<&'a str>) -> HashMap<char, char> {
        for sig in signals.iter() {
            match sig.len() {
                2 => self.known[1] = Some(sig),
                3 => self.known[7] = Some(sig),
                4 => self.known[4] = Some(sig),
                5 => {
                    // 2, 3 or 5.
                }
                6 => {
                    // 0, 6 or 9.
                }
                7 => self.known[8] = Some(sig),
                _ => (),
            }
        }
        loop {
            let known_count = self.known.iter().filter(|x| x.is_some()).count();
            if known_count == 10 {
                break;
            }
            for sig in signals.iter() {
                match sig.len() {
                    5 => {
                        // 2, 3 or 5.

                        // find 2 by checking number of differances with 4.
                        if self.remainder(4, sig) == 3 {
                            self.known[2] = Some(sig);
                            continue;
                        }

                        // find 3 by checking that the difference with 8 does not intersect 1
                        let n = self.remainder(7, sig);
                        if n == 2 {
                            self.known[3] = Some(sig);
                            continue;
                        }

                        // must be 5.
                        self.known[5] = Some(sig);
                    }
                    6 => {
                        // 0, 6 or 9.

                        // find 6 by checking the diff with 1
                        if self.remainder(1, sig) == 5 {
                            self.known[6] = Some(sig);
                            continue;
                        }

                        // find 0 by checking diff with 4
                        if self.remainder(4, sig) == 3 {
                            self.known[0] = Some(sig);
                            continue;
                        }

                        // must be 9
                        self.known[9] = Some(sig);
                    }
                    _ => (),
                }
            }
        }

        /*
        2: 1
        3: 7
        4: 4
        5: 2, 3, 5
        6: 0, 6, 9
        7: 8
        */

        // a = top
        // b = top left
        // c = top right
        // d = middle
        // e = bottom left
        // f = bottom right
        // g = bottom

        let mut mapping: HashMap<char, char> = HashMap::new();
        // remainder of 7 and 1 is a
        mapping.insert(
            Self::unique(self.known[7].unwrap(), &[self.known[1].unwrap()]),
            'a',
        );
        // remainder of 4 and 3, 2 is b
        mapping.insert(
            Self::unique(
                self.known[4].unwrap(),
                &[self.known[3].unwrap(), self.known[2].unwrap()],
            ),
            'b',
        );
        // remainder of 1 and 6 is c
        mapping.insert(
            Self::unique(self.known[1].unwrap(), &[self.known[6].unwrap()]),
            'c',
        );
        // remainder of 8 and 0 is d
        mapping.insert(
            Self::unique(self.known[8].unwrap(), &[self.known[0].unwrap()]),
            'd',
        );
        // remainder of 8 and 9 is e
        mapping.insert(
            Self::unique(self.known[8].unwrap(), &[self.known[9].unwrap()]),
            'e',
        );
        // remainder of 3 and 2 is f
        mapping.insert(
            Self::unique(self.known[3].unwrap(), &[self.known[2].unwrap()]),
            'f',
        );
        // remainder of 5 and 7, 4 is g
        mapping.insert(
            Self::unique(
                self.known[5].unwrap(),
                &[self.known[7].unwrap(), self.known[4].unwrap()],
            ),
            'g',
        );

        mapping
    }

    fn remainder(&mut self, n: u8, sig: &str) -> usize {
        let mut i = 0;
        if let Some(num_string) = &self.known[n as usize] {
            for c in sig.chars() {
                if !num_string.contains(c) {
                    self.buf[i] = c;
                    i += 1;
                }
            }
            i
        } else {
            // nothing to match against, everything is the remainder
            sig.len()
        }
    }

    fn unique(input: &str, mask: &[&str]) -> char {
        let s = mask.iter().fold(input.to_owned(), |remainder, sig| {
            let mut new_remainder = String::new();
            for c in remainder.chars() {
                if !sig.contains(c) {
                    new_remainder.push(c)
                }
            }
            new_remainder
        });
        if s.len() != 1 {
            panic!("too many chars");
        }
        s.chars().next().unwrap()
    }
}

fn signal_to_number(sig: &str) -> Option<u8> {
    match sig {
        "abcefg" => Some(0),
        "cf" => Some(1),
        "acdeg" => Some(2),
        "acdfg" => Some(3),
        "bcdf" => Some(4),
        "abdfg" => Some(5),
        "abdefg" => Some(6),
        "acf" => Some(7),
        "abcdefg" => Some(8),
        "abcdfg" => Some(9),
        _ => None,
    }
}

fn number_to_signal(n: u8) -> Option<&'static str> {
    match n {
        0 => Some("abcefg"),
        1 => Some("cf"),
        2 => Some("acdeg"),
        3 => Some("acdfg"),
        4 => Some("bcdf"),
        5 => Some("abdfg"),
        6 => Some("abdefg"),
        7 => Some("acf"),
        8 => Some("abcdefg"),
        9 => Some("abcdfg"),
        _ => None,
    }
}

/// CharSet is a set consisting of characters.
/// Allows for checking equality of two &str's without sorting.
#[derive(Debug, Eq)]
struct CharSet<'a>(&'a str);

impl<'a> PartialEq for CharSet<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for c in self.0.chars() {
            if !other.0.contains(c) {
                return false;
            }
        }
        true
    }
}

fn to_number(s: &str) -> Option<u8> {
    // a = top
    // b = top left
    // c = top right
    // d = middle
    // e = bottom left
    // f = bottom right
    // g = bottom
    match &CharSet(s) {
        // Note the "x if x == " this is required to avoid destructuring the CharSet type,
        // before PartialEq impl is invoked on it.
        x if x == &CharSet("abcefg") => Some(0),
        x if x == &CharSet("cf") => Some(1),
        x if x == &CharSet("acdeg") => Some(2),
        x if x == &CharSet("acdfg") => Some(3),
        x if x == &CharSet("bcdf") => Some(4),
        x if x == &CharSet("abdfg") => Some(5),
        x if x == &CharSet("abdefg") => Some(6),
        x if x == &CharSet("acf") => Some(7),
        x if x == &CharSet("abcdefg") => Some(8),
        x if x == &CharSet("abcdfg") => Some(9),
        _ => None,
    }
}

#[test]
fn test_charset_eq() {
    assert!(CharSet("dbagf") == CharSet("abdfg")); // OK!

    let ok = match CharSet("dbagf") {
        // Note the "x if x == " this is required to avoid destructuring the CharSet type,
        // before PartialEq impl is invoked on it.
        x if x == CharSet("abdfg") => true,
        _ => false,
    };
    assert!(ok); // OK!
}
