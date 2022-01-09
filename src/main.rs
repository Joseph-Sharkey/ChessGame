// how are we going to write a convincing game of chess?s
use std::io::{self, BufRead};
use std::cmp::PartialEq;

fn main() {
    let mut board: Board = Board::new();
    let mut ended: bool = false;
    let mut winner: i32 = 0;
    while !(ended) {
        board.display();
        let move1: bool = board.movePiece(1);//we need to move the code above into the function to allow for error correction when the player value is invalid
        if move1 == true {
            winner = 1;
            break;
        }
        let move2: bool = board.movePiece(2);
        if move2 == true {
            winner = 2;
            break;
        }

    }
    if winner == 1 {
        println!("player 1 won the game");
    } else {
        println!("player 2 won the game");
    }
}

fn getInput(text: &str) -> String{
    println!("{}", text);
    let stdin = io::stdin();
    let line1: String = stdin.lock().lines().next().unwrap().unwrap();
    return line1;
}

enum ChessPiece {
    Pa,
    Ro,
    Kn,
    Bi,
    Ki,
    Qu,
    O
}

impl PartialEq for ChessPiece {
    fn eq(&self, &Rhs) -> bool {}
}

struct Piece{
    value: ChessPiece,
    owner: i32
}

impl Piece {
    fn new(owner: i32, pieceType: ChessPiece) -> Piece {
        return Piece {
            value: pieceType,
            owner: owner
        }
    }
    fn clone(&self) -> Piece{
        return Piece {
            value: self.value,
            owner: self.owner
        } 
    }
}

struct Board {
    boardData: Vec<Vec<Piece>>  
}

impl Board {
    fn new() -> Board {
        let p1Pawn= Piece::new(1, ChessPiece::Pa);
        let p1Rook = Piece::new(1, ChessPiece::Ro);
        let p1Knight = Piece::new(1, ChessPiece::Ki);
        let p1Bishop = Piece::new(1, ChessPiece::Bi);
        let p1Queen = Piece::new(1, ChessPiece::Qu);
        let p1King = Piece::new(1, ChessPiece::Ki);

        let p2Pawn= Piece::new(2, ChessPiece::Pa);
        let p2Rook = Piece::new(2, ChessPiece::Ro);
        let p2Knight = Piece::new(2, ChessPiece::Ki);
        let p2Bishop = Piece::new(2, ChessPiece::Bi);
        let p2Queen = Piece::new(2, ChessPiece::Qu);
        let p2King = Piece::new(1, ChessPiece::Ki);

        let empty = Piece::new(0, ChessPiece::O);

        let board: Vec<Vec<Piece>> = vec![
            vec![p1Rook.clone(), p1Knight.clone(), p1Bishop.clone(), p1Queen, p1King, p1Bishop.clone(), p1Knight.clone(), p1Rook.clone()],
            vec![p1Pawn.clone(), p1Pawn.clone(), p1Pawn.clone(), p1Pawn.clone(), p1Pawn.clone(), p1Pawn.clone(), p1Pawn.clone(), p1Pawn.clone()],
            vec![empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone()], 
            vec![empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone()],
            vec![empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone()],
            vec![empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone(), empty.clone()],
            vec![p2Pawn.clone(), p2Pawn.clone(), p2Pawn.clone(), p2Pawn.clone(), p2Pawn.clone(), p2Pawn.clone(), p2Pawn.clone(), p2Pawn.clone()],
            vec![p2Rook.clone(), p2Knight.clone(), p2Bishop.clone(), p2Queen.clone(), p2King.clone(), p2Bishop.clone(), p2Knight.clone(), p2Rook.clone()]
        ]; //not the prettiest solution 

        return Board {
            boardData: board
        }
    }

    fn display(&self) {
        for row in self.boardData {
            for x in row {
                let displayValue: &str = match x.value {
                    Pa => "Pa",
                    Ro => "Ro",
                    Kn => "Kn",
                    Bi => "Bi",
                    Qu => "Qu",
                    Ki => "Ki",
                    O => "0"
                };
                print!("{}", displayValue);
                print!(" ");
            }
            print!("\n");
        }
    }
    
    fn isCheckMate(&self) -> bool {
        //we need to establish whether the king is in check, and if he is capable of moving out of check
        // I haven't prevented players from moving into check, so I should probably implement that in a bit
        
        //a player is in check mate if king is in check, and all adjacent moves are either occupied or in check
        // we need to both check whether adjacent places are in check, and then whether the adjacent places are occupied
        // if there is an enemy piece next to the player and that piece is not protected, the king can take that piece
        // we need to also allow other pieces on the same team to block for the king
        //oof this will be hard
    }

