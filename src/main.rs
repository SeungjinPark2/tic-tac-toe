mod game;

use game::{Point, CellState};

fn main() {
    let mut myTTT = game::create_tic_tac_toe();

    println!("{}", myTTT.get_board_summary());

    myTTT.make_move(Point {x: 0, y: 0}, CellState::O).unwrap();

    println!("{}", myTTT.get_board_summary());
}
