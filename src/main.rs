use csv;
use std::num;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::borrow::{BorrowMut, Borrow};
use serde::Deserialize;
use std::ops::{Index, IndexMut};


fn main() {
    let mut board = read_puzzle(String::from("sample_puzzle.csv"));
    println!("{:?}", board);
    println!("{}", &board[Cell{row: 0, col: 0}]);
    println!("{}", &board[Cell{row: 4, col: 2}]);
    println!("{}", &board[Cell{row: 1, col: 1}]);

    solve(board, Cell{row: 0, col: 0});
}

fn solve(curr_board: Board, last_modified_cell: Cell) {

}


#[derive(Debug, Deserialize)]
struct Board {
    board: Vec<Vec<i8>>,
    size: i8,
    subsquare_size: i8
}

impl Board{
    fn new(nums: Vec<Vec<i8>>) -> Board {
        Board{
            size: nums.len() as i8,
            subsquare_size: (nums.len() as f64).sqrt() as i8,
            board: nums,
        }
    }

    fn Index(self: Self, row: usize, col: usize) -> i8 {
        self.board[row][col]
    }

    fn IndexMut(self: Self, row: usize, col: usize) -> i8 {
        self.board[row][col]
    }
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
        subsquare_size: (rows.len() as f64).sqrt() as i8,
        board: rows,
    }
}
