use super::VALUES;

type Result = std::result::Result<(), Vec<u8>>;

trait Solvable {
    fn solve(self) -> Result;
}

// Cell ////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Cell {
    Number(u8),
    //Guess((u8, Vec<u8>)),
    Values(Vec<u8>),
}

impl Default for Cell {
    fn default() -> Self {
        Self::Values(VALUES.to_vec())
    }
}

impl Solvable for Cell {
    /// Internal consistency
    fn solve(mut self) -> Result {
        match self {
            Cell::Number(_) => Ok(()),
            Cell::Values(values) => match &values[..] {
                [] => panic!("Cell had no possible values"),
                [val] => Ok(self = Cell::Number(*val)),
                _ => Err(values),
            },
        }
    }
}

// Square //////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Square<'a> {
    cells: [[&'a Cell; 3]; 3],
    remaining: Vec<u8>,
    resolved: Vec<u8>,
}

impl<'a> From<[[&'a Cell; 3]; 3]> for Square<'a> {
    fn from(cells: [[&'a Cell; 3]; 3]) -> Self {
        Self {
            cells,
            remaining: VALUES.to_vec(),
            resolved: Vec::with_capacity(9),
        }
    }
}

impl Solvable for Square<'_> {
    fn solve(mut self) -> Result {
        let mut delta = Vec::<u8>::new();

        for row in self.cells {
            for cell in row {
                if let Cell::Number(n) = cell {
                    self.resolved.push(*n);
                    self.remaining.remove(
                        self.remaining
                            .iter()
                            .position(|x| *x == *n)
                            .expect(&format!("Cell had no possible value '{n}'")),
                    );
                }
            }
        }

        // repeat until no change
        // collect numbers
        // remove from possible values
        // remove from cells values

        // pass 1: internal consistency
        // pass 2: external consistency
        // for each cell Values
        todo!()
    }
}

// Line ////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Line<'a> {
    cells: [&'a Cell; 9],
    remaining: Vec<u8>,
    resolved: Vec<u8>,
}

impl<'a> From<[&'a Cell; 9]> for Line<'a> {
    fn from(cells: [&'a Cell; 9]) -> Self {
        Self {
            cells,
            remaining: VALUES.to_vec(),
            resolved: Vec::with_capacity(9),
        }
    }
}

impl Solvable for Line<'_> {
    fn solve(mut self) -> Result {
        // pass 1: internal consistency
        // pass 2: external consistency

        for cell in self.cells {
            if let Cell::Number(n) = cell {
                self.resolved.push(*n);
                self.remaining.remove(
                    self.remaining
                        .iter()
                        .position(|x| *x == *n)
                        .expect(&format!("Cell had no possible value '{n}'")),
                );
            }
        }
        todo!()
    }
}

// Board ///////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Board<'a> {
    cells: [[Cell; 9]; 9],
    rows: [Line<'a>; 9],
    columns: [Line<'a>; 9],
    squares: [[Square<'a>; 3]; 3],

    remaining_lines: Vec<&'a Line<'a>>,
    remaining_squares: Vec<&'a Square<'a>>,
}

impl<'a> From<[[Cell; 9]; 9]> for Board<'a> {
    fn from(cells: [[Cell; 9]; 9]) -> Self {
        let mut squares: Vec<[Square<'a>; 3]> = Vec::with_capacity(3);
        let mut rows: Vec<Line> = Vec::with_capacity(9);
        let mut cols: Vec<Line> = Vec::with_capacity(9);

        create_lines(&cells, &mut rows, &mut cols);

        for square_row in 0..=2 {
            for square_col in 0..=2 {
                squares[square_row][square_col] = Square::from([
                    [
                        &cells[square_row * 3][square_col * 3],
                        &cells[square_row * 3][(square_col * 3) + 1],
                        &cells[square_row * 3][(square_col * 3) + 2],
                    ],
                    [
                        &cells[(square_row * 3) + 1][square_col * 3],
                        &cells[(square_row * 3) + 1][(square_col * 3) + 1],
                        &cells[(square_row * 3) + 1][(square_col * 3) + 2],
                    ],
                    [
                        &cells[(square_row * 3) + 2][square_col * 3],
                        &cells[(square_row * 3) + 2][(square_col * 3) + 1],
                        &cells[(square_row * 3) + 2][(square_col * 3) + 2],
                    ],
                ]);
            }
        }

        Self {
            cells,
            rows: rows.try_into().unwrap(),
            columns: cols.try_into().unwrap(),
            squares: squares.try_into().unwrap(),
            remaining_lines: Vec::with_capacity(18),
            remaining_squares: Vec::with_capacity(9),
        }
    }
}

fn create_lines(
    cells: &'static [[Cell; 9]; 9],
    rows: &mut Vec<Line<'static>>,
    cols: &mut Vec<Line<'static>>,
) {
    for i0 in 0..=8 {
        let mut row_cells = Vec::new();
        let mut col_cells = Vec::new();

        for i1 in 0..=8 {
            row_cells.push(&cells[i0][i1]);
            col_cells.push(&cells[i1][i0]);
        }
        {
            let row: [&Cell; 9] = row_cells.try_into().unwrap();

            rows.push(Line::from(row));
        }
        {
            let col: [&Cell; 9] = col_cells.try_into().unwrap();

            cols.push(Line::from(col));
        }
    }
}

impl Solvable for Board<'_> {
    fn solve(self) -> Result {
        Ok(())
    }
}
