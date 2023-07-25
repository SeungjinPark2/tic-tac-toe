mod game;

use game::{Point, CellState};
use std::io;

fn main() {
    let mut myTTT = game::create_tic_tac_toe();
    let mut toggle = CellState::O;

    loop {
        println!("{}", myTTT.get_board_summary());
        let mut line = String::from("");
        io::stdin().read_line(&mut line).unwrap();
        
        let point_as_vec = line.split(',').collect::<Vec<&str>>();
        let point = Point { x: point_as_vec[0].parse().unwrap(), y: point_as_vec[1].parse().unwrap() };

        // make_move 가 CellState 를 받아서 어쩔수 없이 우회하는데 Copy trait 을 구현하던, 참조를 받게 만들던 하는게 맞음.
        match toggle {
            CellState::O => myTTT.make_move(point, CellState::O).unwrap(),
            CellState::X => myTTT.make_move(point, CellState::X).unwrap(),
            _ => (),
        }

        match myTTT.check_board() {
            &CellState::O => {
                println!("winner is O");
                break;
            },
            &CellState::X => {
                println!("winner is X");
                break;
            },
            _ => ()
        }
    }
}
