use csv;
use std::num;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::borrow::{BorrowMut, Borrow};
use serde::Deserialize;
use rand::Rng;
use std::ops::{Index, IndexMut, Rem};
use core::fmt::Alignment::Center;
use std::thread::current;
use std::collections::HashSet;
use rand::prelude::ThreadRng;
use std::hash::Hash;


fn main() {
    let mut board = read_puzzle(String::from("sample_puzzle.csv"));
    println!("{:?}", board);
    println!("{}", &board[Cell{row: 0, col: 0}]);
    println!("{}", &board[Cell{row: 4, col: 2}]);
    println!("{}", &board[Cell{row: 1, col: 1}]);

    let mut rng = rand::thread_rng();

    solve(board.borrow_mut(), Cell{row: 0, col: 0}, rng);
}

fn solve(curr_board: &mut Board, last_modified_cell: Cell, mut rng: ThreadRng) {
    for (ii, row) in curr_board.board.clone().into_iter().enumerate() {
        if (ii < last_modified_cell.row) {
            continue;
        }
        for (jj, col) in row.iter().enumerate() {
            while (curr_board[Cell{row: ii, col: jj}] == 0) {
                let temp = rng.gen_range(1, curr_board.size + 1);
                curr_board[Cell{row: ii, col: jj}] = temp;
            }
        }
    }
}

fn check_intermediate_puzzle(curr_board: Board, last_modified_cell: Cell) -> bool {
    if (!has_unique_elements(curr_board.get_row((&last_modified_cell).row))) {
        return false;
    } else if (!has_unique_elements(curr_board.get_col((&last_modified_cell).col))) {
        return false;
    } else if (!has_unique_elements(curr_board.get_subsquare(last_modified_cell))) {
        return false;
    }
    true
}

/// Checks any sort of iterable to see if there are any duplicates.
/// Copied and pasted shamelessly from: https://stackoverflow.com/a/46767732/3991562
fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}


#[derive(Debug, Deserialize)]
struct Board {
    board: Vec<Vec<i8>>,
    size: i8,
    subsquare_size: usize
}

impl Board{
    fn new(nums: Vec<Vec<i8>>) -> Board {
        Board{
            size: nums.len() as i8,
            subsquare_size: (nums.len() as f64).sqrt() as usize,
            board: nums,
        }
    }

    fn get_row(&self, row: usize) -> Vec<i8> {
        // let stuff = &self.board.clone()[row].iter().filter(|&&x| x != 0).cloned().collect() as &Vec<i8>
        // match stuff.len() {
        //     0 => None,
        //     _ => Some(stuff)
        // }
        self.board.clone()[row].iter().map(|&x| x).filter(|&x| x != 0).into_iter().collect()
    }

    fn get_col(&self, col: usize) -> Vec<i8> {
        self.board.clone().iter().map(|row| row[col]).filter(|&x| x != 0).collect()
    }

    fn get_subsquare(&self, cell: Cell) -> Vec<i8> {
        let subsquare_row_idx = cell.row.rem(self.subsquare_size as usize);
        let subsq_row_start = subsquare_row_idx * self.subsquare_size;
        let subsq_row_end = subsq_row_start + self.subsquare_size;

        let subsquare_col_idx = cell.col.rem(self.subsquare_size as usize);
        let subsq_col_start = subsquare_col_idx * self.subsquare_size;
        let subsq_col_end = subsq_col_start + self.subsquare_size;

        // subsquare = [row[subsq_col_start:subsq_col_end] for row in curr_board[subsq_row_start:subsq_row_end]];
        //
        // // Flatten list of lists into single list and ignore zeros;
        // subsquare_non_zeros = [item for sublist in subsquare for item in sublist if item != 0];
        // let subsquare = self.board.iter().enumerate().filter(|(ii, row)|
        //     (subsq_row_start..subsq_row_end).contains(&ii))
        //     .flatten()
        //     .filter(|&&x| x != 0)
        //     .collect();
        let mut subsquare: Vec<i8> = Vec::new();
        for row in self.board[subsq_row_start..subsq_row_end].iter() {
            subsquare.extend_from_slice(&row[subsq_col_start..subsq_col_end]);
        }

        subsquare
    }

    // fn Index(self: Self, row: usize, col: usize) -> i8 {
    //     self.board[row][col]
    // }
    //
    // fn IndexMut(self: Self, row: usize, col: usize) -> i8 {
    //     self.board[row][col]
    // }
}

struct Cell {
    row: usize,
    col: usize,
}

impl Index<Cell> for Board {
    type Output = i8;
    // fn index(&self, row: i32, col: i32) -> &i8 {
    fn index(&self, cell: Cell) -> &i8 {
        &self.board[cell.row][cell.col]
    }
}

impl IndexMut<Cell> for Board {
    // fn index(&self, row: i32, col: i32) -> &i8 {
    fn index_mut(&mut self, cell: Cell) -> &mut i8 {
        &mut self.board[cell.row][cell.col]
    }
}


// impl Iterator for Board {
//     type Item = Vec<i8>;
//
//     fn next(&mut self) -> Option<&Vec<i8>> {
//         self.board.iter().next().expect("poopsies")
//     }
// }

#[derive(Deserialize)]
struct Board_only {
    board: Vec<Vec<i8>>
}

fn read_puzzle(file_name: String) -> Board {
    // let mut file = File::open(file_name)?;
    let mut reader = csv::Reader::from_path(&file_name).expect("poop0");
    let mut rows: Vec<Vec<i8>> = vec![];

    for mut row in reader.deserialize(){
        let mut values: Vec<i8> = row.expect("poop1");
        rows.push(values);
    }


    Board {
        size: rows.len() as i8,
        subsquare_size: (rows.len() as f64).sqrt() as usize,
        board: rows,
    }
}
