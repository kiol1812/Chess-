use crate::engine::board::Board;
use crate::engine::piece::Piece;
use crate::engine::types::{Color, PieceType, Tile, Position};
use crate::engine::utils::random_position;
use crate::engine::evaluator::evaluate_board;

use rand::seq::{IndexedRandom, SliceRandom};
use rand::Rng;


/// 隨機產生一個指定大小、指定棋子數量的棋盤殘局
pub fn generate_random_board(
    width: usize,
    height: usize,
    num_white: usize,
    num_black: usize,
    num_blocked: usize,
) -> Board {
    let mut board = Board::new(width, height);
    let mut rng = rand::thread_rng();

    // 放置禁區
    let mut blocked_count = 0;
    while blocked_count < num_blocked {
        let pos = random_position(width, height);
        if board.tiles[pos.0][pos.1] == Tile::Empty {
            board.set_blocked(pos);
            blocked_count += 1;
        }
    }

    // 放置白王
    loop {
        let pos = random_position(width, height);
        if board.is_tile_free(pos) {
            board.add_piece(Piece {
                id: 0,
                kind: PieceType::King,
                color: Color::White,
                pos,
            });
            break;
        }
    }

    // 放置黑王
    loop {
        let pos = random_position(width, height);
        if board.is_tile_free(pos) {
            board.add_piece(Piece {
                id: 0,
                kind: PieceType::King,
                color: Color::Black,
                pos,
            });
            break;
        }
    }

    // 可用的非王棋種
    let piece_pool = [
        PieceType::Queen,
        PieceType::Rook,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Pawn,
    ];

    // 放置白棋（不包含王）
    let mut placed = 1; // 已放置白王
    while placed < num_white {
        let pos = random_position(width, height);
        if board.is_tile_free(pos) {
            let piece_type = *piece_pool.choose(&mut rng).unwrap();
            board.add_piece(Piece {
                id: 0,
                kind: piece_type,
                color: Color::White,
                pos,
            });
            placed += 1;
        }
    }

    // 放置黑棋（不包含王）
    let mut placed = 1; // 已放置黑王
    while placed < num_black {
        let pos = random_position(width, height);
        if board.is_tile_free(pos) {
            let piece_type = *piece_pool.choose(&mut rng).unwrap();
            board.add_piece(Piece {
                id: 0,
                kind: piece_type,
                color: Color::Black,
                pos,
            });
            placed += 1;
        }
    }

    board
}

/// 重複生成直到符合指定難度閾值（例如對白方不利）
pub fn generate_by_difficulty(
    width: usize,
    height: usize,
    num_white: usize,
    num_black: usize,
    num_blocked: usize,
    threshold: i32,
) -> Board {
    loop {
        let board = generate_random_board(width, height, num_white, num_black, num_blocked);
        let score = evaluate_board(&board);
        if score <= threshold {
            return board;
        }
    }
}
