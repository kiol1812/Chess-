// Prevent console window in release build on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::rc::Rc;
use std::cell::RefCell;

use slint::{ModelRc, VecModel, SharedString};

use Chess_::engine;
use engine::generator::generate_random_board;
use engine::board::Board;
use engine::evaluator::evaluate_board;
use engine::movement::get_legal_moves;

slint::include_modules!();

fn convert_board_to_gui(board: &Board) -> Vec<SharedString> {
    let mut gui_board = vec![SharedString::from("."); board.width * board.height];
    for y in 0..board.height {
        for x in 0..board.width {
            let idx = y * board.width + x;
            gui_board[idx] = if board.tiles[x][y] == engine::types::Tile::Blocked {
                SharedString::from("#")
            } else if let Some(p) = board.get_piece_at((x, y)) {
                use engine::types::PieceType::*;
                let base = match p.kind {
                    King => "K", Queen => "Q", Rook => "R",
                    Bishop => "B", Knight => "N", Pawn => "P",
                    Custom(_) => "?",
                };
                match p.color {
                    engine::types::Color::White => SharedString::from(base),
                    engine::types::Color::Black => SharedString::from(base.to_lowercase()),
                }
            } else {
                SharedString::from(".")
            };
        }
    }
    gui_board
}

fn index_to_pos(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let app_weak = app.as_weak();

    // 建立 Rc<RefCell> 共享棋盤狀態
    let board = Rc::new(RefCell::new(generate_random_board(8, 8, 5, 5, 5)));

    // 🔁 註冊生成回呼
    {
        let board = board.clone();
        app.on_generate({
            let app = app_weak.clone();
            move || {
                let mut board = board.borrow_mut();
                *board = generate_random_board(8, 8, 5, 5, 5);
                board.print_board();

                let score = evaluate_board(&board);
                println!("[GUI 評分] 分數: {}", score);

                let gui_board = convert_board_to_gui(&board);
                if let Some(app) = app.upgrade() {
                    app.set_board(ModelRc::new(Rc::new(VecModel::from(gui_board))));
                    app.set_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                }
            }
        });
    }

    // 🔁 註冊棋子點擊回呼
    {
        let board = board.clone();
        app.on_piece_clicked({
            let app = app_weak.clone();
            move |index| {
                let board = board.borrow();
                let (x, y) = index_to_pos(index as usize, board.width);
                if let Some(piece) = board.get_piece_at((x, y)) {
                    let legal_moves = get_legal_moves(&board, piece);
                    let mut highlights = vec![false; board.width * board.height];
                    for (x, y) in legal_moves {
                        let idx = y * board.width + x;
                        if idx < highlights.len() {
                            highlights[idx] = true;
                        }
                    }
                    if let Some(app) = app.upgrade() {
                        app.set_highlights(ModelRc::new(Rc::new(VecModel::from(highlights))));
                    }
                }
            }
        });
    }

    // 初始畫面載入一次
    {
        let board = board.borrow();
        let gui_board = convert_board_to_gui(&board);
        app.set_board(ModelRc::new(Rc::new(VecModel::from(gui_board))));
        app.set_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
    }

    app.run()
}
