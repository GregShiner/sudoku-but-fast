use crate::consts;

// use crate::consts::CHECKS;

type PossibilitiesBits = u16; // lowest bit is 1

#[inline(always)]
fn add_possibility(possibilities: PossibilitiesBits, value: u8) -> PossibilitiesBits {
    possibilities | 0b1 << value - 1 // sets the bit associated with the value
}

#[inline(always)]
fn remove_possibility(possibilities: PossibilitiesBits, value: u8) -> PossibilitiesBits {
    possibilities & !(0b1 << value - 1) // resets the bit associated with the value
}

#[inline(always)]
fn check_possibility(possibilities: PossibilitiesBits, value: u8) -> bool {
    (possibilities >> value - 1) & 0b1 == 1 // checks if value is present by shifting the req'd value
                                            // is the right most bit, clearing all but the req'd bit, and checking if its on
}

#[inline(always)]
fn toggle_possibility(possibilities: u16, value: u8) -> u16 {
    possibilities ^ (0b1 << value - 1)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    Unknown(PossibilitiesBits),
    Solved(u8),
}

pub type CellIndex = usize;

pub struct Sudoku {
    pub board: [Cell; 81],
}

impl Sudoku {
    pub fn new() -> Self {
        Sudoku {
            board: [Cell::Unknown(0b111111111); 81],
        }
    }

    #[inline]
    pub fn solve_cell(&mut self, idx: CellIndex, value: u8) -> () {
        self.board[idx] = Cell::Solved(value);
        consts::CHECKS[idx].map(|j| self.remove_poss(j, value));
    }

    #[inline]
    pub fn unsolve_cell(&mut self, idx: CellIndex) -> () {
        self.board[idx] = Cell::Unknown(0b0);
    }

    #[inline]
    pub fn remove_poss(&mut self, idx: CellIndex, value: u8) -> Result<(), ()> {
        match self.board[idx] {
            Cell::Unknown(poss_bits) => {
                self.board[idx] = Cell::Unknown(remove_possibility(poss_bits, value));
                // TODO: count ones and if theres only 1 left, call solve_cell
                Ok(())
            }
            Cell::Solved(_) => Err(()),
        }
    }

    #[inline]
    pub fn add_poss(&mut self, idx: CellIndex, value: u8) -> Result<(), ()> {
        match self.board[idx] {
            Cell::Unknown(poss_bits) => {
                self.board[idx] = Cell::Unknown(add_possibility(poss_bits, value));
                // TODO: count ones and if theres only 1 left, call solve_cell
                Ok(())
            }
            Cell::Solved(_) => Err(()),
        }
    }

    #[inline]
    pub fn toggle_poss(&mut self, idx: CellIndex, value: u8) -> Result<(), ()> {
        match self.board[idx] {
            Cell::Unknown(poss_bits) => {
                self.board[idx] = Cell::Unknown(toggle_possibility(poss_bits, value));
                // TODO: count ones and if theres only 1 left, call solve_cell
                Ok(())
            }
            Cell::Solved(_) => Err(()),
        }
    }
}
