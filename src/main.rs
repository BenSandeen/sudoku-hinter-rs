use csv;
use std::borrow::{BorrowMut, Borrow};
use serde::Deserialize;
use std::ops::{Index, IndexMut, Rem};
use std::collections::HashSet;
use rand::prelude::ThreadRng;
use std::hash::Hash;
use rand::seq::SliceRandom;


fn main() {
    let mut board = read_puzzle(String::from("sample_puzzle.csv"));
    println!("{:?}", board);
    println!("{}", &board[Cell{row: 0, col: 0}]);
    println!("{}", &board[Cell{row: 4, col: 2}]);
    println!("{}", &board[Cell{row: 1, col: 1}]);

    let mut rng = rand::thread_rng();

    board.solve(Cell{row: 0, col: 0}, rng);
    println!("{:?}", board);

    board = read_puzzle(String::from("9x9_tough.csv"));
    board.solve(Cell{row: 0, col: 0}, rng);
    println!("{:?}", board);
    //
    // board = read_puzzle(String::from("16x16_sample_puzzle.csv"));
    // board.solve(Cell{row: 0, col: 0}, rng);
    // println!("{:?}", board);
    //
    // board = read_puzzle(String::from("16x16_another_puzzle.csv"));
    // board.solve(Cell{row: 0, col: 0}, rng);
    // println!("{:?}", board);
}

// fn solve(curr_board: &mut Board, last_modified_cell: Cell, mut rng: ThreadRng) {
//     for (ii, row) in curr_board.board.clone().into_iter().enumerate() {
//         if ii < last_modified_cell.row {
//             continue;
//         }
//
//         for (jj, &_col) in row.iter().enumerate() {
//             let mut untried_cell_values: Vec<i8> = curr_board.all_nums_to_match.clone();
//                 // .into_iter().filter(|x| !curr_board.get_row(ii).contains(x) &&
//                 // !curr_board.get_col(jj).contains(x) &&
//                 // !curr_board.get_subsquare(&Cell { row: ii, col: jj }).contains(x)).collect();
//                 // .into_iter().filter(|x| !curr_board.get_row(ii).contains(x)).collect();
//             // println!("untried_cell_values: {:?}", &untried_cell_values);
//             // println!("row {0}: {1:?}", ii, curr_board.get_row(ii));
//             // println!("col {0}: {1:?}", jj, curr_board.get_col(jj));
//             // println!("subsq {0}{1}: {2:?}", ii, jj, curr_board.get_subsquare(&Cell { row: ii, col: jj }));
//
//             while curr_board[Cell{row: ii, col: jj}] == 0 {
//                 // If there's no more valid numbers to try, backtrack and try previous cell again
//                 if untried_cell_values.len() == 0 {
//                     // println!("backtracking!");
//                     curr_board[last_modified_cell] = 0;
//                     return;
//                 }
//
//                 // let temp = rng.gen_range(1, curr_board.size + 1);
//                 let temp: i8 = match untried_cell_values.choose(&mut rng) {
//                     Some(&x) => x,
//                     None => break,
//                 };
//
//                 untried_cell_values.retain(|&x| x != temp);  // Remove `temp` from list
//                 curr_board[Cell{row: ii, col: jj}] = temp;
//
//                 if check_intermediate_puzzle(&curr_board, Cell{row: ii, col: jj})
//                 {
//                     solve(curr_board.borrow_mut(), Cell{row: ii, col: jj}, rng);
//                 } else {
//                     curr_board[Cell{row: ii, col: jj}] = 0i8;
//                 }
//             }
//         }
//     }
//     if check_complete_puzzle(curr_board) {
//         return
//     }
// }
//
// fn check_intermediate_puzzle(curr_board: &Board, last_modified_cell: Cell) -> bool {
//     // println!("Row: {:?}", curr_board.get_row((&last_modified_cell).row));
//     // println!("Col: {:?}", curr_board.get_col((&last_modified_cell).col));
//     // println!("SS: {:?}", curr_board.get_subsquare(&last_modified_cell));
//     if !has_unique_elements(curr_board.get_row((&last_modified_cell).row)) {
//         return false;
//     } else if !has_unique_elements(curr_board.get_col((&last_modified_cell).col)) {
//         return false;
//     } else if !has_unique_elements(curr_board.get_subsquare(&last_modified_cell)) {
//         return false;
//     }
//     true
// }
//
// fn check_complete_puzzle(curr_board: &Board) -> bool {
//     for row in curr_board.board.iter() {
//         if !curr_board.all_nums_to_match.iter().all(|x| row.contains(x)) {
//             return false;
//         }
//     }
//     for col_idx in 0..curr_board.size as usize {
//         let col = curr_board.borrow().get_col(col_idx);
//         if !curr_board.all_nums_to_match.iter().all(|x| col.contains(x)) {
//             return false;
//         }
//     }
//     for subsquare_row_idx in 0..curr_board.subsquare_size {
//         for subsquare_col_idx in 0..curr_board.subsquare_size {
//             let subsquare = curr_board.get_subsquare(&Cell{row: subsquare_row_idx, col: subsquare_col_idx});
//             if !curr_board.all_nums_to_match.iter().all(|x| subsquare.contains(x)) {
//                 return false;
//             }
//         }
//     }
//
//     return true
// }

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
    size: i8,
    subsquare_size: usize,
    all_nums_to_match: Vec<i8>,
    board: Vec<Vec<i8>>,
}

