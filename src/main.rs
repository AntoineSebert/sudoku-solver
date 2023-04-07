use std::{
    ops::RangeInclusive,
    sync::{Arc, Mutex},
};

const VALUES: RangeInclusive<u8> = 1..=9;

enum Cell {
    Number(u8),
    Guess((u8, Vec<u8>)),
    Values(Vec<u8>),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Values(VALUES.collect())
    }
}

type Line = [Cell; 9];
type Board = [[Cell; 9]; 9];
type Square<'a> = [[&'a Cell; 3]; 3];

trait IsDone {
    fn is_done(self) -> bool;
}

fn remove_if_contains(values: &mut Vec<u8>, cell: &Cell) -> bool {
    if let Cell::Number(n) = cell {
        if let Some(pos) = values.iter().position(|i| *i == *n) {
            values.remove(pos);
            return true;
        }
    }

    false
}

impl IsDone for Board {
    fn is_done(self) -> bool {
        // lines
        for i0 in 0..=8 {
            let mut row_values: Vec<_> = VALUES.collect();
            let mut col_values: Vec<_> = VALUES.collect();

            for i1 in 0..=8 {
                // for each row
                if !remove_if_contains(&mut row_values, &self[i0][i1]) {
                    return false;
                }

                // for each column
                if !remove_if_contains(&mut col_values, &self[i1][i0]) {
                    return false;
                }
            }
        }

        // squares
        for square_row in 0..=2 {
            for square_col in 0..=2 {
                let square: Square = [
                    [
                        &self[square_row * 3][square_col * 3],
                        &self[square_row * 3][(square_col * 3) + 1],
                        &self[square_row * 3][(square_col * 3) + 2],
                    ],
                    [
                        &self[(square_row * 3) + 1][square_col * 3],
                        &self[(square_row * 3) + 1][(square_col * 3) + 1],
                        &self[(square_row * 3) + 1][(square_col * 3) + 2],
                    ],
                    [
                        &self[(square_row * 3) + 2][square_col * 3],
                        &self[(square_row * 3) + 2][(square_col * 3) + 1],
                        &self[(square_row * 3) + 2][(square_col * 3) + 2],
                    ],
                ];

                if !square.is_done() {
                    return false;
                }
            }
        }

        true
    }
}

impl IsDone for Line {
    fn is_done(self) -> bool {
        let mut values: Vec<_> = VALUES.collect();

        for cell in self {
            if !remove_if_contains(&mut values, &cell) {
                return false;
            }
        }

        true
    }
}

impl IsDone for Square<'_> {
    fn is_done(self) -> bool {
        let mut values: Vec<_> = VALUES.collect();

        for row in self {
            for cell in row {
                if !remove_if_contains(&mut values, &cell) {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
    let _board = Arc::new(Mutex::<&mut Board>::new(&mut Board::default()));
    // runs a future for each square
}
