use colored::*;
use std::cmp;
use std::fmt;

static mut MOVE_COUNTER: usize = 0;

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

impl Rank {
    fn valuation(&self, t: Team) -> isize {
        match t {
            Team::White => {
                return match self {
                    Rank::King => 99,
                    Rank::Queen => 9,
                    Rank::Rook => 5,
                    Rank::Knight => 3,
                    Rank::Bishop => 3,
                    Rank::Pawn => 1,
                };
            }
            Team::Black => {
                return match self {
                    Rank::King => -99,
                    Rank::Queen => -9,
                    Rank::Rook => -5,
                    Rank::Knight => -3,
                    Rank::Bishop => -3,
                    Rank::Pawn => -1,
                };
            }
        }
    }
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
        let team_print = |s: &str| match self.team {
            Team::White => s.green(),
            Team::Black => s.red(),
        };
        return match self.rank {
            Rank::King => write!(f, "{}", team_print("K")),
            Rank::Queen => write!(f, "{}", team_print("Q")),
            Rank::Rook => write!(f, "{}", team_print("R")),
            Rank::Knight => write!(f, "{}", team_print("N")),
            Rank::Bishop => write!(f, "{}", team_print("B")),
            Rank::Pawn => write!(f, "{}", team_print("P")),
        };
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
        unsafe {
            MOVE_COUNTER = MOVE_COUNTER + 1;
        }
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

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: Location,
    pub to: Location,
    pub captured: Option<Rank>,
    pub value: isize,
}

