use core::panic;
use std::{
    fs::File,
    io::{self, Read},
};

use crate::consts::{self, SIZE};

// use crate::consts::CHECKS;

type PossibilitiesBits = u16; // lowest bit is 1

#[inline(always)]
fn add_possibility(possibilities: PossibilitiesBits, value: u8) -> PossibilitiesBits {
    possibilities | 0b1 << (value - 1) // sets the bit associated with the value
}

#[inline(always)]
fn remove_possibility(possibilities: PossibilitiesBits, value: u8) -> PossibilitiesBits {
    possibilities & !(0b1 << (value - 1)) // resets the bit associated with the value
}

#[inline(always)]
fn toggle_possibility(possibilities: PossibilitiesBits, value: u8) -> PossibilitiesBits {
    possibilities ^ (0b1 << (value - 1))
}

#[inline(always)]
pub fn check_possibility(possibilities: PossibilitiesBits, value: u8) -> bool {
    (possibilities >> (value - 1)) & 0b1 == 1 // checks if value is present by shifting the req'd value
                                              // is the right most bit, clearing all but the req'd bit, and checking if its on
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    Unknown(PossibilitiesBits),
    Solved(u8),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Unknown(0b00000000)
    }
}

pub type CellIndex = usize;

#[derive(Default, Clone, Copy)]
pub enum State {
    Incomplete,
    #[default]
    Complete,
}

#[derive(Clone)]
pub struct Sudoku {
    pub board: [Cell; SIZE],
    pub state: State,
}

impl Default for Sudoku {
    fn default() -> Self {
        Sudoku {
            board: [Cell::default(); SIZE],
            state: State::default(),
        }
    }
}

pub struct AlreadySolved;
pub struct InvalidCellSolution {
    cell_attempted: CellIndex,
    conflicting_cell: CellIndex,
    value: u8,
}

impl InvalidCellSolution {
    fn new(
        cell_attempted: CellIndex,
        conflicting_cell: CellIndex,
        value: u8,
    ) -> InvalidCellSolution {
        InvalidCellSolution {
            cell_attempted,
            conflicting_cell,
            value,
        }
    }
}

impl Sudoku {
    pub fn from_file(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut buf = String::new();
        let _ = file.read_to_string(&mut buf)?;
        buf = buf.replace("\n", "");
        let mut cells = [Cell::default(); SIZE];
        let new_cells: Vec<Cell> = buf
            .chars()
            .map(|c| match c {
                '0' => Cell::default(),
                c @ '1'..='9' => Cell::Solved(c.to_digit(10).unwrap().try_into().unwrap()),
                c => panic!("Invalid character found in file: '{}'. Characters must be a space or digit 1-9.", c),
            })
            .collect();
        assert!(
            cells.len() == new_cells.len(),
            "Expected file length of {}, file had length {}",
            cells.len(),
            new_cells.len()
        );
        cells[..new_cells.len()].copy_from_slice(&new_cells[..]);
        Ok(Sudoku {
            board: cells,
            state: State::default(),
        })
    }

    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn solve_cell(&mut self, idx: CellIndex, value: u8) -> Result<(), InvalidCellSolution> {
        if let Some(j) = consts::CHECKS[idx]
            .iter()
            .find(|&&j| matches!(self.board[j], Cell::Solved(v) if v == value))
        {
            return Err(InvalidCellSolution::new(idx, *j, value));
        }
        self.board[idx] = Cell::Solved(value);
        consts::CHECKS[idx].map(|j| self.remove_poss(j, value).unwrap_or(()));
        Ok(())
    }

    #[inline]
    pub fn unsolve_cell(&mut self, idx: CellIndex) {
        self.board[idx] = Cell::Unknown(0b0);
    }

    #[inline]
    pub fn remove_poss(&mut self, idx: CellIndex, value: u8) -> Result<(), AlreadySolved> {
        match self.board[idx] {
            Cell::Unknown(poss_bits) => {
                let new = remove_possibility(poss_bits, value);
                self.board[idx] = Cell::Unknown(new);
                if new.count_ones() == 1 {
                    let val = new.trailing_zeros() + 1;
                    let _ = self.solve_cell(idx, val.try_into().unwrap());
                }
                Ok(())
            }
            Cell::Solved(_) => Err(AlreadySolved),
        }
    }

    #[inline]
    pub fn add_poss(&mut self, idx: CellIndex, value: u8) -> Result<(), AlreadySolved> {
        match self.board[idx] {
            Cell::Unknown(poss_bits) => {
                self.board[idx] = Cell::Unknown(add_possibility(poss_bits, value));
                Ok(())
            }
            Cell::Solved(_) => Err(AlreadySolved),
        }
    }

    #[inline]
    pub fn toggle_poss(&mut self, idx: CellIndex, value: u8) -> Result<(), AlreadySolved> {
        match self.board[idx] {
            Cell::Unknown(poss_bits) => {
                self.board[idx] = Cell::Unknown(toggle_possibility(poss_bits, value));
                // TODO: count ones and if theres only 1 left, call solve_cell
                Ok(())
            }
            Cell::Solved(_) => Err(AlreadySolved),
        }
    }
}
