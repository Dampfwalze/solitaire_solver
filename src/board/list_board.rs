use std::fmt::Debug;
use std::fmt::Display;

use crate::board::ArrayBoard as _;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Board([bool; 33]);

impl Board {
    const LINE_OFFSET: [usize; 7] = [2, 2, 0, 0, 0, 2, 2];

    pub fn index_for([x, y]: [isize; 2]) -> Option<usize> {
        if !matches!((y, x), (0..2, 2..5) | (2..5, 0..7) | (5..7, 2..5)) {
            return None;
        }

        let idx_in_line = x as usize - Self::LINE_OFFSET[y as usize];
        let idx = Self::ROW_START_IDX[y as usize] + idx_in_line;
        Some(idx)
    }
}

impl super::ArrayBoard for Board {
    const ROW_START_IDX: [usize; 7] = [0, 3, 6, 13, 20, 27, 30];

    fn get_idx(&self, idx: usize) -> bool {
        self.0[idx]
    }

    fn set_idx(&mut self, idx: usize, value: bool) -> bool {
        std::mem::replace(&mut self.0[idx], value)
    }
}

impl super::Board for Board {
    fn new_start() -> Self {
        let mut board = [true; 33];
        board[33 / 2] = false;

        Self(board)
    }

    fn get(&self, [x, y]: [isize; 2]) -> Option<bool> {
        Self::index_for([x, y]).map(|idx| self.get_idx(idx))
    }

    fn set(&mut self, idx: [isize; 2], value: bool) -> Option<bool> {
        Self::index_for(idx).map(|idx| self.set_idx(idx, value))
    }

    fn get_legal_moves(&self) -> Vec<Self> {
        let mut result: Vec<Board> = vec![];

        for y in 0..7isize {
            for x in 0..7isize {
                let Some(true) = self.get([x, y]) else {
                    continue;
                };

                for [dir_x, dir_y] in [[1, 0], [0, 1], [-1, 0], [0, -1]] {
                    let neighbor_a = [x + dir_x, y + dir_y];
                    let neighbor_b = [x + dir_x * 2, y + dir_y * 2];

                    let first = self.get(neighbor_a);
                    if !matches!(first, Some(true)) {
                        continue;
                    }

                    let second = self.get(neighbor_b);
                    if !matches!(second, Some(false)) {
                        continue;
                    }

                    // Construct new state
                    let mut new_board = *self;
                    new_board.set([x, y], false).unwrap();
                    new_board.set(neighbor_a, false).unwrap();
                    new_board.set(neighbor_b, true).unwrap();

                    result.push(new_board);
                }
            }
        }

        result
    }

    fn marble_count(&self) -> u32 {
        self.0.iter().filter(|v| **v).count() as u32
    }

    fn mirror_horizontal(&self) -> Self {
        let mut board = *self;
        for (&start, &len) in Self::ROW_START_IDX.iter().zip(Self::ROW_LENGTH.iter()) {
            board.0[start..start + len].reverse();
        }
        board
    }

    fn mirror_vertical(&self) -> Self {
        let mut board = *self;
        for y in 0..3 {
            for x in 0..Self::ROW_LENGTH[y] {
                let idx_a = Self::ROW_START_IDX[y] + x;
                let idx_b = Self::ROW_START_IDX[6 - y] + x;
                board.0.swap(idx_a, idx_b);
            }
        }
        board
    }

    fn transpose(&self) -> Self {
        let mut board = *self;
        for y in 0..7 {
            for x in y + 1..7 {
                let (Some(idx_a), Some(idx_b)) = (Self::index_for([x, y]), Self::index_for([y, x]))
                else {
                    continue;
                };
                board.0.swap(idx_a, idx_b);
            }
        }
        board
    }
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
        assert_eq!(Board::index_for([2, 0]), Some(0));
        assert_eq!(Board::index_for([2, 1]), Some(3));
        assert_eq!(Board::index_for([2, 2]), Some(8));
        assert_eq!(Board::index_for([3, 3]), Some(16));
        assert_eq!(Board::index_for([7, 2]), None);
    }

    #[test]
    fn test_start() {
        let board = Board::new_start();
        assert_eq!(board.get([3, 3]), Some(false));
        assert_eq!(board.0.iter().filter(|&&x| x).count(), 32);
    }

    #[test]
    fn test_mirror_horizontal() {
        let mut board = Board([false; 33]);
        board.set([3, 0], true);
        board.set([0, 3], true);

        let mut mirrored = Board([false; 33]);
        mirrored.set([3, 0], true);
        mirrored.set([6, 3], true);

        assert_eq!(board.mirror_horizontal(), mirrored);
    }

    #[test]
    fn test_mirror_vertical() {
        let mut board = Board([false; 33]);
        board.set([3, 0], true);
        board.set([0, 3], true);

        let mut mirrored = Board([false; 33]);
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
    fn test_transpose() {
        // Identity
        let mut board = Board([false; 33]);
        board.set([0, 0], true);
        board.set([1, 1], true);
        board.set([2, 2], true);
        board.set([3, 3], true);
        board.set([4, 4], true);
        board.set([5, 5], true);
        board.set([6, 6], true);

        assert_eq!(board.transpose(), board);

        // Corners
        let mut board = Board([false; 33]);
        board.set([7, 0], true);

        let mut transposed = Board([false; 33]);
        transposed.set([0, 7], true);

        assert_eq!(board.transpose(), transposed);
    }
}
