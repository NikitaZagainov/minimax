#[derive(Eq, PartialEq, Hash, Clone)]
pub struct TicTacToe {
    board: Vec<Vec<char>>,
    board_size: usize,
    player_mark: char,
    bot_mark: char,
    fill: char,
    winning_len: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Mark {
    Player,
    Bot
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Action {
    x: usize,
    y: usize,
    mark: Mark,
}

impl TicTacToe {
    pub fn new(
        size: usize,
        player_mark: char,
        bot_mark: char,
        fill: char,
        winning_len: usize,
    ) -> Self {
        TicTacToe {
            board: vec![vec![fill; size]; size],
            board_size: size,
            player_mark,
            bot_mark,
            fill,
            winning_len,
        }
    }

    pub fn eq(&self, other: &TicTacToe) -> bool {
        return self.board == other.board();
    }

    pub fn clone(&self) -> Self {
        return TicTacToe {
            board: self.board.clone(),
            board_size: self.board_size,
            player_mark: self.player_mark,
            bot_mark: self.bot_mark,
            fill: self.fill,
            winning_len: self.winning_len,
        };
    }

    pub fn board(&self) -> Vec<Vec<char>> {
        self.board.clone()
    }

    pub fn print_board(&self) {
        for row in &self.board {
            for &cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }

    fn check_horizontal(&self, x: usize, y: usize, len: usize) -> Option<Mark> {
        let mut player_line = true;
        let mut bot_line = true;
        for i in 0..len {
            if self.board[x][y + i] != self.player_mark {
                player_line = false;
            }
            if self.board[x][y + i] != self.bot_mark {
                bot_line = false;
            }
        }
        if bot_line {
            return Option::Some(Mark::Bot);
        }
        if player_line {
            return Option::Some(Mark::Player);
        }
        return Option::None;
    }

    fn check_vertical(&self, x: usize, y: usize, len: usize) -> Option<Mark> {
        let mut player_line = true;
        let mut bot_line = true;
        for i in 0..len {
            if self.board[x + i][y] != self.player_mark {
                player_line = false;
            }
            if self.board[x + i][y] != self.bot_mark {
                bot_line = false;
            }
        }
        if bot_line {
            return Option::Some(Mark::Bot);
        }
        if player_line {
            return Option::Some(Mark::Player);
        }
        return Option::None;
    }

    fn check_diagonal(&self, x: usize, y: usize, len: usize) -> Option<Mark> {
        let mut player_line = true;
        let mut bot_line = true;
        for i in 0..len {
            if self.board[x + i][y + i] != self.player_mark {
                player_line = false;
            }
            if self.board[x + i][y + i] != self.bot_mark {
                bot_line = false;
            }
        }
        if bot_line {
            return Option::Some(Mark::Bot);
        }
        if player_line {
            return Option::Some(Mark::Player);
        }
        return Option::None;
    }

    fn check_diagonal_inv(&self, x: usize, y: usize, len: usize) -> Option<Mark> {
        let mut player_line = true;
        let mut bot_line = true;
        for i in 0..len {
            if self.board[x + i][y + len - i - 1] != self.player_mark {
                player_line = false;
            }
            if self.board[x + i][y + len - i - 1] != self.bot_mark {
                bot_line = false;
            }
        }
        if bot_line {
            return Option::Some(Mark::Bot);
        }
        if player_line {
            return Option::Some(Mark::Player);
        }
        return Option::None;
    }

    pub fn make_move(&mut self, action: &Action) -> Result<(), &'static str> {
        if action.x >= self.board_size || action.y > self.board_size {
            return Result::Err("Coordinates are out of bounds");
        }
        if self.board[action.x][action.y] == self.fill {
            match action.mark {
                Mark::Player => self.board[action.x][action.y] = self.player_mark,
                Mark::Bot => self.board[action.x][action.y] = self.bot_mark,
            }
            return Result::Ok(());
        } else {
            return Result::Err("This cell is already occupied");
        }
    }

    pub fn assess_position(&self, len: usize) -> (i32, i32) {
        let mut result = (0, 0);
        for x in 0..self.board_size {
            for y in 0..self.board_size {
                if x <= self.board_size - len {
                    match self.check_vertical(x, y, len) {
                        Option::Some(mark) => match mark {
                            Mark::Player => result.0 += 1,
                            Mark::Bot => result.1 += 1,
                        },
                        Option::None => (),
                    }
                }

                if y <= self.board_size - len {
                    match self.check_horizontal(x, y, len) {
                        Option::Some(mark) => match mark {
                            Mark::Player => result.0 += 1,
                            Mark::Bot => result.1 += 1,
                        },
                        Option::None => (),
                    }
                }

                if x <= self.board_size - len && y <= self.board_size - len {
                    match self.check_diagonal(x, y, len) {
                        Option::Some(mark) => match mark {
                            Mark::Player => result.0 += 1,
                            Mark::Bot => result.1 += 1,
                        },
                        Option::None => (),
                    }
                }

                if x <= self.board_size - len && y <= self.board_size - len {
                    match self.check_diagonal_inv(x, y, len) {
                        Option::Some(mark) => match mark {
                            Mark::Player => result.0 += 1,
                            Mark::Bot => result.1 += 1,
                        },
                        Option::None => (),
                    }
                }
            }
        }

        return result;
    }

    pub fn is_terminal(&self) -> bool {
        let score = self.assess_position(self.winning_len);
        if score == (0, 0) {
            for x in 0..self.board_size {
                for y in 0..self.board_size {
                    if self.board[x][y] == self.fill {
                        return false;
                    }
                }
            }
            return true;
        } else {
            return true;
        }
    }

    pub fn get_winner(&self) -> Option<Mark> {
        let score = self.assess_position(self.winning_len);
        if score.0 > score.1 {
            return Option::Some(Mark::Player);
        } else if score.1 > score.0 {
            return Option::Some(Mark::Bot);
        } else {
            return Option::None;
        }
    }

    pub fn get_legal_moves(&self, mark: &Mark) -> Vec<Action> {
        let mut result: Vec<Action> = Vec::new();
        for x in 0..self.board_size {
            for y in 0..self.board_size {
                if self.board[x][y] == self.fill {
                    result.push(Action::new(x, y, mark.clone()));
                }
            }
        }
        return result;
    }

    pub fn winning_len(&self) -> usize {
        self.winning_len
    }

}

impl Action {
    pub fn new(x: usize, y: usize, mark: Mark) -> Self {
        Action {
            x: x,
            y: y,
            mark: mark,
        }
    }
}

impl Mark {
    pub fn clone(&self) -> Self {
        match *self {
            Mark::Bot => Mark::Bot,
            Mark::Player => Mark::Player
        }
    }
}