use std::collections::HashMap;

use aoc::day04_parser::{BingoCard, BingoCardNumber, BingoInput};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bingo() {
        let bingo_input = aoc::day04_parser::parse_bingo_input("input/day04/example").unwrap();
        let result = find_winning_card(&bingo_input, Winner::First);
        assert_eq!(result, 4512);
    }

    #[test]
    fn find_last_winning_card() {
        let bingo_input = aoc::day04_parser::parse_bingo_input("input/day04/example").unwrap();
        let result = find_winning_card(&bingo_input, Winner::Last);
        assert_eq!(result, 1924);
    }
}

#[derive(Debug, Clone)]
struct PlayingBingoCard {
    numbers: HashMap<u32, (BingoCardNumber, bool)>,
    rows: [u8; 5],
    columns: [u8; 5],
    completed: bool,
}

impl From<&BingoCard> for PlayingBingoCard {
    fn from(input: &BingoCard) -> Self {
        Self {
            numbers: HashMap::from_iter(
                input
                    .numbers
                    .clone()
                    .into_iter()
                    .map(|f| (f.number, (f, false))),
            ),
            rows: [0; 5],
            columns: [0; 5],
            completed: false,
        }
    }
}

#[derive(PartialEq)]
enum Winner {
    First,
    Last,
}
fn find_winning_card(input: &BingoInput, find: Winner) -> u32 {
    // convert to playable bingo cards
    let mut cards: Vec<_> = input
        .bingo_cards
        .iter()
        .map(|f| PlayingBingoCard::from(f))
        .collect();
    let number_of_cards = cards.len();
    let mut winning_card = None;
    let mut winning_number = 0;
    let mut completed = 0;
    'draw: for number in &input.draw_numbers {
        // Enter the number in each card
        for mut card in cards.iter_mut() {
            if card.completed {
                continue;
            }
            let pos = match card.numbers.get(number) {
                Some(p) => p.0,
                None => continue,
            };
            card.rows[pos.row as usize] += 1;
            card.columns[pos.column as usize] += 1;

            // Mark
            card.numbers.get_mut(number).unwrap().1 = true;

            // Check row or column completed
            if card.rows[pos.row as usize] == 5 || card.columns[pos.column as usize] == 5 {
                card.completed = true;
                winning_card = Some(card.clone());
                winning_number = *number;
                match find {
                    Winner::First => break 'draw,
                    Winner::Last => completed += 1,
                };
                if completed == number_of_cards {
                    break 'draw;
                }
            }
        }
    }

    let sum_of_unmarked: u32 = winning_card
        .unwrap()
        .numbers
        .iter()
        .filter(|f| !f.1 .1)
        .map(|f| *f.0)
        .sum();

    sum_of_unmarked * winning_number
}

fn main() {
    let bingo_input = aoc::day04_parser::parse_bingo_input("input/day04/input").unwrap();
    let result = find_winning_card(&bingo_input, Winner::First);
    println!("Part 1: {}", result);
    let result = find_winning_card(&bingo_input, Winner::Last);
    println!("Part 2: {}", result);
}
