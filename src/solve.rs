use crate::{
    bitboard::{BitboardExt, COLUMNS, ROWS, SQUARES},
    Board,
};

impl Board {
    fn find_best_pos(&self) -> Option<usize> {
        let mut best_pos = None;
        let mut best_count = Board::SIZE + 1;

        for pos in 0..Board::AREA {
            let count = self.count[pos] as usize;
            if count == 0 {
                continue;
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

        for digit in 0..Board::SIZE {
            if self.possibilities[digit].is_bit_set(pos) {
                // do
                let affected_mask = self.possibilities[digit]
                    & (COLUMNS[x] | ROWS[y] | SQUARES[x / 3 + (y / 3) * 3]);

                let copied_count = self.count.clone();

                self.possibilities[digit].clear(affected_mask);
                self.occupancies[digit].set_bit(pos);
                self.update_count(affected_mask, pos);

                if self.solve() {
                    return true;
                }

                // undo
                self.occupancies[digit].clear_bit(pos);
                self.possibilities[digit] |= affected_mask;
                self.count = copied_count;
            }
        }
        false
    }
}
