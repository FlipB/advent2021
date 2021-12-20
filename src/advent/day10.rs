use anyhow::Result;

use super::input;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let input_lines = input::get_input_lines(input)?;

    let mut high_scores: Vec<i64> = vec![];
    let mut illegals: Vec<char> = vec![];

    'outer: for line in input_lines {
        let mut stack = ChunkStack::new();
        for c in line.chars() {
            if let Err(_) = stack.push(c) {
                illegals.push(c);
                continue 'outer;
            }
        }

        let _closer = stack.autocomplete_close();
        let score = stack
            .autocomplete_close()
            .chars()
            .fold(0i64, |mut score, c| {
                score = score * 5;
                score += match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("unexpected illegal {}", c),
                };
                score
            });
        high_scores.push(score);
    }

    let sum: u64 = illegals
        .iter()
        .map(|&c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("unexpected illegal {}", c),
        })
        .sum();

    println!("Sum = {}", sum);

    high_scores.sort();
    let mid_score = high_scores.get((high_scores.len() + 0) / 2).unwrap();
    println!("Middle score = {}", mid_score);

    Ok(())
}

struct ChunkStack {
    v: Vec<Chunk>,
}

impl ChunkStack {
    fn new() -> Self {
        ChunkStack { v: vec![] }
    }

    fn push(&mut self, c: char) -> Result<()> {
        // check if char will close the "top" chunk
        if let Some(chunk) = self.v.last() {
            if chunk.is_closer(c) {
                // remove chunk from the stack
                self.v.pop();
                return Ok(());
            }
        }
        // try to make a new chunk from the char then.
        // If it's not a new chunk it's a parser error since it
        // wasn't a closer for the top chunk
        let chunk: Chunk = c.try_into()?;
        self.v.push(chunk);
        Ok(())
    }

    fn autocomplete_close(&self) -> String {
        self.v.iter().rev().fold(String::new(), |mut s, chunk| {
            s.push(chunk.close);
            s
        })
    }
}

struct Chunk {
    open: char,
    close: char,
}

impl Chunk {
    fn is_closer(&self, c: char) -> bool {
        self.close == c
    }
}

impl std::convert::TryFrom<char> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        const CHUNK_OPENER: &str = "({[<";
        const CHUNK_CLOSER: &str = ")}]>";
        let iter = CHUNK_OPENER.chars().zip(CHUNK_CLOSER.chars());
        for (open, close) in iter {
            if value == open {
                return Ok(Chunk { open, close });
            }
        }
        return Err(anyhow::Error::msg("not a chunk open char"));
    }
}
