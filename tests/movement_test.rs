use Chess_::engine::board::Board;
use Chess_::engine::piece::Piece;
use Chess_::engine::types::{Color, PieceType};
use Chess_::engine::movement::get_legal_moves;

#[test]
fn test_rook_moves_straight_lines() {
    let mut board = Board::new(8, 8);
    let rook = Piece {
        id: 0,
        kind: PieceType::Rook,
        color: Color::White,
        pos: (4, 4),
    };
    board.add_piece(rook);

    let rook_ref = board.get_piece_at((4, 4)).unwrap();
    let moves = get_legal_moves(&board, rook_ref);
    assert!(moves.contains(&(4, 0))); // 上
    assert!(moves.contains(&(4, 7))); // 下
    assert!(moves.contains(&(0, 4))); // 左
    assert!(moves.contains(&(7, 4))); // 右
}

#[test]
fn test_bishop_moves_diagonals() {
    let mut board = Board::new(8, 8);
    let bishop = Piece {
        id: 0,
        kind: PieceType::Bishop,
        color: Color::White,
        pos: (3, 3),
    };
    board.add_piece(bishop);

    let bishop_ref = board.get_piece_at((3, 3)).unwrap();
    let moves = get_legal_moves(&board, bishop_ref);
    assert!(moves.contains(&(0, 0)));
    assert!(moves.contains(&(6, 6)));
    assert!(moves.contains(&(0, 6)));
    assert!(moves.contains(&(6, 0)));
}

#[test]
fn test_queen_moves_combined() {
    let mut board = Board::new(8, 8);
    let queen = Piece {
        id: 0,
        kind: PieceType::Queen,
        color: Color::White,
        pos: (4, 4),
    };
    board.add_piece(queen);

    let queen_ref = board.get_piece_at((4, 4)).unwrap();
    let moves = get_legal_moves(&board, queen_ref);
    // 直線
    assert!(moves.contains(&(4, 0)));
    assert!(moves.contains(&(4, 7)));
    assert!(moves.contains(&(0, 4)));
    assert!(moves.contains(&(7, 4)));

    // 對角
    assert!(moves.contains(&(0, 0)));
    assert!(moves.contains(&(7, 7)));
    assert!(moves.contains(&(1, 7)));
    assert!(moves.contains(&(7, 1)));
}

#[test]
fn test_knight_moves() {
    let mut board = Board::new(8, 8);
    let knight = Piece {
        id: 0,
        kind: PieceType::Knight,
        color: Color::White,
        pos: (3, 3),
    };
    board.add_piece(knight);

    let knight_ref = board.get_piece_at((3, 3)).unwrap();
    let moves = get_legal_moves(&board, knight_ref);

    let expected = vec![
        (5, 4), (4, 5), (2, 5), (1, 4),
        (1, 2), (2, 1), (4, 1), (5, 2),
    ];
    for m in expected {
        assert!(moves.contains(&m), "Knight should be able to move to {:?}", m);
    }
}


#[test]
fn test_pawn_moves_forward_and_capture() {
    let mut board = Board::new(8, 8);
    let pawn = Piece {
        id: 0,
        kind: PieceType::Pawn,
        color: Color::White,
        pos: (3, 3),
    };
    board.add_piece(pawn);

    // 加入敵人
    board.add_piece(Piece {
        id: 0,
        kind: PieceType::Knight,
        color: Color::Black,
        pos: (2, 2),
    });
    board.add_piece(Piece {
        id: 0,
        kind: PieceType::Knight,
        color: Color::Black,
        pos: (4, 2),
    });

    let pawn_ref = board.get_piece_at((3, 3)).unwrap();
    let moves = get_legal_moves(&board, &pawn_ref);
    assert!(moves.contains(&(3, 2)), "Pawn should be able to move forward");
    assert!(moves.contains(&(2, 2)), "Pawn should be able to capture left");
    assert!(moves.contains(&(4, 2)), "Pawn should be able to capture right");
}

#[test]
fn test_king_moves_respects_boundaries_and_blocked() {
    let mut board = Board::new(8, 8);
    let king = Piece {
        id: 0,
        kind: PieceType::King,
        color: Color::White,
        pos: (0, 0),
    };
    board.add_piece(king);
    board.set_blocked((0, 1));

    let king_ref = board.get_piece_at((0, 0)).unwrap();
    let moves = get_legal_moves(&board, &king_ref);
    assert!(moves.contains(&(1, 0)));
    assert!(moves.contains(&(1, 1)));
    assert!(!moves.contains(&(0, 1)), "Should not move into blocked tile");
}

#[test]
fn test_show_allowed_movement() {
    let mut board = Board::new(8, 8);
    // 設置禁區（破洞）
    board.set_blocked((3, 3));
    board.set_blocked((4, 4));

    // 新增一個白色騎士
    let knight = Piece {
        id: 0,
        kind: PieceType::Knight,
        color: Color::White,
        pos: (2, 2),
    };
    board.add_piece(knight);

    // 新增一個黑色小兵
    let pawn = Piece {
        id: 0,
        kind: PieceType::Pawn,
        color: Color::Black,
        pos: (4, 2),
    };
    board.add_piece(pawn);

    // 新增一個白色國王
    let king = Piece {
        id: 0,
        kind: PieceType::King,
        color: Color::White,
        pos: (5, 5),
    };
    board.add_piece(king);

    println!("目前棋子與合法走法：");
    for piece in board.pieces.values() {
        let moves = get_legal_moves(&board, piece);
        println!(
            "  - {:?} at {:?} 可走至: {:?}",
            piece.kind, piece.pos, moves
        );
    }
}