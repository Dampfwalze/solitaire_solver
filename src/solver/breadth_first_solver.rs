use std::{collections::HashMap, fmt::Display};

use crate::board::Board;

pub struct Solution<B: Board> {
    pub steps: Vec<B>,
    pub ways_to_solve: usize,
}

pub struct LayerResult<B: Board> {
    pub marbles_left: usize,
    pub intermediate_unique_boards: usize,
    pub intermediate_unique_ways: usize,
    pub solutions: Vec<Solution<B>>,
}

struct BoardState<B: Board> {
    ways_to_solve: usize,
    steps: Vec<B>,
}

type LayerMap<B> = HashMap<B, BoardState<B>>;

pub struct Solver<B: Board> {
    marbles_left: usize,
    intermediate_states: LayerMap<B>,
}

impl<B: Board> Solver<B> {
    pub fn new(board: B) -> Self {
        Self {
            marbles_left: board.marble_count() as usize,
            intermediate_states: HashMap::from([(
                board.clone(),
                BoardState {
                    ways_to_solve: 1,
                    steps: vec![board],
                },
            )]),
        }
    }
}

impl<B: Board> Iterator for Solver<B> {
    type Item = LayerResult<B>;

    fn next(&mut self) -> Option<Self::Item> {
        let old_layer = std::mem::take(&mut self.intermediate_states);
        let mut new_layer: LayerMap<B> = HashMap::new();
        let mut solutions: Vec<Solution<B>> = Vec::new();

        for (board, state) in old_layer {
            let moves = board.get_legal_moves();

            if moves.is_empty() {
                solutions.push(Solution {
                    steps: state.steps.clone(),
                    ways_to_solve: state.ways_to_solve,
                });
            }

            'moves: for next in moves {
                // Skip symmetries
                for board in next.get_symmetries().iter().skip(1) {
                    if new_layer.contains_key(board) {
                        continue 'moves;
                    }
                }

                new_layer
                    .entry(next.clone())
                    .or_insert_with(|| BoardState {
                        ways_to_solve: 0,
                        steps: [&state.steps[..], &[next][..]].concat(),
                    })
                    .ways_to_solve += state.ways_to_solve;
            }
        }

        if solutions.is_empty() && new_layer.is_empty() {
            None
        } else {
            let result = LayerResult {
                marbles_left: self.marbles_left,
                intermediate_unique_boards: new_layer.len(),
                intermediate_unique_ways: new_layer.values().map(|state| state.ways_to_solve).sum(),
                solutions,
            };

            self.intermediate_states = new_layer;
            self.marbles_left -= 1;

            Some(result)
        }
    }
}

impl<B: Board> Display for Solution<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, step) in self.steps.iter().enumerate() {
            writeln!(f, "Step {}:\n{}", idx, step)?;
        }
        Ok(())
    }
}
