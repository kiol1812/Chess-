use crate::engine::board::Board;
use crate::engine::movement::get_legal_moves;
use crate::engine::evaluator::evaluate_board;
use crate::engine::types::{Color, Position};
use crate::engine::piece::Piece;

/// 電腦選擇最佳移動（含 Alpha-Beta 剪枝）
pub fn choose_best_move(board: &Board, color: Color, depth: usize) -> Option<(Position, Position)> {
    let mut best_score = if color == Color::White { i32::MIN } else { i32::MAX };
    let mut best_move = None;

    for piece in board.pieces.values() {
        if piece.color != color {
            continue;
        }

        let moves = get_legal_moves(board, piece);
        for to in moves {
            let mut new_board = board.clone();
            new_board.remove_piece_at(to);
            new_board.move_piece(piece.id, to);

            let score = minimax_ab(
                &new_board,
                depth - 1,
                color.opposite(),
                i32::MIN,
                i32::MAX,
            );

            let better = if color == Color::White {
                score > best_score
            } else {
                score < best_score
            };

            if better {
                best_score = score;
                best_move = Some((piece.pos, to));
            }
        }
    }

    best_move
}

/// Minimax + Alpha-Beta 剪枝遞迴主體
fn minimax_ab(board: &Board, depth: usize, current_color: Color, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return evaluate_board(board);
    }

    let mut best_score = if current_color == Color::White { i32::MIN } else { i32::MAX };

    for piece in board.pieces.values() {
        if piece.color != current_color {
            continue;
        }

        let moves = get_legal_moves(board, piece);
        for to in moves {
            let mut new_board = board.clone();
            new_board.remove_piece_at(to);
            new_board.move_piece(piece.id, to);

            let score = minimax_ab(&new_board, depth - 1, current_color.opposite(), alpha, beta);

            if current_color == Color::White {
                best_score = best_score.max(score);
                alpha = alpha.max(score);
                if beta <= alpha {
                    break; // beta 剪枝
                }
            } else {
                best_score = best_score.min(score);
                beta = beta.min(score);
                if beta <= alpha {
                    break; // alpha 剪枝
                }
            }
        }
    }

    best_score
}

/// 顏色互換工具
trait OppositeColor {
    fn opposite(self) -> Self;
}

impl OppositeColor for Color {
    fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
