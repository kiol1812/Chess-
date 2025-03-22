use crate::engine::board::Board;
use crate::engine::piece::Piece;
use crate::engine::types::{PieceType, Color};

/// 靜態評估函式：正值表示白方優勢，負值表示黑方優勢
/// 評分依據包含：物料分數 + 位置獎勵（中心控制）+ 王的安全性
pub fn evaluate_board(board: &Board) -> i32 {
    let mut score = 0;
    let center_bonus: i32 = 1; // 中心格額外加分

    let center_x = board.width as i32 / 2;
    let center_y = board.height as i32 / 2;

    for piece in board.pieces.values() {
        let base_value = match piece.kind {
            PieceType::King => 0,
            PieceType::Queen => 9,
            PieceType::Rook => 5,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Pawn => 1,
            PieceType::Custom(v) => v as i32,
        };

        // 簡單位置加分（越靠近中心越好）
        // 中央控制有助於移動、牽制敵方，特別在殘局中更有效
        let (x, y) = (piece.pos.0 as i32, piece.pos.1 as i32);
        let dist_x = (center_x - x).abs();
        let dist_y = (center_y - y).abs();
        let positional_bonus = center_bonus.saturating_sub(dist_x + dist_y);

        // 王的安全性：靠邊更安全（簡化模型）
        let king_safety_bonus = if piece.kind == PieceType::King {
            let near_edge = x == 0 || y == 0 || x == board.width as i32 - 1 || y == board.height as i32 - 1;
            if near_edge { 1 } else { -1 }
        } else {
            0
        };

        let total_value = base_value + positional_bonus + king_safety_bonus;

        score += match piece.color {
            Color::White => total_value,
            Color::Black => -total_value,
        }
    }
    score
}

pub fn evaluate_board_verbose(board: &Board) -> i32 {
    let mut score = 0;
    let center_bonus: i32 = 1;

    let center_x = board.width as i32 / 2;
    let center_y = board.height as i32 / 2;

    println!("棋子評分明細：");

    for piece in board.pieces.values() {
        let base_value = match piece.kind {
            PieceType::King => 0,
            PieceType::Queen => 9,
            PieceType::Rook => 5,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Pawn => 1,
            PieceType::Custom(v) => v as i32,
        };

        let (x, y) = (piece.pos.0 as i32, piece.pos.1 as i32);
        let dist_x = (center_x - x).abs();
        let dist_y = (center_y - y).abs();
        let positional_bonus = center_bonus.saturating_sub(dist_x + dist_y);

        let king_safety_bonus = if piece.kind == PieceType::King {
            let near_edge = x == 0 || y == 0 || x == board.width as i32 - 1 || y == board.height as i32 - 1;
            if near_edge { 1 } else { -1 }
        } else {
            0
        };

        let total = base_value + positional_bonus + king_safety_bonus;
        let signed = match piece.color {
            Color::White => total,
            Color::Black => -total,
        };

        println!(
            "- {:?} at ({}, {}): base={}, pos_bonus={}, king_safe={}, total={} [{}]",
            piece.kind, piece.pos.0, piece.pos.1,
            base_value, positional_bonus, king_safety_bonus,
            signed,
            match piece.color {
                Color::White => "White",
                Color::Black => "Black",
            }
        );

        score += signed;
    }

    println!("總分：{}", score);
    score
}
