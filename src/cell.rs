use std::fmt;

/// Either a determined nmber, or either a guess from a set of possible values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    N(u8),
    Val((Option<u8>, Vec<u8>)),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::N(n) => write!(f, " {} ", n),
            Cell::Val((guess, possibilities)) => {
                if let Some(guess) = guess {
                    write!(f, "({})", *guess)
                } else if possibilities.is_empty() {
                    write!(f, "   ")
                } else {
                    write!(f, "<{}>", possibilities.len())
                }
            }
        }
    }
}

impl Default for Cell {
    /// An empty grid.
    fn default() -> Self {
        Self::Val((None, super::VALUES.collect()))
    }
}

impl Cell {
    /// When a guess list contains a single value, we transform the [`Cell`] into a [`Cell::Number`] of this value.
    pub fn collapse(&mut self) -> bool {
        if let Cell::Val((_, possibilities)) = self {
            if let [value] = &possibilities[..] {
                *self = Cell::N(*value);
                return true;
            }
        }
        false
    }
}

impl TryFrom<&str> for Cell {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let [c] = &value.chars().collect::<Vec<_>>()[..] {
            Self::try_from(*c)
        } else {
            Err(format!("Invalid value in cell: {value}"))
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if ('1'..='9').contains(&value) {
            let n = u8::try_from(value.to_digit(10).unwrap()).unwrap(); // I swear bro it's safe bro fr
            Ok(Cell::N(n))
        } else {
            Err(format!("Invalid value in cell: {value}"))
        }
    }
}
