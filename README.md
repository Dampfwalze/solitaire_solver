# Solver for the Solitaire game

[![Test](https://github.com/dampfwalze/solitaire_solver/actions/workflows/test.yaml/badge.svg?branch=master)](https://github.com/dampfwalze/solitaire_solver/actions/workflows/test.yaml?query=branch%3Amaster)
[![Docs](https://github.com/dampfwalze/solitaire_solver/actions/workflows/docs.yml/badge.svg?branch=master)](https://github.com/dampfwalze/solitaire_solver/actions/workflows/docs.yml?query=branch%3Amaster)

Solitaire is a game that is played by only one player. The game board has 33
locations, arranged in a cross shape. At the start, 32 of the locations are
occupied by pegs. Only the center location is empty. The player can move a peg
by jumping it over an adjacent peg into an empty location, and then removing the
jumped peg from the board. The goal of the game is to end up with only one peg
left on the board, ideally in the center location.

[![Solitaire board](https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Spielzug_von_Solit%C3%A4r.gif/330px-Spielzug_von_Solit%C3%A4r.gif)](https://en.wikipedia.org/wiki/Peg_solitaire)

- [Solutions](https://github.com/Dampfwalze/solitaire_solver/wiki)
- [Documentation](https://dampfwalze.github.io/solitaire_solver)

This project implements two board representations and two solving schemes,
namely breadth-first and depth-first search.

## Board representations

### [`list_board::Board`]

The board is represented with an array of 33 booleans, where each boolean
represents a location on the board. The array is densly packed, so every element
of the array represents a location on the board.

```rust,no_run
struct Board([bool; 33]);
```

### [`bit_board::Board`]

The board is represented with one `u64`, where its bytes represent the rows of
an 8x8 bit matrix. Only 7x7 locations are actually used up by the board, like
so:

```rust,no_run
struct Board(u64);
```

```text
  0 1 2 3 4 5 6 7 bits
0 . . ● ● ● . . .
1 . . ● ● ● . . .
2 ● ● ● ● ● ● ● .
3 ● ● ● ● ● ● ● .
4 ● ● ● ● ● ● ● .
5 . . ● ● ● . . .
6 . . ● ● ● . . .
7 . . . . . . . .
bytes
```

[`list_board::Board`]: https://dampfwalze.github.io/solitaire_solver/solitaire_solver/board/list_board/struct.Board.html
[`bit_board::Board`]: https://dampfwalze.github.io/solitaire_solver/solitaire_solver/board/bit_board/struct.Board.html
