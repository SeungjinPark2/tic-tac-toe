use std::io;

struct tic_tac_toe {
    size: u8,
    board: Vec<Vec<&'static str>>,
}

impl tic_tac_toe {
    fn create_new_game(size: u8) -> tic_tac_toe {
        tic_tac_toe { size: size, board: vec![vec!["[]"; size as usize]; size as usize] }
    }
}

fn main() {
    println!("Hello, welcome to rust tic tac toe game.");
    print_board(&tic_tac_toe::create_new_game(3).board);
}

fn print_board(board: &Vec<Vec<&str>>) {
    for i in board {
        let line = i.join("");
        println!("{}", line);
    }
}

fn check_game(board: &Vec<Vec<&str>>) {
    // implement me   
}
