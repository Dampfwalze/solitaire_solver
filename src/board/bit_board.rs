use std::fmt::{Debug, Display};

use crate::board::ArrayBoard as _;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board(u64);

impl Board {
    #[allow(clippy::identity_op)]
    const MASK: u64 = 0
        | 0b00011100
        | 0b00011100 << 8
        | 0b01111111 << 16
        | 0b01111111 << 24
        | 0b01111111 << 32
        | 0b00011100 << 40
        | 0b00011100 << 48
        | 0b00000000 << 56;

    const WIDTH: usize = 7;
    const HEIGHT: usize = 7;

    pub fn index_for([x, y]: [isize; 2]) -> Option<usize> {
        if x < 0 || x >= Self::WIDTH as isize || y < 0 || y >= Self::HEIGHT as isize {
            return None;
        }
        let idx = x + y * 8;
        if Self::MASK >> idx & 1 == 0 {
            None
        } else {
            Some(idx as usize)
        }
    }
}

impl super::ArrayBoard for Board {
    const ROW_START_IDX: [usize; 7] = [2, 10, 16, 24, 32, 42, 50];

    fn get_idx(&self, idx: usize) -> bool {
        self.0 >> idx & 1 == 1
    }

    fn set_idx(&mut self, idx: usize, value: bool) -> bool {
        let old = self.0 >> idx & 1 == 1;
        if value {
            self.0 |= 1 << idx;
        } else {
            self.0 &= !(1 << idx);
        }
        old
    }
}

impl super::Board for Board {
    fn new_start() -> Self {
        Self((!(1 << (3 * 8 + 3))) & Self::MASK)
    }

    fn get(&self, [x, y]: [isize; 2]) -> Option<bool> {
        Self::index_for([x, y]).map(|idx| self.get_idx(idx))
    }

    fn set(&mut self, [x, y]: [isize; 2], value: bool) -> Option<bool> {
        Self::index_for([x, y]).map(|idx| self.set_idx(idx, value))
    }

    #[allow(clippy::identity_op)]
    fn get_legal_moves(&self) -> Vec<Board> {
        let mut result: Vec<Board> = vec![];

        let board = self.0;

        let mut mask = 0b111;
        let mut pattern_a = 0b110;
        let mut pattern_b = 0b011;

        const SHIFT_TABLE: &[usize] = &[2, 8, 6, 1, 1, 1, 1, 4, 1, 1, 1, 1, 4, 1, 1, 1, 1, 6, 8];

        for shift in SHIFT_TABLE {
            mask <<= shift;
            pattern_a <<= shift;
            pattern_b <<= shift;

            // println!("{}", Board(mask));

            let masked = board & mask;
            if masked == pattern_a || masked == pattern_b {
                let new_board = board ^ mask;
                result.push(Board(new_board));
            }
        }

        let mut mask = 1 | 1 << 8 | 1 << 16;
        let mut pattern_a = 1 | 1 << 8 | 0 << 16;
        let mut pattern_b = 0 | 1 << 8 | 1 << 16;

        const SHIFT_TABLE_V: &[usize] = &[2, 1, 1, 6, 1, 1, 4, 1, 1, 1, 1, 1, 1, 4, 1, 1, 6, 1, 1];

        for shift in SHIFT_TABLE_V {
            mask <<= shift;
            pattern_a <<= shift;
            pattern_b <<= shift;

            // println!("{}", Board(mask));

            let masked = board & mask;
            if masked == pattern_a || masked == pattern_b {
                let new_board = board ^ mask;
                result.push(Board(new_board));
            }
        }

        result
    }

    fn is_solved(&self) -> bool {
        self.0.count_ones() == 1
    }

    fn marble_count(&self) -> u32 {
        self.0.count_ones()
    }

    fn mirror_horizontal(&self) -> Self {
        Self(u64::from_ne_bytes(self.0.to_ne_bytes().map(|b| b.reverse_bits())) >> 1)
    }

    fn mirror_vertical(&self) -> Self {
        let mut bytes = self.0.to_ne_bytes();
        bytes[..7].reverse();
        Self(u64::from_ne_bytes(bytes))
    }

    fn transpose(&self) -> Self {
        Self(byte_transpose(self.0))
    }
}

fn byte_transpose(mut x: u64) -> u64 {
    let mut t;
    t = (x ^ (x >> 7)) & 0x00AA00AA00AA00AA;
    x = x ^ t ^ (t << 7);
    t = (x ^ (x >> 14)) & 0x0000CCCC0000CCCC;
    x = x ^ t ^ (t << 14);
    t = (x ^ (x >> 28)) & 0x00000000F0F0F0F0;
    x = x ^ t ^ (t << 28);
    x
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::ArrayBoard::fmt(self, f)
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::ArrayBoard::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board as _;

    use super::*;

    #[test]
    fn test_index_for() {
        assert_eq!(Board::index_for([0, 0]), None);
        assert_eq!(Board::index_for([5, 0]), None);
        assert_eq!(Board::index_for([2, 0]), Some(2));
        assert_eq!(Board::index_for([2, 1]), Some(10));
        assert_eq!(Board::index_for([2, 2]), Some(18));
        assert_eq!(Board::index_for([3, 3]), Some(3 * 8 + 3));
        assert_eq!(Board::index_for([7, 2]), None);
    }

    #[test]
    fn test_start() {
        let board = Board::new_start();
        assert_eq!(board.get([3, 3]), Some(false));
        assert_eq!(board.0.count_ones(), 32);
        assert_eq!((board.0 & Board::MASK).count_ones(), 32);
    }

    #[test]
    fn test_mirror_horizontal() {
        let mut board = Board(0);
        board.set([3, 0], true);
        board.set([0, 3], true);

        let mut mirrored = Board(0);
        mirrored.set([3, 0], true);
        mirrored.set([6, 3], true);

        assert_eq!(board.mirror_horizontal(), mirrored);
    }

    #[test]
    fn test_mirror_vertical() {
        let mut board = Board(0);
        board.set([3, 0], true);
        board.set([0, 3], true);

        let mut mirrored = Board(0);
        mirrored.set([3, 6], true);
        mirrored.set([0, 3], true);

        assert_eq!(board.mirror_vertical(), mirrored);
    }

    #[test]
    fn test_get_legal_moves() {
        let board = Board::new_start();
        let moves = board.get_legal_moves();
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_byte_transpose() {
        // Identity
        let x = 1 | (1 << 9) | (1 << 18) | (1 << 27) | (1 << 36) | (1 << 45) | (1 << 54);
        assert_eq!(byte_transpose(x), x);

        // Corners
        assert_eq!(byte_transpose(1 << 7), 1 << 56);
    }
}
