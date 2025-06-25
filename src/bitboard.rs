use crate::Board;

pub const MAX: u128 = 2417851639229258349412351;

pub const ROWS: [u128; Board::SIZE] = [
    511,
    261632,
    133955584,
    68585259008,
    35115652612096,
    17979214137393152,
    9205357638345293824,
    4713143110832790437888,
    2413129272746388704198656,
];

pub const COLUMNS: [u128; Board::SIZE] = [
    4731607904558235517441,
    9463215809116471034882,
    18926431618232942069764,
    37852863236465884139528,
    75705726472931768279056,
    151411452945863536558112,
    302822905891727073116224,
    605645811783454146232448,
    1211291623566908292464896,
];

pub const SQUARES: [u128; Board::SIZE] = [
    1838599,
    14708792,
    117670336,
    246772580483072,
    1974180643864576,
    15793445150916608,
    33121255085135066300416,
    264970040681080530403328,
    2119760325448644243226624,
];

pub trait BitboardExt {
    fn set_bit(&mut self, pos: usize);
    fn clear_bit(&mut self, pos: usize);
    fn clear(&mut self, mask: u128);
    fn is_bit_set(self, pos: usize) -> bool;
}

impl BitboardExt for u128 {
    fn set_bit(&mut self, pos: usize) {
        *self |= 1 << pos;
    }

    fn clear_bit(&mut self, pos: usize) {
        *self &= !(1 << pos)
    }

    fn clear(&mut self, mask: u128) {
        *self &= !mask;
    }

    fn is_bit_set(self, pos: usize) -> bool {
        self & (1 << pos) != 0
    }
}
