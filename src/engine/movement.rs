use crate::engine::board::Board;
use crate::engine::piece::Piece;
use crate::engine::types::{Color, PieceType, Position};

/// 取得特定棋子的合法走法
pub fn get_legal_moves(board: &Board, piece: &Piece) -> Vec<Position> {
    match piece.kind {
        PieceType::Knight => knight_moves(board, piece),
        PieceType::Rook => rook_moves(board, piece),
        PieceType::Pawn => pawn_moves(board, piece),
        PieceType::Bishop => bishop_moves(board, piece),
        PieceType::Queen => queen_moves(board, piece),
        PieceType::King => king_moves(board, piece),
        PieceType::Custom(_) => vec![], // 可自訂邏輯
    }
}

fn knight_moves(board: &Board, piece: &Piece) -> Vec<Position> {
    let (x, y) = piece.pos;
    let deltas = [
        (2, 1), (1, 2), (-1, 2), (-2, 1),
        (-2, -1), (-1, -2), (1, -2), (2, -1),
    ];

    deltas.iter()
        .filter_map(|(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 {
                let pos = (nx as usize, ny as usize);
                if board.is_tile_free(pos) || board.get_piece_at(pos).map(|p| p.color != piece.color).unwrap_or(false) {
                    Some(pos)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn rook_moves(board: &Board, piece: &Piece) -> Vec<Position> {
    straight_line_moves(board, piece, &[(1, 0), (-1, 0), (0, 1), (0, -1)])
}

fn bishop_moves(board: &Board, piece: &Piece) -> Vec<Position> {
    straight_line_moves(board, piece, &[(1, 1), (1, -1), (-1, 1), (-1, -1)])
}

fn queen_moves(board: &Board, piece: &Piece) -> Vec<Position> {
    let mut moves = rook_moves(board, piece);
    moves.extend(bishop_moves(board, piece));
    moves
}

fn king_moves(board: &Board, piece: &Piece) -> Vec<Position> {
    let (x, y) = piece.pos;
    let deltas = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    deltas.iter()
        .filter_map(|(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 {
                let pos = (nx as usize, ny as usize);
                if board.is_tile_free(pos) || board.get_piece_at(pos).map(|p| p.color != piece.color).unwrap_or(false) {
                    // TODO: 加入檢查是否會被將軍
                    Some(pos)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn pawn_moves(board: &Board, piece: &Piece) -> Vec<Position> {
    let mut moves = Vec::new();
    let (x, y) = piece.pos;
    let direction: isize = match piece.color {
        Color::White => -1,
        Color::Black => 1,
    };
    let ny = y as isize + direction;
    if ny >= 0 && ny < board.height as isize {
        // 向前移動
        let forward_pos = (x, ny as usize);
        if board.is_tile_free(forward_pos) {
            moves.push(forward_pos);
        }
        // 吃子：左斜與右斜
        for dx in [-1, 1] {
            let nx = x as isize + dx;
            if nx >= 0 && nx < board.width as isize {
                let pos = (nx as usize, ny as usize);
                if let Some(target) = board.get_piece_at(pos) {
                    if target.color != piece.color {
                        moves.push(pos);
                    }
                }
            }
        }
    }
    moves
}

fn straight_line_moves(board: &Board, piece: &Piece, directions: &[(isize, isize)]) -> Vec<Position> {
    let mut moves = vec![];
    for &(dx, dy) in directions {
        let (mut x, mut y) = (piece.pos.0 as isize, piece.pos.1 as isize);
        loop {
            x += dx;
            y += dy;
            if x < 0 || y < 0 || x >= board.width as isize || y >= board.height as isize {
                break;
            }
            let pos = (x as usize, y as usize);
            if board.tiles[pos.0][pos.1] != crate::engine::types::Tile::Empty {
                break;
            }
            if let Some(p) = board.get_piece_at(pos) {
                if p.color != piece.color {
                    moves.push(pos); // 可以吃敵方
                }
                break; // 遇到任何棋子都不能再走
            }
            moves.push(pos);
        }
    }
    moves
}
