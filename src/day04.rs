use anyhow::{Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Default, Debug)]
struct Board {
    numbers: [[u8; 5]; 5],
    finished: bool,
}

impl Board {
    #[inline]
    fn is_marked(&self, pos_x: usize, pos_y: usize) -> bool {
        self.numbers[pos_y][pos_x] == 0
    }

    fn mark(&mut self, number: u8) {
        for row in 0..5 {
            for column in 0..5 {
                if self.numbers[row][column] == number {
                    self.numbers[row][column] = 0;
                    return;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        'loop_rows: for row in 0..5 {
            for column in 0..5 {
                if !self.is_marked(column, row) {
                    continue 'loop_rows;
                }
            }
            return true;
        }
        'loop_columns: for column in 0..5 {
            for row in 0..5 {
                if !self.is_marked(column, row) {
                    continue 'loop_columns;
                }
            }
            return true;
        }
        return false;
    }

    fn calc_score(&self, marked_number: u8) -> u32 {
        let mut sum: u32 = 0;

        for row in 0..5 {
            for column in 0..5 {
                sum += self.numbers[row][column] as u32;
            }
        }

        sum * marked_number as u32
    }
}

pub fn main() -> Result<()> {
    let file = File::open("input/input-4-2.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().into_iter().map(|l| l.unwrap());

    let numbers = read_numbers(&lines.next().context("Missing first line")?)?;
    let mut boards = read_boards(&mut lines)?;

    for number in numbers {
        println!("[ {} ]", number);
        for board in &mut boards {
            if board.finished {
                continue;
            }

            board.mark(number);

            if board.has_won() {
                let score = board.calc_score(number);
                println!("The board has won with score {}:\n{:?}", score, board);

                board.finished = true;
            }
        }
    }

    Ok(())
}

fn read_numbers(line: &String) -> Result<Vec<u8>> {
    line.split(",")
        .map(|s| s.parse::<u8>().context("Couldn't parse number as u8"))
        .collect::<Result<Vec<u8>>>()
}

fn read_boards(lines: &mut dyn Iterator<Item = String>) -> Result<Vec<Board>> {
    let mut result = Vec::new();

    while let Some(board) = read_board(lines)? {
        result.push(board);
    }

    Ok(result)
}

fn read_board(lines: &mut dyn Iterator<Item = String>) -> Result<Option<Board>> {
    // Ignore first empty line
    if lines.next().is_none() {
        return Ok(None);
    }

    let mut result = Board::default();

    for row in 0..5 {
        let line = lines.next().context("")?;
        let numbers = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u8>().context("Couldn't parse number"));

        for (i, number) in numbers.enumerate() {
            result.numbers[row][i] = number?;
        }
    }

    Ok(Some(result))
}