impl Board{
    fn new(nums: Vec<Vec<i8>>) -> Board {
        Board {
            size: nums.len() as i8,
            subsquare_size: (nums.len() as f64).sqrt() as usize,
            all_nums_to_match: (1..nums.len() as i8 + 1).collect(),
            board: nums,
        }
    }

    fn solve(&mut self, last_modified_cell: Cell, mut rng: ThreadRng) {
        let mut untried_cell_values: Vec<i8> = self.all_nums_to_match.clone();

        for (ii, row) in self.board.clone().into_iter().enumerate() {
            if ii < last_modified_cell.row {
                continue;
            }

            for (jj, &_col) in row.iter().enumerate() {
                untried_cell_values = self.all_nums_to_match.clone();
                // let mut untried_cell_values: Vec<i8> = self.all_nums_to_match.clone();
                // .into_iter().filter(|x| !self.get_row(ii).contains(x) &&
                // !self.get_col(jj).contains(x) &&
                // !self.get_subsquare(&Cell { row: ii, col: jj }).contains(x)).collect();
                // .into_iter().filter(|x| !self.get_row(ii).contains(x)).collect();
                // println!("untried_cell_values: {:?}", &untried_cell_values);
                // println!("row {0}: {1:?}", ii, self.get_row(ii));
                // println!("col {0}: {1:?}", jj, self.get_col(jj));
                // println!("subsq {0}{1}: {2:?}", ii, jj, self.get_subsquare(&Cell { row: ii, col: jj }));

                while self[Cell{row: ii, col: jj}] == 0 {
                    // If there's no more valid numbers to try, backtrack and try previous cell again
                    if untried_cell_values.len() == 0 {
                        // println!("backtracking!");
                        self[last_modified_cell] = 0;
                        return;
                    }

                    // let temp = rng.gen_range(1, curr_board.size + 1);
                    let temp: i8 = match untried_cell_values.choose(&mut rng) {
                        Some(&x) => x,
                        None => break,
                    };

                    untried_cell_values.retain(|&x| x != temp);  // Remove `temp` from list
                    self[Cell{row: ii, col: jj}] = temp;

                    if self.check_intermediate_puzzle(Cell{row: ii, col: jj})
                    {
                        self.solve(Cell{row: ii, col: jj}, rng);
                    } else {
                        self[Cell{row: ii, col: jj}] = 0i8;
                    }
                }
            }
        }
        if self.check_complete_puzzle() {
            return
        }
    }

