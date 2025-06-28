use crate::bitboard::{self, BitboardExt, COLUMNS, ROWS, SQUARES};

pub struct Board {
    pub possibilities: [u128; Board::SIZE],
    pub occupancies: [u128; Board::SIZE],
    pub count: [usize; Board::AREA],
}

impl Board {
    pub const SIZE: usize = 9;
    pub const AREA: usize = 81;

    pub fn new() -> Self {
        Self {
            possibilities: [bitboard::MAX; Board::SIZE],
            occupancies: [0; Board::SIZE],
            count: [Board::SIZE; Board::AREA],
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

        self.reset();

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

                let affected_mask = self.possibilities[digit]
                    & (COLUMNS[x] | ROWS[y] | SQUARES[x / 3 + (y / 3) * 3]);

                // update board
                self.occupancies[digit].set_bit(i);
                self.possibilities[digit].clear(affected_mask);
                self.update_count(affected_mask, i);
            }
        }
        Ok(())
    }

    pub fn update_count(&mut self, mut affected_mask: u128, pos: usize) {
        while affected_mask != 0 {
            let bit = affected_mask.trailing_zeros() as usize;
            affected_mask &= affected_mask - 1;

            if self.count[bit] > 0 {
                self.count[bit] -= 1;
            }
        }
        self.count[pos] = 0;
    }

    pub fn reset(&mut self) {
        self.possibilities = [bitboard::MAX; Board::SIZE];
        self.occupancies = [0; Board::SIZE];
        self.count = [Board::SIZE; Board::AREA];
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