impl Move {
    fn new(from: Location, to: Location, captured: Option<Rank>, value: isize) -> Option<Self> {
        if from.valid_location && to.valid_location {
            return Some(Move {
                from,
                to,
                captured,
                value,
            });
        }
        return None;
    }
    fn to_algebraic(&self) -> String {
        let mut result = String::new();
        result.push(Location::index_to_column(self.from.column as usize));
        result.push(Location::index_to_row(self.from.row as usize));
        result.push(Location::index_to_column(self.to.column as usize));
        result.push(Location::index_to_row(self.to.row as usize));
        return result;
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

#[derive(Clone, Debug, PartialEq)]
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

    pub fn move_piece(b: &Board, next_move: String) -> Board {
        let mut cloned_board = b.clone();
        let (from_col, from_row, to_col, to_row) = Location::str_to_coords(next_move);
        match &b.squares[from_row][from_col].piece {
            Some(p) => match p.rank {
                // Handle Pawn Promotion - Presume Always Queen
                Rank::Pawn => match p.team {
                    Team::White => {
                        if to_row == 7 {
                            cloned_board.squares[to_row][to_col].piece =
                                Some(Piece::new(p.team, Rank::Queen));
                            cloned_board.squares[from_row][from_col].piece = None;
                        } else {
                            cloned_board.squares[to_row][to_col].piece = Some(*p);
                            cloned_board.squares[from_row][from_col].piece = None;
                        }
                    }
                    Team::Black => {
                        if to_row == 0 {
                            cloned_board.squares[to_row][to_col].piece =
                                Some(Piece::new(p.team, Rank::Queen));
                            cloned_board.squares[from_row][from_col].piece = None;
                        } else {
                            cloned_board.squares[to_row][to_col].piece = Some(*p);
                            cloned_board.squares[from_row][from_col].piece = None;
                        }
                    }
                },
                Rank::King => {
                    cloned_board.squares[to_row][to_col].piece = Some(*p);
                    cloned_board.squares[from_row][from_col].piece = None;
                    if (from_col as isize - to_col as isize).abs() == 2 {
                        match to_col {
                            2 => {
                                let rook = cloned_board.squares[from_row][0].piece.unwrap();
                                cloned_board.squares[to_row][to_col + 1].piece = Some(rook);
                                cloned_board.squares[to_row][0].piece = None;
                            }
                            6 => {
                                let rook = cloned_board.squares[from_row][7].piece.unwrap();
                                cloned_board.squares[to_row][to_col - 1].piece = Some(rook);
                                cloned_board.squares[to_row][7].piece = None;
                            }
                            _ => (),
                        }
                    }
                }
                _ => {
                    cloned_board.squares[to_row][to_col].piece = Some(*p);
                    cloned_board.squares[from_row][from_col].piece = None;
                }
            },
            None => (),
        }
        match &b.next_to_move {
            Team::White => cloned_board.next_to_move = Team::Black,
            Team::Black => cloned_board.next_to_move = Team::White,
        }
        return cloned_board;
    }

    fn is_own_king_checked(b: &Board, m: &Move) -> bool {
        let move_string = Location::coords_to_str(
            m.from.column as usize,
            m.from.row as usize,
            m.to.column as usize,
            m.to.row as usize,
        );
        let future_board = Board::move_piece(&b, move_string);
        let next_moves = Board::find_valid_moves(&future_board);
        return next_moves
            .into_iter()
            .filter(|m| {
                m.captured.is_some()
                    && m.captured.unwrap() == Rank::King
                    && future_board.squares[m.to.row as usize][m.to.column as usize]
                        .piece
                        .unwrap()
                        .team
                        == b.next_to_move
            })
            .map(|m| m.captured.unwrap())
            .filter(|c| c == &Rank::King)
            .collect::<Vec<Rank>>()
            .len()
            > 0;
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
            Rank::Pawn => match p.team {
                Team::White => match from.row {
                    1 => 2,
                    _ => 1,
                },
                Team::Black => match from.row {
                    6 => 2,
                    _ => 1,
                },
            },
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
                            let n = Move::new(
                                from,
                                to,
                                Some(piece.rank.clone()),
                                piece.rank.valuation(self.next_to_move),
                            );
                            moves.push(n);
                            continue;
                        }
                    }
                    let n = Move::new(from, to, None, 0);
                    moves.push(n);
                }
            }
            Rank::Pawn => {
                for delta in 1..(max_distance + 1) {
                    let to: Location;
                    match delta {
                        1 => match &dir {
                            Direction::N => to = Location::new(row + delta, column),
                            Direction::S => to = Location::new(row - delta, column),
                            Direction::NE => to = Location::new(row + delta, column + delta),
                            Direction::NW => to = Location::new(row + delta, column - delta),
                            Direction::SW => to = Location::new(row - delta, column + delta),
                            Direction::SE => to = Location::new(row - delta, column - delta),
                            _ => to = Location::new(-1, -1),
                        },
                        _ => match &dir {
                            Direction::N => to = Location::new(row + delta, column),
                            Direction::S => to = Location::new(row - delta, column),
                            _ => to = Location::new(-1, -1),
                        },
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
                            if dir == Direction::SE
                                || dir == Direction::SW
                                || dir == Direction::NE
                                || dir == Direction::NW
                            {
                                let n = Move::new(
                                    from,
                                    to,
                                    Some(piece.rank.clone()),
                                    piece.rank.valuation(self.next_to_move),
                                );
                                moves.push(n);
                                break;
                            }
                            if dir == Direction::N || dir == Direction::S {
                                break;
                            }
                        }
                    }
                    if dir == Direction::N || dir == Direction::S {
                        let n = Move::new(from, to, None, 0);
                        moves.push(n);
                    }
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
                            let n = Move::new(
                                from,
                                to,
                                Some(piece.rank.clone()),
                                piece.rank.valuation(self.next_to_move),
                            );
                            moves.push(n);
                            break;
                        }
                    }
                    let n = Move::new(from, to, None, 0);
                    moves.push(n);
                }
            }
        }
        return moves;
    }

    fn generate_all_possible_moves_for_piece(b: &Board, sqr: &Square) -> Vec<Move> {
        let from = sqr.location.clone();
        let mut moves: Vec<Option<Move>> = vec![];
        match &sqr.piece {
            Some(piece) => match piece.rank {
                Rank::Pawn => match piece.team {
                    Team::White => {
                        moves.append(&mut b.navigate(Direction::N, from));
                        moves.append(&mut b.navigate(Direction::NW, from));
                        moves.append(&mut b.navigate(Direction::NE, from));
                    }
                    Team::Black => {
                        moves.append(&mut b.navigate(Direction::S, from));
                        moves.append(&mut b.navigate(Direction::SW, from));
                        moves.append(&mut b.navigate(Direction::SE, from));
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

    pub fn alphabeta(
        board: Board,
        node: Move,
        depth: isize,
        a: isize,
        b: isize,
        maximizing_player: bool,
    ) -> isize {
        let mut alpha = a.clone();
        let mut beta = b.clone();
        let node_board = Board::move_piece(&board, node.to_algebraic());
        let mut valid_moves: Vec<Move> = Board::find_valid_moves(&node_board)
            .into_iter()
            // .filter(|m| !Board::is_own_king_checked(&node_board, m))
            .collect();
        let max_val = valid_moves
            .clone()
            .into_iter()
            .map(|m| m.value)
            .max()
            .unwrap();
        if depth == 0 {
            return max_val;
        }
        valid_moves.sort_by(|a, b| a.value.cmp(&b.value));
        if maximizing_player {
            let mut value = isize::MIN;
            for child in valid_moves.into_iter() {
                let child_board = Board::move_piece(&node_board, child.to_algebraic());
                value = cmp::max(
                    value,
                    Board::alphabeta(child_board, child, depth - 1, alpha, beta, false),
                );
                alpha = cmp::max(alpha, value);
                if beta <= alpha {
                    break;
                }
            }
            return value + max_val;
        } else {
            let mut value = isize::MAX;
            for child in valid_moves.into_iter() {
                let child_board = Board::move_piece(&node_board, child.to_algebraic());
                value = cmp::min(
                    value,
                    Board::alphabeta(child_board, child, depth - 1, alpha, beta, true),
                );
                beta = cmp::min(beta, value);
                if beta <= alpha {
                    break;
                }
            }
            return value + max_val;
        }
    }

    pub fn choose_next_move(b: Board, moves: Vec<Move>, _depth: isize) -> String {
        let res = moves.clone().into_iter().map(|m| {
            Move::new(
                m.from,
                m.to,
                m.captured,
                Board::alphabeta(
                    b.clone(),
                    m,
                    _depth,
                    isize::MIN,
                    isize::MAX,
                    b.next_to_move == Team::White,
                ),
            )
            .unwrap()
        });
        let best = match b.next_to_move {
            Team::White => {
                let max_val = res.clone().into_iter().map(|m| m.value).max().unwrap();
                let best_moves: Vec<Move> = res
                    .clone()
                    .into_iter()
                    .filter(|m| m.value == max_val)
                    .collect();
                let best_moves_capture: Vec<Move> = res
                    .clone()
                    .into_iter()
                    .filter(|m| m.value == max_val)
                    .filter(|m| m.captured.is_some())
                    .collect();
                if best_moves_capture.len() > 0 {
                    best_moves_capture
                        .into_iter()
                        .max_by_key(|m| m.captured.unwrap().valuation(Team::White))
                        .unwrap()
                } else {
                    let index = (rand::random::<f32>() * best_moves.len() as f32).floor() as usize;
                    best_moves[index]
                }
                // res.max_by_key(|m| m.unwrap().value).unwrap().unwrap()
            }
            Team::Black => {
                let min_val = res.clone().into_iter().map(|m| m.value).min().unwrap();
                let best_moves: Vec<Move> = res
                    .clone()
                    .into_iter()
                    .filter(|m| m.value == min_val)
                    .collect();
                let best_moves_capture: Vec<Move> = res
                    .clone()
                    .into_iter()
                    .filter(|m| m.value == min_val)
                    .filter(|m| m.captured.is_some())
                    .collect();
                if best_moves_capture.len() > 0 {
                    best_moves_capture
                        .into_iter()
                        .min_by_key(|m| m.captured.unwrap().valuation(Team::Black))
                        .unwrap()
                } else {
                    let index = (rand::random::<f32>() * best_moves.len() as f32).floor() as usize;
                    best_moves[index]
                }
                // res.min_by_key(|m| m.unwrap().value).unwrap().unwrap()
            }
        };
        let all: Vec<String> = res
            .clone()
            .into_iter()
            .map(|m| format!("{}.{}", m.to_algebraic(), m.value))
            .collect();
        println!("possible move values: {:?}", all);
        println!("who moves: {:?}", b.next_to_move);
        println!("best move value: {:?}", best.value);
        return best.to_algebraic();
    }

    pub fn find_next_move(b: &Board, _depth: isize) -> String {
        let all_possible_moves: Vec<Move> = Board::find_valid_moves(&b)
            .into_iter()
            .filter(|m| !Board::is_own_king_checked(&b, m))
            .collect();
        let chosen = Board::choose_next_move(b.clone(), all_possible_moves, _depth);
        println!("Chosen Move: {}", chosen);
        return chosen;
    }

    pub fn find_valid_moves(b: &Board) -> Vec<Move> {
        let mut all_moves = vec![];
        for row in (0..b.squares.len()).rev() {
            for column in 0..b.squares[row].len() {
                let curr_square = &b.squares[row][column];
                match &curr_square.piece {
                    Some(piece) => {
                        if piece.team == b.next_to_move {
                            let mut result: Vec<Move> =
                                Board::generate_all_possible_moves_for_piece(&b, &curr_square);
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

    // f4h5
    #[test]
    fn create_possible_checkmate_situation() {
        let moves = "";
        let mut board = Board::new();
        for next_move in moves.split_whitespace() {
            board = Board::move_piece(&board, next_move.to_string());
        }
        println!("{}", board);
        let (c1, r1, c2, r2) = Location::str_to_coords("".to_string());
        let next_move = Move::new(
            Location::new(r1 as isize, c1 as isize),
            Location::new(r2 as isize, c2 as isize),
            Some(Rank::Pawn),
            1,
        )
        .unwrap();
        let result = Board::alphabeta(
            board.clone(),
            next_move,
            1,
            isize::MIN,
            isize::MAX,
            board.next_to_move == Team::White,
        );
        println!("alphabeta {}", result);
        board = Board::move_piece(&board, next_move.to_algebraic());
        println!("{}", board);
    }

    #[test]
    fn create_alpha_beta_situation() {
        let moves = "e2e4 e7e6 d2d4 f8c5 d4c5 c7c6 c1g5 d8g5 g1f3 g5c5 b2b4 c5b4 d1d2";
        let mut board = Board::new();
        for next_move in moves.split_whitespace() {
            board = Board::move_piece(&board, next_move.to_string());
        }
        println!("{}", board);
        let (c1, r1, c2, r2) = Location::str_to_coords("b4e4".to_string());
        let next_move = Move::new(
            Location::new(r1 as isize, c1 as isize),
            Location::new(r2 as isize, c2 as isize),
            Some(Rank::Pawn),
            1,
        )
        .unwrap();
        let result = Board::alphabeta(
            board.clone(),
            next_move,
            5,
            isize::MIN,
            isize::MAX,
            board.next_to_move == Team::White,
        );
        println!("alphabeta {}", result);
        board = Board::move_piece(&board, next_move.to_algebraic());
        println!("{}", board);
    }

    #[test]
    fn create_queen_trade_situation() {
        let moves = "e2e4 e7e6 d2d4 f8c5 d4c5 c7c6 c1g5 d8g5 g1f3 g5c5 b2b4 c5b4 d1d2";
        let mut board = Board::new();
        for next_move in moves.split_whitespace() {
            board = Board::move_piece(&board, next_move.to_string());
        }
        println!("{}", board);
        let next_move = Board::find_next_move(&board, 1);
        board = Board::move_piece(&board, next_move);
        println!("{}", board);
    }

    #[test]
    fn create_rook_trade_situation() {
        let moves = "e2e4 b7b6 d2d4 g8h6 b1c3 g7g6 g1f3 d7d6 f1b5 c7c6 b5c6 b8c6 c1h6 c8b7 h6f8 h7h5 f3g5 h8f8 g5h7 f8h8 h7f6 e7f6 e4e5 h5h4 e5d6 c6a5 d6d7 e8d7 d4d5 b7c6 d5c6 d7e8 c6c7 d8d7 c7c8q d7c8 e1g1 a5c6 d1e1 e8f8 e1e4 c8b8 e4c6 b8c8 c6c8 a8c8 a1e1 f6f5 e1e2 f5f4 f1e1 h8h7 e2e8";
        let mut board = Board::new();
        for next_move in moves.split_whitespace() {
            board = Board::move_piece(&board, next_move.to_string());
        }
        println!("{}", board);
        let next_move = Board::find_next_move(&board, 0);
        board = Board::move_piece(&board, next_move);
        println!("{}", board);
    }

    #[test]
    fn create_chess_move() {
        let board = Board::new();
        let mv = Move::new(Location::new(1, 5), Location::new(3, 5), None, 0).unwrap();
        println!("{}", board);
        println!(
            "Is King Checked {}",
            Board::is_own_king_checked(&board, &mv)
        );
    }

    #[test]
    fn create_chess_board() {
        let mut board = Board::new();
        println!("{}", board);
        for _n in 0..120 {
            let next_move = Board::find_next_move(&board, 2);
            board = Board::move_piece(&board, next_move);
            println!("{}", board);
            let next_move = Board::find_next_move(&board, 2);
            board = Board::move_piece(&board, next_move);
            println!("{}", board);
        }
    }
}
