use crate::{
    bitboard::{BitboardExt, COLUMNS, ROWS, SQUARES},
    Board,
};

impl Board {
    fn find_best_pos(&self) -> Option<usize> {
        let mut best_pos = None;
        let mut best_count = 10;
        let occupancy = self.occupancy();

        for pos in 0..81 {
            if occupancy.is_bit_set(pos) {
                continue;
            }
            let mut count = 0;
            for digit in 0..9 {
                if self.possibilities[digit].is_bit_set(pos) {
                    count += 1;
                    if count >= best_count {
                        break;
                    }
                }
            }
            if count < best_count {
                best_count = count;
                best_pos = Some(pos);
                if best_count <= 1 {
                    break;
                }
            }
        }

        best_pos
    }

    pub fn solve(&mut self) -> bool {
        let pos = match self.find_best_pos() {
            Some(p) => p,
            None => return true,
        };

        let x = pos % Board::SIZE as usize;
        let y = pos / Board::SIZE as usize;

        for digit in 0..9 {
            if self.possibilities[digit].is_bit_set(pos) {
                // do
                let affected_mask = self.possibilities[digit]
                    & (COLUMNS[x] | ROWS[y] | SQUARES[x / 3 + (y / 3) * 3]);

                self.possibilities[digit].clear(affected_mask);
                self.occupancies[digit].set_bit(pos);

                if self.solve() {
                    return true;
                }

                // undo
                self.occupancies[digit].clear_bit(pos);
                self.possibilities[digit] |= affected_mask;
            }
        }
        false
    }
}
