#![allow(dead_code)]

use std::{fs, io::Write};

use crate::board::*;

mod board;
mod solver;

fn main() {
    let board = bit_board::Board::new_start();

    // Table header
    println!(
        " {:<13} │ {:<13} │ {:<15} │ {:<26} │ {:<26}",
        "Marbles left",
        "Ending states",
        "Ways to solve",
        "Unique intermediate boards",
        "Unique intermediate ways"
    );

    let start_time = std::time::Instant::now();

    fs::create_dir_all("solutions").expect("Failed to create solutions directory");
    let mut index_file =
        fs::File::create("solutions/index.md").expect("Failed to create index file");

    let solution_count = solver::breadth_first_solver::Solver::new(board)
        .inspect(|result| {
            // Table row
            println!(
                " {:13} │ {:13} │ {:15} │ {:26} │ {:26}",
                result.marbles_left,
                result.solutions.len(),
                result
                    .solutions
                    .iter()
                    .map(|s| s.ways_to_solve)
                    .sum::<usize>(),
                result.intermediate_unique_boards,
                result.intermediate_unique_ways,
            );

            if !result.solutions.is_empty() {
                fs::create_dir_all(format!("solutions/solutions_{}", result.marbles_left))
                    .expect("Failed to create solution directory");

                write!(
                    index_file,
                    "## Solutions with {} marbles left\n\n",
                    result.marbles_left
                )
                .expect("Failed to write to index file");
            }

            // Write solutions to files. Max 20 per layer.
            for (idx, solution) in result.solutions.iter().enumerate().take(20) {
                fs::write(
                    format!(
                        "solutions/solutions_{}/solution_{}.md",
                        result.marbles_left, idx
                    ),
                    format!(
                        "## Solution {idx}

| Steps | Marbles left | Ways to solve |
|--------------|---------------|--------------|
| {} | {} | {} |

```
{solution}
```",
                        solution.steps.len() - 1,
                        result.marbles_left,
                        solution.ways_to_solve
                    ),
                )
                .expect("Failed to write solution file");

                writeln!(
                    index_file,
                    "- [Solution {}](solutions_{}/solution_{}.md) with {} ways to solve",
                    idx, result.marbles_left, idx, solution.ways_to_solve
                )
                .expect("Failed to write to index file");
            }
        })
        .map(|result| result.solutions.len())
        .sum::<usize>();

    println!(
        "Found {} solutions in {:.2} seconds!",
        solution_count,
        start_time.elapsed().as_secs_f32()
    );
}
