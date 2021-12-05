use anyhow::Error;
use anyhow::Result;

use super::input;

struct BingoBoard {
    numbers: Vec<BingoNumber>,
}

impl BingoBoard {
    const SIZE: usize = 5;

    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &BingoNumber>> {
        (0..Self::SIZE).map(move |n| self.row(n))
    }

    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &BingoNumber>> {
        (0..Self::SIZE).map(move |n| self.column(n))
    }

    fn row(&self, n: usize) -> impl Iterator<Item = &BingoNumber> {
        let n = n % (self.numbers.len() / Self::SIZE);
        self.numbers[n * Self::SIZE..(n + 1) * Self::SIZE].iter()
    }

    fn row_mut(&mut self, n: usize) -> impl Iterator<Item = &mut BingoNumber> {
        let n = n % (self.numbers.len() / Self::SIZE);
        self.numbers[n * Self::SIZE..(n + 1) * Self::SIZE].iter_mut()
    }

    fn column(&self, n: usize) -> impl Iterator<Item = &BingoNumber> {
        let n = n % (self.numbers.len() / Self::SIZE);
        self.numbers.iter().enumerate().filter_map(move |(i, x)| {
            if (i % Self::SIZE) == n {
                Some(x)
            } else {
                None
            }
        })
    }

    fn column_mut(&mut self, n: usize) -> impl Iterator<Item = &mut BingoNumber> {
        let n = n % (self.numbers.len() / Self::SIZE);
        self.numbers
            .iter_mut()
            .enumerate()
            .filter_map(
                move |(i, x)| {
                    if (i % Self::SIZE) == n {
                        Some(x)
                    } else {
                        None
                    }
                },
            )
    }

    /// score will return the score and the number of draws to win or None.
    pub fn score(&self, draw: &[BingoNumber]) -> Option<(Score, usize)> {
        if draw.len() < 5 {
            return None;
        }
        for n in 5..draw.len() {
            let draw = &draw[0..n];
            if self.bingo(draw) {
                return Some((self.calc_score(draw), n));
            }
        }
        None
    }

    fn bingo(&self, draw: &[BingoNumber]) -> bool {
        for row in self.rows() {
            let winning = row.take_while(|x| draw.contains(x)).count() == Self::SIZE;
            if winning {
                return true;
            }
        }
        for col in self.columns() {
            let winning = col.take_while(|x| draw.contains(x)).count() == Self::SIZE;
            if winning {
                return true;
            }
        }

        false
    }

    fn calc_score(&self, draw: &[BingoNumber]) -> Score {
        let winning_number = draw.last().or(Some(&0)).unwrap();
        let sum_of_remaining: u32 = self.numbers.iter().filter(|x| !draw.contains(x)).sum();
        sum_of_remaining * winning_number
    }
}

type Score = u32;
type BingoNumber = u32;

pub fn print_result(input: impl std::io::Read) -> Result<()> {
    let (draw, boards) = parse_bingo_input(input)?;

    let (winning, losing) = get_winning_and_losing_board(boards, draw.as_slice());
    if let Some(winning) = winning {
        let (score, draw) = winning.score(draw.as_slice()).unwrap();
        println!("Bingo board won on draw {} with score {}", draw, score);
    }
    if let Some(losing) = losing {
        let (score, draw) = losing.score(draw.as_slice()).unwrap();
        println!("Bingo board lost on draw {} with score {}", draw, score);
    }
    Ok(())
}

fn get_winning_and_losing_board(
    mut boards: Vec<BingoBoard>,
    draw: &[BingoNumber],
) -> (Option<BingoBoard>, Option<BingoBoard>) {
    let mut winning = None;
    let mut losing = None;

    for n in 5..draw.len() {
        let draw = &draw[0..n];

        // iterate boards and remove winners, tracking first and last winner
        boards.retain(|b| {
            if let Some((_, _)) = b.score(draw) {
                if winning.is_none() {
                    winning = Some(BingoBoard {
                        numbers: b.numbers.to_owned(),
                    });
                } else {
                    losing = Some(BingoBoard {
                        numbers: b.numbers.to_owned(),
                    });
                }
                false
            } else {
                true
            }
        });
        if boards.len() == 0 {
            break;
        }
    }

    (winning, losing)
}

fn parse_bingo_input(input: impl std::io::Read) -> Result<(Vec<BingoNumber>, Vec<BingoBoard>)> {
    let lines = input::get_input_lines(input)?;
    let mut iter = lines.iter().map(|s| s.as_str());
    // First line is the draw, followed by new line.
    let draw_line = iter.next().ok_or(Error::msg("error"))?;
    let draw_numbers: Vec<BingoNumber> = draw_line
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut boards: Vec<BingoBoard> = vec![];
    loop {
        let numbers = parse_board_numbers(&mut iter)?;
        if numbers == None {
            break;
        }
        let numbers = numbers.unwrap();
        let board = BingoBoard { numbers };
        boards.push(board);
    }

    Ok((draw_numbers, boards))
}

fn parse_board_numbers<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
) -> Result<Option<Vec<BingoNumber>>> {
    let board_lines = lines.skip(1).take(5);
    let numbers: Vec<BingoNumber> = board_lines
        .map(|s| s.split(" "))
        .flatten()
        .filter_map(|x| x.parse().ok())
        .collect();

    if numbers.len() == 0 {
        return Ok(None);
    }
    if numbers.len() != BingoBoard::SIZE * BingoBoard::SIZE {
        return Err(anyhow::Error::msg("error parsing number in bingo board"));
    }

    Ok(Some(numbers))
}
