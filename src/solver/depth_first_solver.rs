use std::collections::HashSet;

use crate::board::Board;

pub struct Solution<B: Board> {
    pub steps: Vec<B>,
    pub marbles_left: usize,
}

pub struct Solver<B: Board> {
    visited: HashSet<B>,
    stack: Vec<B>,
    solutions: Vec<Solution<B>>,
}

impl<B: Board> Solver<B> {
    pub fn solve(board: B) -> Vec<Solution<B>> {
        let mut solver = Self {
            visited: HashSet::new(),
            stack: vec![board],
            solutions: Vec::new(),
        };
        solver.solve_recursive();
        solver.solutions.sort_by_key(|s| s.marbles_left);
        solver.solutions
    }

    fn solve_recursive(&mut self) {
        let Some(board) = self.stack.last() else {
            return;
        };

        for symmetry in board.get_symmetries() {
            if self.visited.contains(&symmetry) {
                return;
            }
        }

        self.visited.insert(board.clone());

        let moves = board.get_legal_moves();

        if moves.is_empty() {
            self.add_solution();
        }

        for next in moves {
            self.stack.push(next);
            self.solve_recursive();
            self.stack.pop();
        }
    }

    fn add_solution(&mut self) {
        let solution = Solution {
            steps: self.stack.clone(),
            marbles_left: self.stack.last().unwrap().marble_count() as usize,
        };
        self.solutions.push(solution);

        println!(
            "Found solution with {} marbles left:\n{}",
            self.stack.last().unwrap().marble_count(),
            self.stack.last().unwrap()
        );
    }
}
