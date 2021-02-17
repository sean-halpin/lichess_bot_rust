use std::fmt;

fn algebraic_to_index(c: &char) -> usize {
    return match c {
        'a' | '1' => 0,
        'b' | '2' => 1,
        'c' | '3' => 2,
        'd' | '4' => 3,
        'e' | '5' => 4,
        'f' | '6' => 5,
        'g' | '7' => 6,
        'h' | '8' => 7,
        _ => usize::MAX,
    };
}

#[derive(Clone, Debug)]
pub enum Team {
    White,
    Black,
}

#[derive(Clone, Debug)]
pub enum Rank {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Clone, Debug)]
pub struct Piece {
    team: Team,
    rank: Rank,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.rank {
            Rank::King => write!(f, "K"),
            Rank::Queen => write!(f, "Q"),
            Rank::Rook => write!(f, "R"),
            Rank::Knight => write!(f, "N"),
            Rank::Bishop => write!(f, "B"),
            Rank::Pawn => write!(f, "P"),
        }
    }
}

impl Piece {
    fn new(team: Team, rank: Rank) -> Self {
        Piece { team, rank }
    }
}

#[derive(Clone, Debug)]
pub struct Square {
    pub piece: Option<Piece>,
}

impl Square {
    fn new() -> Self {
        Square { piece: None }
    }
    fn with_piece(piece: Piece) -> Self {
        Square { piece: Some(piece) }
    }
}

#[derive(Debug)]
pub struct Board {
    pub squares: Vec<Vec<Square>>,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = vec![vec![Square::new(); 8]; 8];
        for x in 0..squares.len() {
            for y in 0..squares[x].len() {
                match x {
                    0 => match y {
                        0 | 7 => {
                            squares[x][y] = Square::with_piece(Piece::new(Team::White, Rank::Rook))
                        }
                        1 | 6 => {
                            squares[x][y] =
                                Square::with_piece(Piece::new(Team::White, Rank::Knight))
                        }
                        2 | 5 => {
                            squares[x][y] =
                                Square::with_piece(Piece::new(Team::White, Rank::Bishop))
                        }
                        3 => {
                            squares[x][y] = Square::with_piece(Piece::new(Team::White, Rank::Queen))
                        }
                        4 => {
                            squares[x][y] = Square::with_piece(Piece::new(Team::White, Rank::King))
                        }
                        _ => (),
                    },
                    7 => match y {
                        0 | 7 => {
                            squares[x][y] = Square::with_piece(Piece::new(Team::Black, Rank::Rook))
                        }
                        1 | 6 => {
                            squares[x][y] =
                                Square::with_piece(Piece::new(Team::Black, Rank::Knight))
                        }
                        2 | 5 => {
                            squares[x][y] =
                                Square::with_piece(Piece::new(Team::Black, Rank::Bishop))
                        }
                        3 => {
                            squares[x][y] = Square::with_piece(Piece::new(Team::Black, Rank::Queen))
                        }
                        4 => {
                            squares[x][y] = Square::with_piece(Piece::new(Team::Black, Rank::King))
                        }
                        _ => (),
                    },
                    1 => squares[x][y] = Square::with_piece(Piece::new(Team::White, Rank::Pawn)),
                    6 => squares[x][y] = Square::with_piece(Piece::new(Team::Black, Rank::Pawn)),
                    _ => squares[x][y] = Square::new(),
                }
            }
        }
        Board { squares }
    }

    pub fn move_piece(&mut self, next_move: String) {
        let char_vec: Vec<char> = next_move.chars().collect();
        let from_col = algebraic_to_index(&char_vec[0]);
        let from_row = algebraic_to_index(&char_vec[1]);
        let to_col = algebraic_to_index(&char_vec[2]);
        let to_row = algebraic_to_index(&char_vec[3]);
        match &self.squares[from_row][from_col].piece {
            Some(p) => {
                self.squares[to_row][to_col].piece = Some(p.clone());
                self.squares[from_row][from_col].piece = None;
            }
            None => (),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        println!("###############");
        for x in (0..self.squares.len()).rev() {
            for y in 0..self.squares[x].len() {
                match &self.squares[x][y].piece {
                    Some(piece) => print!("{} ", piece),
                    None => print!(". "),
                }
            }
            println!();
        }
        println!("###############");
        Ok(())
    }
}
