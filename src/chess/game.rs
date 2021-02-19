use colored::*;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Team {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rank {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub row: isize,
    pub column: isize,
    pub valid_location: bool,
}

impl Location {
    fn new(row: isize, column: isize) -> Self {
        if (row >= 0) && (row <= 7) && (column >= 0) && (column <= 7) {
            return Location {
                row,
                column,
                valid_location: true,
            };
        } else {
            return Location {
                row,
                column,
                valid_location: false,
            };
        }
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
            _ => 99,
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
            _ => '9',
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
            _ => '9',
        };
    }
    fn coords_to_str(
        from_column: usize,
        from_row: usize,
        to_column: usize,
        to_row: usize,
    ) -> String {
        let mut result = String::new();
        result.push(Location::index_to_column(from_column));
        result.push(Location::index_to_row(from_row));
        result.push(Location::index_to_column(to_column));
        result.push(Location::index_to_row(to_row));
        return result;
    }
    fn str_to_coords(algebraic: String) -> (usize, usize, usize, usize) {
        let index_vec: Vec<usize> = algebraic
            .chars()
            .map(|c| Location::algebraic_to_index(&c))
            .collect();
        return (index_vec[0], index_vec[1], index_vec[2], index_vec[3]);
    }
}

#[derive(Clone, Debug)]
pub struct Move {
    pub from: Location,
    pub to: Location,
    pub captured: Option<Rank>,
}