    fn isValidMove(&self, direction: &str, distance: i32, player: i32, xMove: usize, yMove: usize, maxDistance: i32) -> bool {
        if self.boardData[yMove][xMove].owner != player {
            return false
        };
        if distance > maxDistance {
            return false
        };
        let newXCoord: usize = xMove;
        let newYCoord: usize = yMove;

       if self.boardData[yMove][xMove].value == ChessPiece::Kn {
            if player == 1 {
                match direction {
                    "forward right" => {newYCoord -= 2; newXCoord += 1},
                    "forward left" => {newYCoord -= 2; newXCoord -= 1},
                    "right forward" => {newYCoord -= 1; newXCoord += 2},
                    "right backwards" => {newYCoord += 1; newXCoord -= 2},
                    "left forwards" => {newYCoord -= 1; newXCoord -= 2},
                    "left backwards" => {newYCoord += 1; newXCoord += 2},
                    "backwards right" => {newYCoord += 2; newXCoord += 1},
                    "backwards left" => {newYCoord += 2; newXCoord -= 1}
                };
            } else {
                match direction {
                    "forward right" => {newYCoord += 2; newXCoord += 1},
                    "forward left" => {newYCoord += 2; newXCoord -= 1},
                    "right forward" => {newYCoord += 1; newXCoord += 2},
                    "right backwards" => {newYCoord -= 1; newXCoord -= 2},
                    "left forwards" => {newYCoord += 1; newXCoord -= 2},
                    "left backwards" => {newYCoord -= 1; newXCoord += 2},
                    "backwards right" => {newYCoord -= 2; newXCoord += 1},
                    "backwards left" => {newYCoord -= 2; newXCoord -= 1}
                };
                if newXCoord > 7 || newXCoord < 0 {
                    return false
                }
                if newYCoord > 7 || newYCoord < 0 {
                    return false
                }
                if self.boardData[newYCoord][newXCoord].owner == player {
                    return false;
                }
                return true;
            }
        }; //knight has different movement system to other pieces and therefore needs a separate condition

        if player == 1 { //player 1 has pieces at the top of the board, player 2 has pieces at the bottom of the board, and move in different directions
            //right and left remain constant irrespective of the player
            //also note that board y coordinates are inverted compared to cartesian. 0 is at the top
            match direction {
                "forward" => newYCoord += distance as usize,
                "right" => newXCoord += distance as usize,
                "left" => newXCoord -= distance as usize,
                "backwards" => newYCoord -= distance as usize,
                "forward right diagonal" => {newYCoord += distance as usize; newXCoord += distance as usize},
                "forward left diagonal" => {newYCoord += distance as usize; newXCoord -= distance as usize},
                "backwards right diagonal" => {newYCoord -= distance as usize; newXCoord += distance as usize},
                "backwards left diagonal" => {newYCoord -= distance as usize; newXCoord -= distance as usize} 
            };
        } else {
            match direction {
                "forward" => newYCoord -= distance as usize,
                "right" => newXCoord += distance as usize,
                "left" => newXCoord -= distance as usize,
                "backwards" => newYCoord += distance as usize,
                "forward right diagonal" => {newYCoord -= distance as usize; newXCoord += distance as usize},
                "forward left diagonal" => {newYCoord -= distance as usize; newXCoord += distance as usize},
                "backwards right diagonal" => {newYCoord += distance as usize; newXCoord += distance as usize},
                "backwards left diagonal" => {newYCoord += distance as usize; newXCoord -= distance as usize}
            };
        }
        if newXCoord > 7 || newXCoord < 0 {
            return false
        }
        if newYCoord > 7  || newXCoord < 0{
            return false
        };
        if self.boardData[newYCoord][newXCoord].owner == player {
            return false
        };

        if direction == "forward" || direction == "right" || direction == "left" || direction == "backwards" {
            if player == 1 {
                match direction {
                    "forward" => {
                        //this bit is awkward because if the new coord is smaller than the current coord, it will be impossible to loop through the values inbetween
                        }
                    },
                    "right" => {},
                    "left" => {},
                    "backwards" => {}
                } else {

                }
                
            } else {

            }
    }
    
        //piece must not go through any of the other pieces
        //this is relatively straight forward if the piece is going in a straight line, not so much diagonally 
    
