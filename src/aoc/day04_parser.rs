use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct BingoCardNumber {
    pub number: u32,
    pub row: u8,
    pub column: u8,
}

#[derive(Debug)]
pub struct BingoCard {
    pub numbers: Vec<BingoCardNumber>,
}

#[derive(Debug)]
pub struct BingoInput {
    pub draw_numbers: Vec<u32>,
    pub bingo_cards: Vec<BingoCard>,
}

pub fn parse_bingo_input(filename: &str) -> Result<BingoInput, ParserError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let raw_draw_numbers = lines.next().ok_or(ParserError::ReadDrawNumbersError)??;
    let draw_numbers: Vec<_> = raw_draw_numbers
        .split(',')
        .map(|f| u32::from_str_radix(f, 10))
        .collect::<Result<_, _>>()?;

    let mut bingo_cards = vec![];
    'cards: loop {
        // Blank line
        lines.next();
        let mut card_numbers = vec![];
        let mut row = 0;
        while let Some(line) = lines.next() {
            let line = line?;
            let row_numbers: Vec<_> = line
                .split_whitespace()
                .map(|f| u32::from_str_radix(f, 10))
                .collect::<Result<Vec<_>, _>>()?;
            let mut row_numbers = row_numbers
                .iter()
                .enumerate()
                .map(|f| BingoCardNumber {
                    number: *f.1,
                    row: row,
                    column: f.0 as u8,
                })
                .collect();
            card_numbers.append(&mut row_numbers);
            if row == 4 {
                bingo_cards.push(BingoCard {
                    numbers: card_numbers,
                });
                continue 'cards;
            }
            row += 1;
        }
        break;
    }

    Ok(BingoInput {
        draw_numbers,
        bingo_cards,
    })
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to read input file")]
    ReadFileError(#[from] std::io::Error),
    #[error("Could not read draw numbers from first line")]
    ReadDrawNumbersError,
    #[error("Failed to parse a number")]
    IntParse(#[from] std::num::ParseIntError),
}
