use crate::bitboard::{self, BitboardExt, COLUMNS, ROWS, SQUARES};

pub struct Board {
    pub possibilities: [u128; Board::SIZE],
    pub occupancies: [u128; Board::SIZE],
}

impl Board {
    pub const SIZE: usize = 9;
    pub const AREA: usize = 81;

    pub fn new() -> Self {
        let possibilities = [bitboard::MAX; Board::SIZE];

        let occupancies = [0; Board::SIZE];

        Self {
            possibilities,
            occupancies,
        }
    }

    pub fn set(&mut self, board_string: &str) -> Result<(), String> {
        if board_string.len() != Board::AREA {
            return Err(format!(
                "Invalid length: expected {}, got {}",
                Board::AREA,
                board_string.len()
            ));
        }
        if !board_string
            .chars()
            .all(|c| (c.is_digit(10) && c != '0') || c == '.')
        {
            return Err("Invalid characters in input string".to_string());
        }

        self.clear();

        for (i, c) in board_string.chars().enumerate() {
            if c != '.' {
                let x = i % Board::SIZE;
                let y = i / Board::SIZE;

                let digit = c.to_digit(10).unwrap() as usize - 1;

                if !self.possibilities[digit].is_bit_set(i) {
                    return Err(format!(
                        "Digit {} cannot be placed at position {}",
                        digit + 1,
                        i
                    ));
                }

                self.occupancies[digit].set_bit(i);

                let affected_mask = self.possibilities[digit]
                    & (COLUMNS[x] | ROWS[y] | SQUARES[x / 3 + (y / 3) * 3]);

                self.possibilities[digit].clear(affected_mask);
            }
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.possibilities = [bitboard::MAX; Board::SIZE];
        self.occupancies = [0; Board::SIZE];
    }

    pub fn occupancy(&self) -> u128 {
        self.occupancies.iter().fold(0, |acc, o| acc | o)
    }

    pub fn print(&self) {
        println!("*-----------------------*");
        for y in 0..Board::SIZE {
            if y % 3 == 0 && y != 0 {
                println!("|-------+-------+-------|")
            }
            for x in 0..Board::SIZE {
                let index = x + y * Board::SIZE;
                let value = (0..Board::SIZE)
                    .find_map(|i| {
                        if self.occupancies[i].is_bit_set(index) {
                            Some((i + 1).to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or(".".to_string());

                if x % 3 == 0 {
                    print!("| ")
                }

                print!("{} ", value);
            }
            println!("|");
        }
        println!("*-----------------------*");
    }
}
