const BOARD_WIDTH : usize = 8usize;
const BOARD_HEIGHT: usize  = 8usize;


enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

enum Player {
    Black,
    White
}

//Position of the piece on the board is (i,j) tuple where i and j vary between 0-7
struct Position(u8, u8);

struct Piece {
    owner: Player,
    kind: PieceType
}

struct GameBoard {
    pieces: [[Option<Piece>; BOARD_WIDTH]; BOARD_HEIGHT],
    playerTurn: Player,
}

impl GameBoard {
    fn new () -> Self {
        let pieces = [['R','N','B','K','Q','B','N','R'],
                  ['P','P','P','P','P','P','P','P'],
                  [' ',' ',' ',' ',' ',' ',' ',' '],
                  [' ',' ',' ',' ',' ',' ',' ',' '],
                  [' ',' ',' ',' ',' ',' ',' ',' '],
                  [' ',' ',' ',' ',' ',' ',' ',' '],
                  ['p','p','p','p','p','p','p','p'],
                  ['r','n','b','k','q','b','n','r']]
            .map(|row| {
                row.map(|chr| {
                    Some(match chr {
                        'R' => Piece { kind: PieceType::Rook, owner: Player::Black},
                        'N' => Piece { kind: PieceType::Knight, owner: Player::Black},
                        'B' => Piece { kind: PieceType::Bishop, owner: Player::Black},
                        'K' => Piece { kind: PieceType::King, owner: Player::Black},
                        'Q' => Piece { kind: PieceType::Queen, owner: Player::Black},
                        'P' => Piece { kind: PieceType::Pawn, owner: Player::Black},
                        'r' => Piece { kind: PieceType::Rook, owner: Player::White},
                        'n' => Piece { kind: PieceType::Knight, owner: Player::White},
                        'b' => Piece { kind: PieceType::Bishop, owner: Player::White},
                        'k' => Piece { kind: PieceType::King, owner: Player::White},
                        'q' => Piece { kind: PieceType::Queen, owner: Player::White},
                        'p' => Piece { kind: PieceType::Pawn, owner: Player::White},
                        _ => panic!("invalid initial board setup")
                    })
                })
            });

        GameBoard{ pieces, playerTurn: Player::White }
    }
}

                
                


    


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
