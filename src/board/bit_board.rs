use std::fmt::{Debug, Display};

use crate::board::ArrayBoard as _;

/// A solitaire board represented as one single 64-bit integer.
///
/// The bytes of the integer represent the rows of an 8x8 bit matrix. The first
/// 7x7 bits of that matrix are used to represent the board:
///
/// ```text
///   0 1 2 3 4 5 6 7
/// 0 . . x x x . . .
/// 1 . . x x x . . .
/// 2 x x x x x x x .
/// 3 x x x x x x x .
/// 4 x x x x x x x .
/// 5 . . x x x . . .
/// 6 . . x x x . . .
/// 7 . . . . . . . .
/// ```
///
/// > Note that only 33 locations are valid, so the remaining 31 bits are
/// > treated as meaningless. To guarantee this meaninglessness, they should
/// > always be 0. Not all operations are safe without that invariant.
///
/// ## Generating legal moves
///
/// This implementation makes heavy use of the advantages the bit representation
/// offers and the properties a legal move has.
///
/// ### Legal move properties
///
/// - A legal move always operates on exactly 3 locations in a strait line.
///     - A legal move can be made anywhere on the board, where a strait line of
///       3 locations exists.
///     - All possible locations where a strait line of 3 locations exists are
///       known ahead of time.
/// - Executing a move is the same as inverting all 3 locations in the line. (An
///   occupied location becomes empty, an empty location becomes occupied.)
///
/// ### Bit matrix properties
///
/// - Shifting a bit matrix mask by 1 bit to the left or right corresponds to
///   translating the mask by 1 location to the right or left on the board.
/// - Shifting a bit matrix mask by 8 bits to the left or right corresponds to
///   translating the mask by 1 row downward or upward on the board.
///
/// ### Algorithm
///
/// The algorithm works by sweeping a 3-bit mask across all possible locations
/// on the board, where a 3 bits long line would fit on the board. This is done
/// by shifting a corresponding mask by a specified amount of bits, each
/// iteration. This amount is specified ahead of time.
///
/// Since there are two different masks for a line of 3 (horizontal and
/// vertical), that result in a different sweeping table, the algorithm needs
/// two passes.
///
/// Each locations needs to be matched against the two patterns in which a legal
/// move can be made. Since the execution of both moves is the same (inverting
/// the same 3 bits), the algorithm only needs to check if either of them
/// matches and then execute the move by inverting the 3 bits that are selected
/// by the mask.
///
/// The algorithm could be improved by computing the mask for each iteration
/// ahead of time. But since the compiler performs heavy optimizations, it
/// likely already applies that kind of optimization.
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

    pub fn enforce_invariant(&mut self) {
        self.0 &= Self::MASK;
    }

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

        const SHIFT_TABLE: &[u8] = &[2, 8, 6, 1, 1, 1, 1, 4, 1, 1, 1, 1, 4, 1, 1, 1, 1, 6, 8];

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

        const SHIFT_TABLE_V: &[u8] = &[2, 1, 1, 6, 1, 1, 4, 1, 1, 1, 1, 1, 1, 4, 1, 1, 6, 1, 1];

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
