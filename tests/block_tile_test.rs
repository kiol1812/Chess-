use Chess_::engine::board::Board;
use Chess_::engine::piece::Piece;
use Chess_::engine::types::{Color, PieceType, Tile};

#[test]
fn test_piece_not_on_blocked_tile() {
    let mut board = Board::new(8, 8);

    // 將 (3, 3) 設為禁區
    board.set_blocked((3, 3));
    assert_eq!(board.tiles[3][3], Tile::Blocked);
    assert!(!board.is_tile_free((3, 3)));

    // 嘗試在禁區放棋子
    let piece = Piece {
        id: 0,
        kind: PieceType::Knight,
        color: Color::White,
        pos: (3, 3),
    };
    let _placed_id = board.add_piece(piece);

    // 應該沒成功放上（因 add_piece 不處理合法性時，這個測試可作為提醒）
    let placed_piece = board.get_piece_at((3, 3));
    if placed_piece.is_some() {
        panic!("棋子不應該被放置在禁區！");
    }
}