    fn check_intermediate_puzzle(&self, last_modified_cell: Cell) -> bool {
        // println!("Row: {:?}", curr_board.get_row((&last_modified_cell).row));
        // println!("Col: {:?}", curr_board.get_col((&last_modified_cell).col));
        // println!("SS: {:?}", curr_board.get_subsquare(&last_modified_cell));
        if !has_unique_elements(self.get_row((&last_modified_cell).row)) {
            return false;
        } else if !has_unique_elements(self.get_col((&last_modified_cell).col)) {
            return false;
        } else if !has_unique_elements(self.get_subsquare(&last_modified_cell)) {
            return false;
        }
        true
    }

    fn check_complete_puzzle(&self) -> bool {
        for row in self.board.iter() {
            if !self.all_nums_to_match.iter().all(|x| row.contains(x)) {
                return false;
            }
        }
        for col_idx in 0..self.size as usize {
            let col = self.borrow().get_col(col_idx);
            if !self.all_nums_to_match.iter().all(|x| col.contains(x)) {
                return false;
            }
        }
        for subsquare_row_idx in 0..self.subsquare_size {
            for subsquare_col_idx in 0..self.subsquare_size {
                let subsquare = self.get_subsquare(&Cell{row: subsquare_row_idx, col: subsquare_col_idx});
                if !self.all_nums_to_match.iter().all(|x| subsquare.contains(x)) {
                    return false;
                }
            }
        }

        return true
    }

    #[inline]
    fn get_row(&self, row: usize) -> Vec<i8> {
        self.board[row].clone().iter().map(|&x| x).filter(|&x| x != 0).into_iter().collect()
    }

    #[inline]
    fn get_col(&self, col: usize) -> Vec<i8> {
        self.board.clone().iter().map(|row| row[col]).filter(|&x| x != 0).collect()
    }

    fn get_subsquare(&self, cell: &Cell) -> Vec<i8> {
        let subsquare_row_idx = cell.row.rem(self.subsquare_size as usize);
        let subsq_row_start = subsquare_row_idx * self.subsquare_size;
        let subsq_row_end = subsq_row_start + self.subsquare_size;

        let subsquare_col_idx = cell.col.rem(self.subsquare_size as usize);
        let subsq_col_start = subsquare_col_idx * self.subsquare_size;
        let subsq_col_end = subsq_col_start + self.subsquare_size;

        let mut subsquare: Vec<i8> = Vec::new();
        for row in self.board[subsq_row_start..subsq_row_end].iter() {
            subsquare.extend_from_slice(&row[subsq_col_start..subsq_col_end]);
        }

        subsquare.clone().iter().filter(|&&x| x != 0).map(|x| *x).collect::<Vec<i8>>()
        // self.board.clone().iter().enumerate()
        //     .filter(|(idx, &row)| subsquare_row_idx <= *idx && *idx < subsq_row_end)
        //     .map(|(_, &row)| row[subsq_col_start..subsq_col_end])
        //     .collect()
    }
}

struct Cell {
    row: usize,
    col: usize,
}

impl Index<Cell> for Board {
    type Output = i8;
    fn index(&self, cell: Cell) -> &i8 {
        &self.board[cell.row][cell.col]
    }
}

impl IndexMut<Cell> for Board {
    fn index_mut(&mut self, cell: Cell) -> &mut i8 {
        &mut self.board[cell.row][cell.col]
    }
}

fn read_puzzle(file_name: String) -> Board {
    let mut reader = csv::Reader::from_path(&file_name).expect("poop0");
    let mut rows: Vec<Vec<i8>> = vec![];

    for row in reader.deserialize(){
        let values: Vec<i8> = row.expect("poop1");
        rows.push(values);
    }
    println!("Number of rows: {}", rows.len());

    Board {
        size: rows.len() as i8,
        subsquare_size: (rows.len() as f64).sqrt() as usize,
        all_nums_to_match: (1..rows.len() as i8 + 1).collect(),
        board: rows,
    }
}