impl Move {
    fn new(_board: &Board, from: Location, to: Location, captured: Option<Rank>) -> Option<Self> {
        if from.valid_location && to.valid_location {
            return Some(Move { from, to, captured });
        }
        return None;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Square {
    pub location: Location,
    pub piece: Option<Piece>,
}

impl Square {
    fn new(piece: Option<Piece>, row: usize, column: usize) -> Self {
        Square {
            location: Location::new(row as isize, column as isize),
            piece,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub next_to_move: Team,
    pub squares: Vec<Vec<Square>>,
}

#[derive(Clone, Debug)]
pub enum Direction {
    N,
    S,
    E,
    W,
    NW,
    NE,
    SW,
    SE,
    KNIGHT,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = vec![vec![Square::new(None, 0, 0); 8]; 8];
        for x in 0..squares.len() {
            for y in 0..squares[x].len() {
                match x {
                    0 => match y {
                        0 | 7 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::White, Rank::Rook)), x, y)
                        }
                        1 | 6 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::White, Rank::Knight)), x, y)
                        }
                        2 | 5 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::White, Rank::Bishop)), x, y)
                        }
                        3 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::White, Rank::Queen)), x, y)
                        }
                        4 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::White, Rank::King)), x, y)
                        }
                        _ => (),
                    },
                    7 => match y {
                        0 | 7 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::Black, Rank::Rook)), x, y)
                        }
                        1 | 6 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::Black, Rank::Knight)), x, y)
                        }
                        2 | 5 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::Black, Rank::Bishop)), x, y)
                        }
                        3 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::Black, Rank::Queen)), x, y)
                        }
                        4 => {
                            squares[x][y] =
                                Square::new(Some(Piece::new(Team::Black, Rank::King)), x, y)
                        }
                        _ => (),
                    },
                    1 => {
                        squares[x][y] = Square::new(Some(Piece::new(Team::White, Rank::Pawn)), x, y)
                    }
                    6 => {
                        squares[x][y] = Square::new(Some(Piece::new(Team::Black, Rank::Pawn)), x, y)
                    }
                    _ => squares[x][y] = Square::new(None, x, y),
                }
            }
        }
        Board {
            next_to_move: Team::White,
            squares,
        }
    }

    pub fn move_piece(b: Board, next_move: String) -> Board {
        let mut cloned_board = b.clone();
        let (from_col, from_row, to_col, to_row) = Location::str_to_coords(next_move);
        match &b.squares[from_row][from_col].piece {
            Some(p) => {
                cloned_board.squares[to_row][to_col].piece = Some(p.clone());
                cloned_board.squares[from_row][from_col].piece = None;
            }
            None => (),
        }
        match &b.next_to_move {
            Team::White => cloned_board.next_to_move = Team::Black,
            Team::Black => cloned_board.next_to_move = Team::White,
        }
        return cloned_board;
    }

    fn is_king_checked(b: &Board, m: &Move) -> bool {
        for r in 0..8 {
            for c in 0..8 {
                let sqr = &b.squares[r][c];
                if sqr.piece.is_some() {
                    let p = sqr.piece.as_ref().unwrap();
                    if (p.rank == Rank::King) && (p.team == b.next_to_move) {
                        let move_string = Location::coords_to_str(
                            m.from.column as usize,
                            m.from.row as usize,
                            m.to.column as usize,
                            m.to.row as usize,
                        );
                        let future_board = Board::move_piece(b.clone(), move_string);
                        let next_moves = Board::find_next_moves(future_board);
                        return next_moves
                            .into_iter()
                            .filter(|m| m.captured.is_some())
                            .map(|m| m.captured.unwrap())
                            .filter(|c| c == &Rank::King)
                            .collect::<Vec<Rank>>()
                            .len()
                            > 0;
                    }
                }
            }
        }
        return false;
    }

    pub fn navigate(&self, dir: Direction, from: Location) -> Vec<Option<Move>> {
        let mut moves: Vec<Option<Move>> = vec![];
        let row = from.row;
        let column = from.column;
        let p = &self.squares[row as usize][column as usize]
            .piece
            .as_ref()
            .unwrap();
        let max_distance = match p.rank {
            Rank::Knight => 0,
            Rank::Pawn => 1,
            Rank::King => 1,
            Rank::Rook => 8,
            Rank::Bishop => 8,
            Rank::Queen => 8,
        };
        match p.rank {
            Rank::Knight => {
                let knight_moves: Vec<Vec<isize>> = vec![
                    vec![1, 2],
                    vec![1, -2],
                    vec![2, 1],
                    vec![2, -1],
                    vec![-1, 2],
                    vec![-1, -2],
                    vec![-2, 1],
                    vec![-2, -1],
                ];
                for r in 0..knight_moves.len() {
                    let to = Location::new(
                        (row as isize) + knight_moves[r][0],
                        (column as isize) + knight_moves[r][1],
                    );
                    // If we land out of bounds
                    if !to.valid_location {
                        continue;
                    }
                    // If we land on a piece
                    let land_on_piece = &self.squares[to.row as usize][to.column as usize]
                        .piece
                        .is_some();
                    if *land_on_piece == true {
                        let piece = &self.squares[to.row as usize][to.column as usize]
                            .piece
                            .as_ref()
                            .unwrap();
                        // If we land on our own piece
                        if piece.team == self.next_to_move {
                            continue;
                        }
                        // If we land on enemy piece
                        if piece.team != self.next_to_move {
                            let n = Move::new(self, from, to, Some(piece.rank.clone()));
                            moves.push(n);
                            continue;
                        }
                    }
                    let n = Move::new(self, from, to, None);
                    moves.push(n);
                }
            }
            _ => {
                for delta in 1..(max_distance + 1) {
                    let to: Location;
                    match &dir {
                        Direction::N => to = Location::new(row + delta, column),
                        Direction::S => to = Location::new(row - delta, column),
                        Direction::E => to = Location::new(row, column + delta),
                        Direction::W => to = Location::new(row, column - delta),
                        Direction::NE => to = Location::new(row + delta, column + delta),
                        Direction::NW => to = Location::new(row + delta, column - delta),
                        Direction::SW => to = Location::new(row - delta, column + delta),
                        Direction::SE => to = Location::new(row - delta, column - delta),
                        _ => to = Location::new(-1, -1),
                    }
                    // If we land out of bounds
                    if !to.valid_location {
                        break;
                    }
                    // If we land on a piece
                    let land_on_piece = &self.squares[to.row as usize][to.column as usize]
                        .piece
                        .is_some();
                    if *land_on_piece == true {
                        let piece = &self.squares[to.row as usize][to.column as usize]
                            .piece
                            .as_ref()
                            .unwrap();
                        // If we land on our own piece
                        if piece.team == self.next_to_move {
                            break;
                        }
                        // If we land on enemy piece
                        if piece.team != self.next_to_move {
                            let n = Move::new(self, from, to, Some(piece.rank.clone()));
                            moves.push(n);
                            break;
                        }
                    }
                    let n = Move::new(self, from, to, None);
                    moves.push(n);
                }
            }
        }
        return moves;
    }

    fn generate_all_possible_moves(b: &Board, sqr: &Square) -> Vec<Move> {
        let from = sqr.location.clone();
        let mut moves: Vec<Option<Move>> = vec![];
        match &sqr.piece {
            Some(piece) => match piece.rank {
                Rank::Pawn => match piece.team {
                    Team::White => {
                        moves.append(&mut b.navigate(Direction::N, from));
                    }
                    Team::Black => {
                        moves.append(&mut b.navigate(Direction::S, from));
                    }
                },
                Rank::Knight => moves.append(&mut b.navigate(Direction::KNIGHT, from)),
                Rank::King => {
                    moves.append(&mut b.navigate(Direction::N, from));
                    moves.append(&mut b.navigate(Direction::S, from));
                    moves.append(&mut b.navigate(Direction::E, from));
                    moves.append(&mut b.navigate(Direction::W, from));
                    moves.append(&mut b.navigate(Direction::NW, from));
                    moves.append(&mut b.navigate(Direction::NE, from));
                    moves.append(&mut b.navigate(Direction::SW, from));
                    moves.append(&mut b.navigate(Direction::SE, from));
                }
                Rank::Rook => {
                    moves.append(&mut b.navigate(Direction::N, from));
                    moves.append(&mut b.navigate(Direction::S, from));
                    moves.append(&mut b.navigate(Direction::E, from));
                    moves.append(&mut b.navigate(Direction::W, from));
                }
                Rank::Bishop => {
                    moves.append(&mut b.navigate(Direction::NW, from));
                    moves.append(&mut b.navigate(Direction::NE, from));
                    moves.append(&mut b.navigate(Direction::SW, from));
                    moves.append(&mut b.navigate(Direction::SE, from));
                }
                Rank::Queen => {
                    moves.append(&mut b.navigate(Direction::N, from));
                    moves.append(&mut b.navigate(Direction::S, from));
                    moves.append(&mut b.navigate(Direction::E, from));
                    moves.append(&mut b.navigate(Direction::W, from));
                    moves.append(&mut b.navigate(Direction::NW, from));
                    moves.append(&mut b.navigate(Direction::NE, from));
                    moves.append(&mut b.navigate(Direction::SW, from));
                    moves.append(&mut b.navigate(Direction::SE, from));
                }
            },
            _ => {}
        }
        return moves
            .into_iter()
            .filter(|m| m.is_some())
            .map(|m| m.unwrap())
            .filter(|m| m.to.valid_location)
            .collect();
    }

    pub fn choose_next_move(moves: Vec<Move>) -> String {
        let index = (rand::random::<f32>() * moves.len() as f32).floor() as usize;
        let item = &moves[index];
        let next = Location::coords_to_str(
            item.from.column as usize,
            item.from.row as usize,
            item.to.column as usize,
            item.to.row as usize,
        );
        println!("next_move {:?}", next);
        return next;
    }

    pub fn find_next_move(b: Board) -> String {
        return Board::choose_next_move(
            Board::find_next_moves(b.clone())
                .into_iter()
                .filter(|m| !Board::is_king_checked(&b, m))
                .collect(),
        );
    }

    pub fn find_next_moves(b: Board) -> Vec<Move> {
        let mut all_moves = vec![];
        for row in (0..b.squares.len()).rev() {
            for column in 0..b.squares[row].len() {
                let curr_square = &b.squares[row][column];
                match &curr_square.piece {
                    Some(piece) => {
                        if piece.team == b.next_to_move {
                            let mut result: Vec<Move> =
                                Board::generate_all_possible_moves(&b, &curr_square);
                            all_moves.append(&mut result);
                        }
                    }
                    None => (),
                }
            }
        }
        return all_moves;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        println!("##a#b#c#d#e#f#g#h###");
        for x in (0..self.squares.len()).rev() {
            print!("{}#", x + 1);
            for y in 0..self.squares[x].len() {
                match &self.squares[x][y].piece {
                    Some(piece) => print!("{} ", piece),
                    None => print!(". "),
                }
            }
            print!("#{}", x + 1);
            println!();
        }
        println!("##a#b#c#d#e#f#g#h###");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_chess_move() {
        let board = Board::new();
        let mv = Move::new(&board, Location::new(1, 5), Location::new(3, 5), None).unwrap();
        println!("{}", board);
        println!("Is King Checked {}", Board::is_king_checked(&board, &mv));
    }

    #[test]
    fn create_chess_board() {
        let mut board = Board::new();
        println!("{}", board);
        for _n in 0..5 {
            let next_move = Board::find_next_move(board.clone());
            board = Board::move_piece(board, next_move);
            println!("{}", board);
        }
    }
}
