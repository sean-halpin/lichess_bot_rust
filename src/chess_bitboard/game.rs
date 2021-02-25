use chess::Board;
use chess::ChessMove;
use chess::Color;
use chess::MoveGen;
use chess::Piece;
use chess::Square;
use std;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct ChessEngine {
    pub bitboard: Board,
}

pub trait FromString {
    fn piece_char_to_enum(c: char) -> Piece;
    fn from_string(m: String) -> ChessMove;
}

impl FromString for ChessMove {
    fn piece_char_to_enum(c: char) -> Piece {
        println!("{}", c);
        return match c {
            'q' => Piece::Queen,
            'n' => Piece::Knight,
            'b' => Piece::Bishop,
            'r' => Piece::Rook,
            _ => Piece::Queen,
        };
    }
    fn from_string(algebraic: String) -> chess::ChessMove {
        let index_vec: Vec<char> = algebraic.chars().collect();
        let mut sqr_from = String::from(index_vec[0]);
        sqr_from.push(index_vec[1]);
        let mut sqr_to = String::from(index_vec[2]);
        sqr_to.push(index_vec[3]);
        let mut piece: Option<Piece> = None;
        if index_vec.len() > 4 {
            piece = Some(Self::piece_char_to_enum(index_vec[4]));
        }
        return ChessMove::new(
            Square::from_string(sqr_from).unwrap(),
            Square::from_string(sqr_to).unwrap(),
            piece,
        );
    }
}

impl ChessEngine {
    pub fn default() -> Self {
        ChessEngine {
            bitboard: Board::default(),
        }
    }
    pub fn from_fen(fen: String) -> Self {
        let bitboard = Board::from_str(&fen.clone()).unwrap();
        ChessEngine { bitboard }
    }
    pub fn next_to_move(&self) -> Color {
        return self.bitboard.side_to_move();
    }
    pub fn move_piece(b: &ChessEngine, m: ChessMove) -> ChessEngine {
        let mut new_board = Board::default();
        b.bitboard.make_move(m, &mut new_board);
        return ChessEngine {
            bitboard: new_board,
        };
    }
    pub fn find_next_move(b: &ChessEngine, depth: isize) -> ChessMove {
        let mut iterable = MoveGen::new_legal(&b.bitboard);
        let targets = b.bitboard.color_combined(!b.bitboard.side_to_move());
        iterable.set_iterator_mask(*targets);
        let moves = iterable.collect::<Vec<ChessMove>>();
        if moves.len() > 0 {
            let index = (rand::random::<f32>() * moves.len() as f32).floor() as usize;
            return moves[index];
        } else {
            iterable = MoveGen::new_legal(&b.bitboard);
            let moves = iterable.collect::<Vec<ChessMove>>();
            let index = (rand::random::<f32>() * moves.len() as f32).floor() as usize;
            return moves[index];
        }
    }
}

impl std::fmt::Display for ChessEngine {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        println!("##a#b#c#d#e#f#g#h###");
        for rank in '1'..'9' {
            for file in 'a'..'i' {
                if file == 'a' {
                    print!("{}#", rank);
                }
                let mut sqr = String::from(file);
                sqr.push(rank);
                let print: Option<Piece> =
                    self.bitboard.piece_on(Square::from_string(sqr).unwrap());

                match print {
                    Some(p) => print!("{} ", p),
                    None => print!(". "),
                }
                if file == 'h' {
                    print!("{}#", rank);
                }
            }
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
    fn create_chess_board() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut board = ChessEngine::from_fen(fen_string.to_string());
        println!("{}", board);
        for _n in 0..120 {
            let next_move = ChessEngine::find_next_move(&board, 2);
            board = ChessEngine::move_piece(&board, next_move);
            println!("{}", board);
        }
    }

    #[test]
    fn parse_fen() {
        let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen_split = fen_string.split_whitespace().collect::<Vec<&str>>();
        let (
            _fen_pieces,
            _next,
            _castling_ability,
            _en_passant_target,
            _half_move_clock,
            _move_counter,
        ) = match &fen_split[..] {
            &[fen_pieces, next, castling_ability, en_passant_target, half_move_clock, move_counter, ..] => {
                (
                    fen_pieces,
                    next,
                    castling_ability,
                    en_passant_target,
                    half_move_clock,
                    move_counter,
                )
            }
            _ => unreachable!(),
        };
    }
}
