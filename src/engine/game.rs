use crate::engine::board::Board;
use crate::engine::piece::Piece;
use crate::engine::types::{Color, Position};
use crate::engine::movement::get_legal_moves;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnState {
    White,
    Black,
}

impl TurnState {
    pub fn switch(self) -> Self {
        match self {
            TurnState::White => TurnState::Black,
            TurnState::Black => TurnState::White,
        }
    }

    pub fn as_color(self) -> Color {
        match self {
            TurnState::White => Color::White,
            TurnState::Black => Color::Black,
        }
    }
}

pub struct GameState {
    pub board: Board,
    pub turn: TurnState,
}

impl GameState {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            turn: TurnState::White,
        }
    }

    /// 嘗試從 `from` 移動到 `to`
    /// 若合法則執行移動並切換回合，否則回傳錯誤訊息
    pub fn try_move(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        // 不保留對 piece 的 reference，只拿出 id 與 color
        let piece_info = self.board.get_piece_at(from)
            .map(|p| (p.id, p.color))
            .ok_or("來源格沒有棋子")?;
    
        if piece_info.1 != self.turn.as_color() {
            return Err("不是你的回合");
        }
    
        let piece = self.board.pieces.get(&piece_info.0).unwrap();
        let legal_moves = get_legal_moves(&self.board, piece);
    
        if !legal_moves.contains(&to) {
            return Err("非法走法");
        }
    
        self.board.remove_piece_at(to);
        self.board.move_piece(piece_info.0, to);
        self.turn = self.turn.switch();
    
        Ok(())
    }    
}

pub fn check_game_end(board: &Board) -> Option<&'static str> {
    let mut white_king_exists = false;
    let mut black_king_exists = false;

    for piece in board.pieces.values() {
        use crate::engine::types::PieceType::*;
        if piece.kind == King {
            match piece.color {
                crate::engine::types::Color::White => white_king_exists = true,
                crate::engine::types::Color::Black => black_king_exists = true,
            }
        }
    }

    match (white_king_exists, black_king_exists) {
        (true, true) => None,
        (false, true) => Some("黑方勝利！"),
        (true, false) => Some("白方勝利！"),
        (false, false) => Some("雙方國王都不見了？！？！"),
    }
}
