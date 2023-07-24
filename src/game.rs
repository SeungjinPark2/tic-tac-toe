use core::fmt;

pub enum CellState {
    O,
    X,
    NONE,
}

pub struct tic_tac_toe {
    pub board: Vec<Vec<CellState>>, 
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl tic_tac_toe {
    pub fn get_board_summary(&self) -> String {
        let row_iter = self.board.iter();

        row_iter.map(|column| {
            column.iter()
                .map(|c| c.to_string())
                .reduce(|acc, a| {
                   format!("{}{}", acc, a)
                }).unwrap()
        }).reduce(|acc, a| {
            format!("{}\n{}", acc, a)
        }).unwrap()
    }

    pub fn make_move(&mut self, point: Point, cell: CellState) -> Result<(), String> {
        let row = match self.board.get_mut(point.y) {
            None => return Err(format!("point y={} is invalid", point.x).to_string()),
            Some(row) => row,
        };

        match row.get(point.x) {
            None => Err(format!("point x={} is invalid", point.x).to_string()),
            Some(_) => {
                row[point.x] = cell;
                Ok(())
            }
        }
    }

    // 일시적으로 CellState 으로 Winner 를 구분하도록 한다.
    // 추후에 Player struct 를 만들어서 구분하자.
    fn check_board(&self) -> CellState{
        // 일단 승리 조건을 거의 하드코딩에 가깝게 구현하자.
        // 목적은 게임을 만드는것.

        CellState::NONE
    }
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell_string = match self {
            CellState::NONE => "[ ]",
            CellState::O => "[O]",
            CellState::X => "[X]" 
        };
        write!(f, "{}", cell_string)
    }
}

pub fn create_tic_tac_toe() -> tic_tac_toe {
    tic_tac_toe { board: vec![
        vec![CellState::NONE, CellState::NONE, CellState::NONE],
        vec![CellState::NONE, CellState::NONE, CellState::NONE],
        vec![CellState::NONE, CellState::NONE, CellState::NONE]
    ] }
}
