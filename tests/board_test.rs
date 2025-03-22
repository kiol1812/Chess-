use Chess_::engine::board::Board;
use Chess_::engine::piece::Piece;
use Chess_::engine::types::{Color, PieceType, Tile};

#[test]
fn test_create_board() {
    let board = Board::new(8, 8);
    assert_eq!(board.width, 8);
    assert_eq!(board.height, 8);
    assert_eq!(board.tiles[0][0], Tile::Empty);
}

#[test]
fn test_set_blocked() {
    let mut board = Board::new(5, 5);
    board.set_blocked((2, 3));
    assert_eq!(board.tiles[2][3], Tile::Blocked);
}

#[test]
fn test_add_and_get_piece() {
    let mut board = Board::new(5, 5);
    let piece = Piece {
        id: 0,
        kind: PieceType::Knight,
        color: Color::White,
        pos: (1, 1),
    };
    let id = board.add_piece(piece);
    let retrieved = board.get_piece_at((1, 1)).unwrap();
    assert_eq!(retrieved.id, id);
    assert_eq!(retrieved.kind, PieceType::Knight);
}

#[test]
fn test_is_tile_free() {
    let mut board = Board::new(5, 5);
    board.set_blocked((2, 2));
    assert!(!board.is_tile_free((2, 2))); // 被封鎖
    let piece = Piece {
        id: 0,
        kind: PieceType::Pawn,
        color: Color::Black,
        pos: (1, 1),
    };
    board.add_piece(piece);
    assert!(!board.is_tile_free((1, 1))); // 有棋子
    assert!(board.is_tile_free((0, 0)));  // 空白格
}
