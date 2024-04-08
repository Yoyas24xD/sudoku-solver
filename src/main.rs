use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Clone, Copy)]
struct Sudoku {
    columns: [[u8; 9]; 9],
    rows: [[u8; 9]; 9],
    squares: [[u8; 9]; 9],
}

fn load_sudoku<P>(path: P) -> Sudoku
where
    P: AsRef<Path>,
{
    // Initialize empty sudoku
    let mut sudoku = Sudoku {
        columns: [[0; 9]; 9],
        rows: [[0; 9]; 9],
        squares: [[0; 9]; 9],
    };

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (j, c) in line.chars().enumerate() {
            let n = c.to_digit(10).unwrap() as u8;
            sudoku.columns[j][i] = n;
            sudoku.rows[i][j] = n;
            let square = (i / 3) * 3 + j / 3;
            let index = (i % 3) * 3 + j % 3;
            sudoku.squares[square][index] = n;
        }
    }
    sudoku
}

// #[cfg(debug_assertions)]
fn print_sudoku(sudoku: &Sudoku) {
    for row in sudoku.rows.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn solve_sudoku(sudoku: &mut Sudoku) -> Option<Sudoku> {
    for row in 0..9 {
        for col in 0..9 {
            if sudoku.rows[row][col] == 0 {
                // Find possible values
                let mut possible = vec![];
                for i in 1..=9 {
                    if sudoku.rows[row].contains(&i) {
                        continue;
                    }
                    if sudoku.columns[col].contains(&i) {
                        continue;
                    }
                    let square_row = 3 * (row / 3);
                    // let square_col = 3 * (col / 3);
                    let square = &sudoku.squares[square_row + (col / 3)];
                    if square.contains(&i) {
                        continue;
                    }
                    possible.push(i);
                }
                // Try each possible value
                for &p in &possible {
                    // Try p
                    sudoku.rows[row][col] = p;
                    sudoku.columns[col][row] = p;
                    let square_row = 3 * (row / 3);
                    // let square_col = 3 * (col / 3);
                    sudoku.squares[square_row + (col / 3)][3 * (row % 3) + (col % 3)] = p;
                    if let Some(solved) = solve_sudoku(sudoku) {
                        return Some(solved);
                    }
                    // Undo the assignment if no solution found
                    sudoku.rows[row][col] = 0;
                    sudoku.columns[col][row] = 0;
                    sudoku.squares[square_row + (col / 3)][3 * (row % 3) + (col % 3)] = 0;
                }
                return None;
            }
        }
    }
    Some(*sudoku)
}

fn main() {
    let mut sudoku = load_sudoku("problem.txt");
    // print_sudoku(&sudoku);
    let solved = solve_sudoku(&mut sudoku);
    match solved {
        Some(solved) => {
            println!("Solved:");
            print_sudoku(&solved);
        }
        None => {
            println!("No solution found");
        }
    }
}
