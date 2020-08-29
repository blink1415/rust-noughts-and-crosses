use std::io;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Symbol {
    None,
    X,
    O,
}

impl Symbol {
    fn to_string(&self) -> String {
        match self {
            Symbol::None => String::from(""),
            Symbol::X => String::from("X"),
            Symbol::O => String::from("O"),
        }
    }
}

struct Game {
    board: [[Symbol; 3]; 3],
    winner: Symbol,
}

impl Game {
    // Only needs to check if last move was a winning move
    fn check_for_win(&self, last_x: usize, last_y: usize) -> bool {
        // Used to check if anyone has won on the diagonals
        let mut diag = false;

        // Diagonal 1 has coordinates with equal x and y values
        // Diagonal 2 has coordinates with x and y values adding up to side length - 1

        if last_x == last_y {
            diag = diag
                || (self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]);
        } else if last_x + last_y == 2 {
            diag = diag
                || (self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0]);
        }

        // Checks column, row, and diagonal(s)
        (self.board[last_x][0] == self.board[last_x][1]
            && self.board[last_x][1] == self.board[last_x][2])
            || (self.board[0][last_y] == self.board[1][last_y]
                && self.board[1][last_y] == self.board[2][last_y])
            || diag
    }

    fn check_for_draw(&self) -> bool{
        for x in 0..3 {
            for y in 0..3 {
                if self.board[x][y] == Symbol::None {
                    return false;
                }
            }
        }
        return true;
    }

    // Checks if the move aims at an empty square
    fn is_legal_move(&self, last_x: usize, last_y: usize) -> bool {
        if self.board[last_x][last_y] == Symbol::None {
            true
        } else {
            false
        }
    }

    // Prints the board in a 3x3 square
    // Example: X_O
    //          O_X
    //          OXO
    fn display(&self) {
        let mut output = String::from(" 123\n");
        for x in 0..3 {
            output.push_str(&(x + 1).to_string());
            for y in 0..3 {
                match self.board[x][y] {
                    Symbol::None => output.push_str("_"),
                    Symbol::X => output.push_str("X"),
                    Symbol::O => output.push_str("O"),
                }
            }
            output.push_str("\n");
        }
        println!("{}", output);
    }

    fn make_move(&mut self, x: usize, y: usize, sym: Symbol) {
        self.board[x][y] = sym;
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}

fn main() {
    let mut game = Game {
        board: [[Symbol::None; 3]; 3],
        winner: Symbol::None,
    };

    // Tracks who's turn it is
    let mut current_player = Symbol::O;

    clear();

    while game.winner == Symbol::None {
        // Draws game board to terminal
        game.display();

        // Announces which player's turn it is
        match current_player {
            Symbol::O => println!("Current move: O"),
            Symbol::X => println!("Current move: X"),
            Symbol::None => panic!(
                "Current player has been set to empty, but the game is attempting to make a move."
            ),
        }

        // Gets input from user
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.split_whitespace();
        let vec = input.collect::<Vec<&str>>();

        // Only accepted inputs is two integers
        if vec.len() == 2 {
            // print!("{}[2J", 27 as char);
            let x: usize;
            let y: usize;

            match vec[0].parse::<usize>() {
                Ok(n) => y = n,
                Err(_) => {
                    clear();
                    println!("You entered an invalid input. Please write your input as \"x y\".",);
                    continue;
                }
            }

            match vec[1].parse::<usize>() {
                Ok(n) => x = n,
                Err(_) => {
                    clear();
                    println!(
                        "Error: You entered an invalid input. Please write your input as \"x y\".",
                    );
                    continue;
                }
            }

            if x == 0 || x > 4 || y == 0 || y > 4 {
                clear();
                println!("Error: Coordinates can only be between 0 and 4.");
                continue;
            }

            let x: usize = x - 1;
            let y: usize = y - 1;

            if game.is_legal_move(x, y) {
                game.make_move(x, y, current_player);
                if game.check_for_win(x, y) {
                    game.winner = current_player;
                    clear();
                    game.display();
                    println!("{} has won", current_player.to_string());
                    break;
                }
                else if game.check_for_draw() {
                    clear();
                    game.display();
                    println!("Game has ended in a draw.");
                    break;
                }
                match current_player {
                    Symbol::O => current_player = Symbol::X,
                    Symbol::X => current_player = Symbol::O,
                    Symbol::None => panic!("Current player has been set to empty, but the game is attempting to make a move.")
                }
            } else {
                clear();
                println!("Error: Not a legal move ({}, {})", x, y);
                continue
            }
            clear();
        } else {
            clear();
            println!("You entered an invalid input. Please write your input as \"x y\".",);
            continue;
        }
    }
}
