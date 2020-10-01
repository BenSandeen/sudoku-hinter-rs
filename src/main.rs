use csv;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::borrow::{BorrowMut, Borrow};
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::io;
use std::ops::{Index, IndexMut, Rem};


fn main() {
    let mut board = read_puzzle(String::from("sample_puzzle.csv"));
    println!("{}", board);
    // println!("{}", &board[Cell{row: 0, col: 0}]);
    // println!("{}", &board[Cell{row: 4, col: 2}]);
    // println!("{}", &board[Cell{row: 1, col: 1}]);

    let mut rng = rand::thread_rng();

    board.solve(Cell{row: 0, col: 0}, rng);
    println!("{}", board);

    board = read_puzzle(String::from("9x9_tough.csv"));
    println!("{}", board);
    board.solve(Cell{row: 0, col: 0}, rng);
    println!("{}", board);

    // board = read_puzzle(String::from("16x16_sample_puzzle.csv"));
    // board.solve(Cell{row: 0, col: 0}, rng);
    // println!("{:?}", board);
    //
    // board = read_puzzle(String::from("16x16_another_puzzle.csv"));
    // board.solve(Cell{row: 0, col: 0}, rng);
    // println!("{:?}", board);
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
struct Choices {
    cell: Cell,
    choices: Vec<i8>
}

#[derive(Debug, Deserialize)]
struct Board {
    size: i8,
    subsquare_size: usize,
    all_nums_to_match: Vec<i8>,
    // all_nums_to_match: HashSet<i8>,
    available_choices: Vec<Choices>,
    board: Vec<Vec<i8>>,
}

impl Board{
    fn new(nums: Vec<Vec<i8>>) -> Board {
        Board {
            size: nums.len() as i8,
            subsquare_size: (nums.len() as f64).sqrt() as usize,
            all_nums_to_match: (1..nums.len() as i8 + 1).collect(),
            available_choices: {
                let mut default_choices: Vec<Choices> = Vec::new();
                for (ii, row) in nums.clone().into_iter().enumerate() {
                    for (jj, &_col) in row.iter().enumerate() {
                        default_choices.push(Choices {
                            cell: Cell { row: ii, col: jj },
                            choices: (1..nums.len() as i8 + 1).collect() });
                    }
                }

                default_choices
            },
            board: nums,
        }
    }

    // fn init_choices(board: Vec<Vec<i8>>) -> Vec<Choices> {
    //     let mut default_choices: Vec<Choices> = Vec::new();
    //     for (ii, row) in board.clone().into_iter().enumerate() {
    //         for (jj, &_col) in row.iter().enumerate() {
    //             default_choices.push(Choices {
    //                 cell: Cell { row: ii, col: jj },
    //                 choices: (1..nums.len() as i8 + 1).collect() });
    //         }
    //     }
    //
    //     default_choices
    // }

    // fn solve(&mut self, last_modified_cell: Cell, mut rng: ThreadRng) {
    //     let mut untried_cell_values: Vec<i8> = self.all_nums_to_match.clone();
    //
    //     for (ii, row) in self.board.clone().into_iter().enumerate() {
    //         if ii < last_modified_cell.row {
    //             continue;
    //         }
    //
    //         for (jj, &_col) in row.iter().enumerate() {
    //             untried_cell_values = self.all_nums_to_match.clone();
    //
    //             while self[Cell{row: ii, col: jj}] == 0 {
    //                 // If there's no more valid numbers to try, backtrack and try previous cell again
    //                 if untried_cell_values.len() == 0 {
    //                     self[last_modified_cell] = 0;
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
    //                 self[Cell{row: ii, col: jj}] = temp;
    //
    //                 if self.check_intermediate_puzzle(Cell{row: ii, col: jj})
    //                 {
    //                     self.solve(Cell{row: ii, col: jj}, rng);
    //                 } else {
    //                     self[Cell{row: ii, col: jj}] = 0i8;
    //                 }
    //             }
    //         }
    //     }
    //     if self.check_complete_puzzle() {
    //         return
    //     }
    // }

    fn solve(&mut self, last_modified_cell: Cell, mut rng: ThreadRng) {
        self.update_choices();
        // println!("board: {}", self);

        // for choices in &mut self.available_choices {
        for choices_idx in 0..self.available_choices.len() {
            if self.check_complete_puzzle() {
                return;
            }

            // println!("{:?}", self.available_choices[choices_idx]);
            // let mut guess = String::new();
            // io::stdin().read_line(&mut guess).expect("Failed to read line");

            let choices_thing = &self.available_choices[choices_idx];
            let cell = Cell{row: choices_thing.cell.row, col: choices_thing.cell.col};

            let mut valid_choices: Vec<i8> = choices_thing.choices.iter().map(|&x| x).collect();

            // println!("cell: {}", cell);
            // println!("choices: {:?}", valid_choices);


            while self.board[(&cell).row][(&cell).col] == 0 {
                // let mut valid_choices: Vec<i8> = choices.choices.clone();
                // let valid_choices: Vec<i8> = self.available_choices[choices_idx].choices;
                if valid_choices.len() == 0 {  // Backtrack if no valid numbers can be picked
                    self.board[cell.row][cell.col] = 0;
                    self.board[(&last_modified_cell).row][(&last_modified_cell).col] = 0;
                    self.update_choices();
                    return;
                }

                let temp: i8 = match &valid_choices.choose(&mut rng) {
                    Some(&x) => x,
                    None => break,
                };

                valid_choices.retain(|&x| x != temp);  // Remove `temp` from list
                self.board[(&cell).row][(&cell).col] = temp;
                // drop(choices_thing);
                &self.solve(Cell{row: (&cell).row, col: (&cell).col}, rng);
            }
        }
        if self.check_complete_puzzle() {
            return;
        }
    }

    fn update_choices(&mut self) {
        // println!("available_choices: {:?}", self.available_choices);

        self.available_choices.clear();
        // println!("available_choices: {:?}", self.available_choices);
        for (ii, row) in self.board.clone().into_iter().enumerate() {
            for (jj, &_col) in row.iter().enumerate() {
                let mut invalid_vals: Vec<i8> = self.get_row(ii);

                // if ii == 1 && jj == 5 {
                //     println!("invalid from row 1: {:?}", invalid_vals.iter());
                // }

                invalid_vals.append(&mut self.get_col(jj));

                // if ii == 1 && jj == 5 {
                //     println!("invalid from row 1 and col 5: {:?}", invalid_vals.iter());
                // }


                invalid_vals.append(&mut self.get_subsquare(&Cell{row: ii, col: jj}));

                // if ii == 1 && jj == 5 {
                //     println!("invalid from row 1 and col 5 and subsquare: {:?}", invalid_vals.iter());
                // }


                let valid_vals: Vec<i8> = self.all_nums_to_match.clone().into_iter()
                    .filter(|&x| !invalid_vals.contains(&x)).collect::<Vec<i8>>();

                self.available_choices.push(Choices{
                    cell: Cell{row:ii, col: jj},
                    choices: valid_vals.clone(),
                });

                // if ii == 1 && jj == 5 {
                //     println!("invalid choices for (1, 5): {:?}", invalid_vals.iter());
                //     println!("valid_choices for (1, 5): {:?}", valid_vals.iter());
                // }

            }
        }
        // println!("Before sorting!");
        // for choice in self.available_choices.iter() {
        //     println!("{:?}", choice);
        // }
        self.available_choices.sort_by(|a, b| a.choices.len().cmp(&b.choices.len()));

        // println!("After sorting!");
        // for choice in self.available_choices.iter() {
        //     println!("{:?}", choice);
        // }
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
        // Rust integer division also seems to perform an implicit `floor` operation, but this
        // explicit version makes things more obvious so that I won't forget that I need the floor
        let subsquare_row_idx = ((cell.row / self.subsquare_size) as f32).floor() as usize;
        let subsq_row_start = subsquare_row_idx * self.subsquare_size;
        let subsq_row_end = subsq_row_start + self.subsquare_size;

        let subsquare_col_idx = ((cell.col / self.subsquare_size) as f32).floor() as usize;
        let subsq_col_start = subsquare_col_idx * self.subsquare_size;
        let subsq_col_end = subsq_col_start + self.subsquare_size;

        let mut subsquare: Vec<i8> = Vec::new();
        // for row in self.board[subsq_row_start..subsq_row_end].iter() {
        for (ii, row) in self.board.iter().enumerate() {
            if !(subsq_row_start <= ii && ii < subsq_row_end) {
                continue
            }
            // if cell.row == 1 && cell.col == 5 {
            //     println!("Getting numbers from row: {:?}", row.iter());
            // }

            subsquare.extend_from_slice(&row[subsq_col_start..subsq_col_end]);
        }

        // if cell.row == 1 && cell.col == 5 {
        //     println!("Subsquare row start and end: {}-{} \t col start and end: {}-{}",
        //              subsq_row_start, subsq_row_end,
        //              subsq_col_start, subsq_col_end
        //     );
        //     println!("row_idx: {}, \t col_idx: {}", subsquare_row_idx, subsquare_col_idx);
        //     println!("(1, 5) subsquare: {:?}", subsquare.iter());
        // }

        subsquare.clone().iter().filter(|&&x| x != 0).map(|x| *x).collect::<Vec<i8>>()
    }
}

#[derive(Debug, Deserialize)]
struct Cell {
    row: usize,
    col: usize,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(row: {}, col: {})", self.row, self.col)
    }
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (ii, row) in self.board.iter().enumerate() {
            write!(f, "\n");
            for (jj, col) in row.iter().enumerate() {
                write!(f, " {} ", *col);
            }
        }
        write!(f, "\n")
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

    // Board {
    //     size: rows.len() as i8,
    //     subsquare_size: (rows.len() as f64).sqrt() as usize,
    //     all_nums_to_match: (1..rows.len() as i8 + 1).collect(),
    //     available_choices: Board::init_choices(),
    //     board: rows,
    // }
    Board::new(rows)
}
