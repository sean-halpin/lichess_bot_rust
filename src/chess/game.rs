use colored::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
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

impl Piece {
    fn new(team: Team, rank: Rank) -> Self {
        Piece { team, rank }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.team {
            Team::White => match self.rank {
                Rank::King => write!(f, "{}", "K".green()),
                Rank::Queen => write!(f, "{}", "Q".green()),
                Rank::Rook => write!(f, "{}", "R".green()),
                Rank::Knight => write!(f, "{}", "N".green()),
                Rank::Bishop => write!(f, "{}", "B".green()),
                Rank::Pawn => write!(f, "{}", "P".green()),
            },
            Team::Black => match self.rank {
                Rank::King => write!(f, "{}", "K".red()),
                Rank::Queen => write!(f, "{}", "Q".red()),
                Rank::Rook => write!(f, "{}", "R".red()),
                Rank::Knight => write!(f, "{}", "N".red()),
                Rank::Bishop => write!(f, "{}", "B".red()),
                Rank::Pawn => write!(f, "{}", "P".red()),
            },
        }
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
    fn index_to_column(c: usize) -> char {
        return match c {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => ' ',
        };
    }
    fn index_to_row(r: usize) -> char {
        return match r {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            6 => '7',
            7 => '8',
            _ => ' ',
        };
    }
    fn coords_to_str(
        from_column: usize,
        from_row: usize,
        to_column: usize,
        to_row: usize,
    ) -> String {
        let mut result = String::new();
        result.push(Square::index_to_column(from_column));
        result.push(Square::index_to_row(from_row));
        result.push(Square::index_to_column(to_column));
        result.push(Square::index_to_row(to_row));
        return result;
    }
    fn str_to_coords(algebraic: String) -> (usize, usize, usize, usize) {
        let index_vec: Vec<usize> = algebraic
            .chars()
            .map(|c| Square::algebraic_to_index(&c))
            .collect();
        return (index_vec[0], index_vec[1], index_vec[2], index_vec[3]);
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub next_to_move: Team,
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
        Board {
            next_to_move: Team::White,
            squares,
        }
    }

    pub fn move_piece(&mut self, next_move: String) {
        let (from_col, from_row, to_col, to_row) = Square::str_to_coords(next_move);
        match &self.squares[from_row][from_col].piece {
            Some(p) => {
                self.squares[to_row][to_col].piece = Some(p.clone());
                self.squares[from_row][from_col].piece = None;
            }
            None => (),
        }
        match &self.next_to_move {
            Team::White => self.next_to_move = Team::Black,
            Team::Black => self.next_to_move = Team::White,
        }
    }

    pub fn find_next_move(&mut self) -> String {
        for row in (0..self.squares.len()).rev() {
            for column in 0..self.squares[row].len() {
                match &self.squares[row][column].piece {
                    Some(piece) => {
                        if piece.team == self.next_to_move {
                            match piece.rank {
                                Rank::Pawn => match piece.team {
                                    Team::White => {
                                        return Square::coords_to_str(column, row, column, row + 1);
                                    }
                                    Team::Black => {
                                        return Square::coords_to_str(column, row, column, row - 1);
                                    }
                                },
                                Rank::King => {}
                                Rank::Queen => {}
                                Rank::Rook => {}
                                Rank::Knight => {}
                                Rank::Bishop => {}
                            }
                        }
                    }
                    None => (),
                }
            }
        }
        return "".to_string();
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

#[cfg(test)]
mod tests {
    use crate::chess::Board;

    #[test]
    fn create_chess_board() {
        let mut board = Board::new();
        println!("{}", board);
        board.move_piece("e2e4".to_string());
        println!("{}", board);
        let next_move = board.find_next_move();
        println!("{}", next_move);
        board.move_piece(next_move);
        println!("{}", board);
    }
}
