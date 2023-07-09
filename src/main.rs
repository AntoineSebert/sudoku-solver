mod cell;

use cell::Cell;

use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

/// The interval $[1; 9]$.
const VALUES: RangeInclusive<u8> = 1..=9;

type Line<'a> = [&'a Cell; 9];
type Square<'a> = [[&'a Cell; 3]; 3];

/// 2D array of [`Cell`].
type Board = [[Cell; 9]; 9];
type Coord = (u8, u8);

trait IsDone {
    /// Returns whether or not all the cells have been assigned to a different number in $[1; 9]$.
    fn is_done(&self) -> bool;
}

struct Neighbourhood {
    square: Vec<Coord>,
    row: Vec<Coord>,
    col: Vec<Coord>,
}

/// Returns whether or not a cell value has been removed from a set of values.
fn remove_if_contains(values: &mut Vec<u8>, cell: &Cell) -> bool {
    if let Cell::N(n) = cell {
        if let Some(pos) = values.iter().position(|i| *i == *n) {
            values.remove(pos);
            return true;
        }
    }

    false
}

/*
impl IsDone for Board {
    fn is_done(&self) -> bool {
        self.rows.iter().all(Line::is_done)
            && self.cols.iter().all(Line::is_done)
            && self
                .squares
                .iter()
                .all(|square_row| square_row.iter().all(Square::is_done))
    }
}
*/

impl IsDone for Line<'_> {
    fn is_done(&self) -> bool {
        let mut values: Vec<_> = VALUES.collect();

        for cell in self {
            if !remove_if_contains(&mut values, cell) {
                return false;
            }
        }

        true
    }
}

impl IsDone for Square<'_> {
    fn is_done(&self) -> bool {
        let mut values: Vec<_> = VALUES.collect();

        for row in self {
            for cell in row {
                if !remove_if_contains(&mut values, cell) {
                    return false;
                }
            }
        }

        true
    }
}

fn display(board: &Board) {
    for x in 0..board.len() {
        if x % 3 == 0 {
            println!("-------------------------------------");
        }

        print!("|");

        for cell in &board[x] {
            print!("{cell}|");
        }

        println!();
    }

    println!("-------------------------------------");
}

fn get_solved_cells(board: &Board) -> u8 {
    board
        .iter()
        .map(|row| row.iter().filter(|cell| matches!(cell, Cell::N(_))).count())
        .sum::<usize>() as u8
}

fn compute_neighbours() -> HashMap<Coord, HashSet<Coord>> {
    let mut neighbours = HashMap::<Coord, HashSet<Coord>>::new();

    for x in 0..=8 {
        for y in 0..=8 {
            let _neighbours = (0..=8)
                // row
                .map(|x_row| (x_row, y))
                // col
                .chain((0..=8).map(|y_col| (x, y_col)))
                // square
                .chain((0..=2).flat_map(|square_x| {
                    (0..=2).map(move |square_y| (square_x + (x % 3), square_y + (y % 3)))
                }))
                // remove target cell
                .filter(|coord| *coord != (x, y))
                .collect::<HashSet<Coord>>();

            neighbours.insert((x, y), _neighbours);
        }
    }

    neighbours
}

fn main() {
    use Cell::{Val, N};

    let df = Cell::default;

    let _sample = r#"
-------------------
|1|3| |4| |8| | |9|
| | |5|1|2| |7| |4|
| |9|4| | | |2|1| |
-------------------
|9| | | | | | |2|3|
| |8| | |4| | |9| |
|3|2| | | | | | |5|
-------------------
| |5|3| | | |9|7| |
|4| |9| |6|2|5| | |
|6| | |5| |7| |4|2|
-------------------
"#;

    let mut board: Board = [
        [N(1), N(3), df(), N(4), df(), N(8), df(), df(), N(9)],
        [df(), df(), N(5), N(2), N(1), df(), N(7), df(), N(4)],
        [df(), N(9), N(4), df(), df(), df(), N(2), N(1), df()],
        //
        [N(9), df(), df(), df(), df(), df(), df(), N(2), N(3)],
        [df(), N(8), df(), df(), N(4), df(), df(), N(9), df()],
        [N(3), N(2), df(), df(), df(), df(), df(), df(), N(5)],
        //
        [df(), N(5), N(3), df(), df(), df(), N(9), N(7), df()],
        [N(4), df(), N(9), df(), N(6), N(2), N(5), df(), df()],
        [N(6), df(), df(), N(5), df(), N(7), df(), N(4), N(2)],
    ];

    let mut solved_cells: u8 = get_solved_cells(&board);
    let mut pass = 0u64;

    let neighbourhood = compute_neighbours();

    while solved_cells < 81 {
        println!("Pass {pass} - solved {solved_cells}");
        display(&board);

        let mut new_solved_cells = 0;

        // reduce possibilities (simple)
        // rows
        for row in &mut board {
            let constraints = row
                .iter()
                .filter_map(|cell| if let N(n) = cell { Some(n) } else { None })
                .copied()
                .collect::<Vec<_>>();

            for cell in row {
                if let Val((_, possibilities)) = cell {
                    possibilities.retain(|e| !constraints.contains(e));
                }
            }
        }
        // columns
        for y in 0..=8 {
            let constraints = (0..=8)
                .map(|x| &board[x as usize][y as usize])
                .filter_map(|cell| if let N(n) = cell { Some(n) } else { None })
                .copied()
                .collect::<Vec<_>>();

            for cell in &mut board[y as usize] {
                if let Val((_, ref mut possibilities)) = cell {
                    possibilities.retain(|e| !constraints.contains(e));
                }
            }
        }
        // squares
        println!();
        let squares = (0..=2)
            .map(|x0| {
                (0..=2)
                    .map(move |y0| {
                        (0..=2)
                            .flat_map(move |x1| (0..=2).map(move |y1| (x0 * 3 + x1, y0 * 3 + y1)))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for square_row in squares {
            for mut square in square_row {
                let constraints = square
                    .iter()
                    .filter_map(|(x, y)| {
                        if let N(n) = board[*x][*y] {
                            Some(n)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                square.iter_mut().for_each(|(x, y)| {
                    if let Val((_, possibilities)) = &mut board[*x][*y] {
                        possibilities.retain(|e| !constraints.contains(&e));
                    }
                });
            }
        }

        // reduce possibilities (lines of same numbers)
        // rows
        // columns
        // squares

        // reduce single numbers
        for ((x, y), neighbours) in &neighbourhood {
            if let Val((_, possibilities)) = &board[*x as usize][*y as usize] {
                if possibilities.len() == 1 {
                    println!("1");
                    let neighbours_possibilities = neighbours
                        .iter()
                        .filter_map(|(n_x, n_y)| {
                            if let Val((_, possibilities)) = &board[*n_x as usize][*n_y as usize] {
                                Some(possibilities.clone())
                            } else {
                                None
                            }
                        })
                        .flatten()
                        .collect::<Vec<_>>();

                    let value = *possibilities.first().unwrap();

                    println!("({x}, {y}): {neighbours_possibilities:?}"); // BUG wrong neighbourhoods

                    if !neighbours_possibilities.contains(&value)
                        && board[*x as usize][*y as usize].collapse()
                    {
                        println!("collapse ({x}, {y}) to {value}");

                        new_solved_cells += 1;
                    }
                }
            }
        }

        if new_solved_cells == 0 {
            break;
        }

        solved_cells += new_solved_cells;
        pass += 1;
    }

    display(&board);
}
