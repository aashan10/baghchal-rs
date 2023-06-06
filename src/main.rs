use std::io;

const TIGER: i32 = 1;
const GOAT: i32 = 2;
const EMPTY: i32 = 0;

struct BaghChal {
    total_remaining_goats: i32,
    killed_goats: i32,
    board: [[i32; 5]; 5],
    turn: i32,
}

impl BaghChal {
    fn new() -> BaghChal {
        BaghChal {
            total_remaining_goats: 20,
            killed_goats: 0,
            board: [[EMPTY; 5]; 5],
            turn: TIGER,
        }
    }

    fn init_board(&mut self) {
        for i in 0..5 {
            for j in 0..5 {
                self.board[i][j] = EMPTY;
            }
        }
        self.board[0][0] = TIGER;
        self.board[0][4] = TIGER;
        self.board[4][0] = TIGER;
        self.board[4][4] = TIGER;
    }

    fn switch_turn(&mut self) {
        if self.turn == TIGER {
            self.turn = GOAT;
        } else {
            self.turn = TIGER;
        }
    }

    fn player_info(&self) {
        println!("Goats killed: {}", self.killed_goats);
        if self.turn == TIGER {
            println!("\nTigers turn:\n");
        } else {
            println!("\nGoats turn:\n");
            println!("\nREMAINING GOATS: {}", self.total_remaining_goats);
        }

        self.print_board();
    }

    fn move_piece(&mut self, row: usize, col: usize, target_row: usize, target_col: usize) {
        if self.board[row][col] != self.turn {
            println!("The selected coordinate doesn't have a piece of the current player!");
            self.make_move();
            return;
        }

        let available_moves = self.available_moves(row, col);
        if !available_moves.contains(&[target_row, target_col]) {
            println!("Invalid Move!!");
            self.make_move();
            return;
        }

        self.board[target_row][target_col] = self.turn;
        self.board[row][col] = EMPTY;

        if self.turn == TIGER {
            if let Some((killed_row, killed_col)) = self.calculate_killed_goat(row, col, target_row, target_col) {
                self.board[killed_row][killed_col] = EMPTY;
                self.killed_goats += 1;
            }
        } else {
            self.total_remaining_goats -= 1;
        }
    }

    fn calculate_killed_goat(&self, row: usize, col: usize, target_row: usize, target_col: usize) -> Option<(usize, usize)> {
        let killed_row = (row as i32 + target_row as i32) / 2;
        let killed_col = (col as i32 + target_col as i32) / 2;
        if self.board[killed_row as usize][killed_col as usize] == GOAT {
            Some((killed_row as usize, killed_col as usize))
        } else {
            None
        }
    }

    fn available_moves(&self, row: usize, col: usize) -> Vec<[usize; 2]> {
        let mut moves = Vec::new();

        if self.turn == TIGER {
            let offsets = vec![
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1)
            ];
            for &(dx, dy) in offsets.iter() {
                let new_row = (row as i32 + dx) as usize;
                let new_col = (col as i32 + dy) as usize;
                if new_row < 5 && new_col < 5 && self.board[new_row][new_col] == EMPTY {
                    moves.push([new_row, new_col]);
                }
            }
        } else {
            let offsets = vec![
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
            ];

            for &(dx, dy) in offsets.iter() {
                let new_row = (row as i32 + dx) as usize;
                let new_col = (col as i32 + dy) as usize;
                if new_row < 5 && new_col < 5 && self.board[new_row][new_col] == EMPTY {
                    moves.push([new_row, new_col]);
                }
            }
        }

        moves
    }

    fn is_game_over(&self) -> bool {
        if self.total_remaining_goats == 0 || self.killed_goats >= 5 {
            return true;
        }

        let mut tiger_count = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == TIGER {
                    tiger_count += 1;
                }
            }
        }

        if tiger_count < 2 {
            return true;
        }

        false
    }

    fn make_move(&mut self) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if coords.len() != 4 {
            println!("Invalid input! Please enter four space-separated numbers.");
            self.make_move();
            return;
        }

        let row = coords[0];
        let col = coords[1];
        let target_row = coords[2];
        let target_col = coords[3];

        self.move_piece(row, col, target_row, target_col);

        self.switch_turn();

        if self.is_game_over() {
            println!("Game over!");
            return;
        }

        self.player_info();
        self.make_move();
    }

    fn print_board(&self) {
        println!("====================");
        for i in 0..5 {
            println!("\n|\t|\t|\t|\t|");
            for j in 0..5 {
                match self.board[i][j] {
                    TIGER => print!("T\t"),
                    GOAT => print!("G\t"),
                    EMPTY => print!("-\t"),
                    _ => unreachable!(),
                }
            }
            println!("\n|\t|\t|\t|\t|");
        }
        println!("====================");
    }
}

fn main() {
    let mut game = BaghChal::new();
    game.init_board();
    game.player_info();
    game.make_move();
}