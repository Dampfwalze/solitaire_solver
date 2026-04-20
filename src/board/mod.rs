use std::fmt::Display;

pub mod bit_board;
pub mod list_board;

pub trait Board: Sized + Clone + Eq + std::hash::Hash + std::fmt::Debug + Display {
    fn new_start() -> Self;

    fn get(&self, idx: [isize; 2]) -> Option<bool>;
    fn set(&mut self, idx: [isize; 2], value: bool) -> Option<bool>;

    fn get_legal_moves(&self) -> Vec<Self>;

    fn marble_count(&self) -> u32;

    fn is_solved(&self) -> bool {
        self.marble_count() == 1
    }

    fn mirror_horizontal(&self) -> Self;
    fn mirror_vertical(&self) -> Self;
    fn transpose(&self) -> Self;

    fn get_symmetries(&self) -> [Self; 8] {
        let transposed = self.transpose();
        let mirrored_horizontal = self.mirror_horizontal();
        let transposed_mirrored_horizontal = transposed.mirror_horizontal();
        [
            self.clone(),
            mirrored_horizontal.clone(),
            self.mirror_vertical(),
            mirrored_horizontal.mirror_vertical(),
            transposed.clone(),
            transposed_mirrored_horizontal.clone(),
            transposed.mirror_vertical(),
            transposed_mirrored_horizontal.mirror_vertical(),
        ]
    }
}

/// Extension trait for boards that are stored in a consecutive fashion.
pub trait ArrayBoard: Board {
    const ROW_START_IDX: [usize; 7];
    const ROW_LENGTH: [usize; 7] = [3, 3, 7, 7, 7, 3, 3];

    fn get_idx(&self, idx: usize) -> bool;
    fn set_idx(&mut self, idx: usize, value: bool) -> bool;

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = |f: &mut std::fmt::Formatter<'_>, row: usize, len: usize| -> std::fmt::Result {
            let start = Self::ROW_START_IDX[row];
            for i in start..(start + len) {
                if self.get_idx(i) {
                    f.write_str(" ●")?;
                } else {
                    f.write_str(" ○")?;
                }
            }
            Ok(())
        };

        f.write_str("    ┌───────┐\n")?;

        f.write_str("    │")?;
        line(f, 0, 3)?;
        f.write_str(" │\n")?;

        f.write_str("┌───┘")?;
        line(f, 1, 3)?;
        f.write_str(" └───┐\n")?;

        f.write_str("│")?;
        line(f, 2, 7)?;
        f.write_str(" │\n")?;

        f.write_str("│")?;
        line(f, 3, 7)?;
        f.write_str(" │\n")?;

        f.write_str("│")?;
        line(f, 4, 7)?;
        f.write_str(" │\n")?;

        f.write_str("└───┐")?;
        line(f, 5, 3)?;
        f.write_str(" ┌───┘\n")?;

        f.write_str("    │")?;
        line(f, 6, 3)?;
        f.write_str(" │\n")?;

        f.write_str("    └───────┘\n")?;

        Ok(())
    }
}