    fn movePiece(&mut self, player: i32) -> bool{
        let yMove: usize = getInput("select square on y axis with piece that you want to move: 1-8: ").parse::<usize>().unwrap() - 1;
        let xMove: usize = getInput("select square on x axis with piece that you want to move: 1-8: ").parse::<usize>().unwrap() - 1;
        let piece: ChessPiece = self.boardData[yMove][xMove].value;
        while piece == ChessPiece::O {
            println!("empty space, please pick a new piece");
            yMove: usize = getInput("select square on y axis with piece that you want to move: 1-8: ").parse::<usize>().unwrap();
            xMove: usize = getInput("select square on x axis with piece that you want to move: 1-8: ").parse::<usize>().unwrap();
            piece: ChessPiece = self.boardData[yMove - 1][xMove - 1].value;
        }
        let distance = getInput("select the distance you would like to move your piece: ").parse::<i32>().unwrap();
        let possibleDirection: Vec<&str>= match piece {
            Pa => vec!["forward", "right", "left"],
            Ro => vec!["forward","right", "left", "backwards"],
            Kn => vec!["forward right", "forward left","right forward", "right backwards",  "left forward", "left backwards", "backwards right", "backwards left"], //specific conditions will be needed for the knight
            Bi => vec!["forward right diagonal", "forward left diagonal", "backward right diagonal", "backward left diagonal"],
            Ki => vec!["forward", "right", "left", "backwards"],
            Qu => vec!["forward", "right", "left", "backwards", "forward right diagonal", "forward left diagonal", "backward right diagonal", "backward left diagonal"]
        };
        let maxDistance: i32 = match piece {
            Pa => 1,
            Ro => 8,
            Kn => 3, //this requires specific conditions for movement
            Bi => 8,
            Ki => 1,
            Qu => 8 
        };
        println!("these are the possible directions: ");
        for x in possibleDirection {
            println!("{}", x)
        }
        let direction = getInput("select your orientation from the listed values: ");
        println!("the maximum distance the piece can move is: {}", maxDistance);
        println!("note that the knight must always move a distance of 3");
        let distance: i32 = getInput("select your distance: ").parse::<i32>().unwrap();
        while !(self.isValidMove(&direction, distance, player, xMove, yMove, maxDistance)) {
            direction = getInput("select your orientation from the listed values: ");
            println!("the maximum distance the piece can move is: {}", maxDistance);
            distance: i32 = getInput("select your distance: ").parse::<i32>().unwrap();
        }
        self.movePieceOnBoard(&direction, distance, xMove, yMove, player);
        //check if move is valid using try catch statements, then proceed with the move with the given parameters, and add specific programming for the knight
        //then figure out how tf to write the checkmate detector, and also how to prevent the king from moving into check. idk how that is going to work at the moment
        self.display();
        return self.isCheckMate();
    }
    fn movePieceOnBoard(&mut self, direction: &str, distance: i32, xMove: usize, yMove: usize, player: i32) {
        let newXCoord = xMove;
        let newYCoord = yMove;
        if player == 1 {
            match direction {
                "forward" => newYCoord += distance as usize,
                "right" => newXCoord += distance as usize,
                "left" => newXCoord -= distance as usize,
                "backwards" => newYCoord -= distance as usize,
                "forward right diagonal" => {newYCoord += distance as usize; newXCoord += distance as usize},
                "forward left diagonal" => {newYCoord += distance as usize; newXCoord -= distance as usize},
                "backwards right diagonal" => {newYCoord -= distance as usize; newXCoord += distance as usize},
                "backwards left diagonal" => {newYCoord -= distance as usize; newXCoord -= distance as usize} 
            }
        } else {
            match direction {
                "forward" => newYCoord -= distance as usize,
                "right" => newXCoord += distance as usize,
                "left" => newXCoord -= distance as usize,
                "backwards" => newYCoord += distance as usize,
                "forward right diagonal" => {newYCoord -= distance as usize; newXCoord += distance as usize},
                "forward left diagonal" => {newYCoord -= distance as usize; newXCoord += distance as usize},
                "backwards right diagonal" => {newYCoord += distance as usize; newXCoord += distance as usize},
                "backwards left diagonal" => {newYCoord += distance as usize; newXCoord -= distance as usize}
            }
       }
       if self.boardData[newYCoord][newXCoord].value != ChessPiece::O {
           if player == 1 {
               println!("player 2 has lost a piece");
           } else {
               println!("player 1 has lost a piece");
           }
       } else {
           println!("no pieces were lost");
       }
       self.boardData[newYCoord][newXCoord] = self.boardData[yMove][xMove].clone();
       self.boardData[yMove][xMove] = Piece {
           value: ChessPiece::O,
           owner: 0
       };
    }
}
