use core::fmt;
#[derive(PartialEq)]
pub enum CellState {
    O,
    X,
    NONE,
}

// struct 으로 만들긴 했으나 굳이 그럴필요가 없는 단일 필드만 갖는 녀석이 되어 버렸음.
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
    pub fn check_board(&self) -> &CellState {
        // 일단 승리 조건을 거의 하드코딩에 가깝게 구현하자.
        // 목적은 게임을 만드는것.
        // 난 개 멍청한 것 같다 이렇게 하지 않으면 모르게따ㅏㅏㅏㅏ
        if self.board[0][0] == self.board[0][1]
            && self.board[0][1] == self.board[0][2] {
            &self.board[0][0]
        } else if self.board[1][0] == self.board[1][1]
            && self.board[1][1] == self.board[1][2] {
            &self.board[1][0]
        } else if self.board[2][0] == self.board[2][1]
            && self.board[2][1] == self.board[2][2] {
            &self.board[2][0]
        } else if self.board[0][0] == self.board[1][0]
            && self.board[1][0] == self.board[2][0] {
            &self.board[0][0]
        } else if self.board[0][1] == self.board[1][1]
            && self.board[1][1] == self.board[2][1] {
            &self.board[0][1]
        } else if self.board[0][2] == self.board[1][2]
            && self.board[1][2] == self.board[2][2] {
            &self.board[0][2]
        } else if self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2] {
            &self.board[0][0]
        } else if self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0] {
            &self.board[0][2]
        } else {
            &CellState::NONE
        }
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
