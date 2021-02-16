use std::fmt;

pub enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Piece::King => write!(f, "K"),
            Piece::Queen => write!(f, "Q"),
            Piece::Rook => write!(f, "R"),
            Piece::Knight => write!(f, "N"),
            Piece::Bishop => write!(f, "B"),
            Piece::Pawn => write!(f, "P"),
        }
    }
}

#[derive(Clone)]
struct Square {}

pub struct Board {
    pub squares: vec![vec![Square; 8]; 8],
}

impl Board {
    fn new() -> Self {
        let mut squares = vec![vec![Square {}; 8]; 8];
        for x in 0..squares.len() {
            for y in 0..squares[x].len() {
                squares[x][y] = Square {};
            }
        }
        Board { squares }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("{:?}", self.squares);
        Ok(())
    }
}
