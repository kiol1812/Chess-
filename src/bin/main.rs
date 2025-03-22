// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{ModelRc, VecModel, SharedString};

use Chess_::engine;
use engine::generator::generate_random_board;
use engine::board::Board;
use engine::evaluator::evaluate_board;

use std::rc::Rc;

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
            // gui_board[idx] = SharedString::from(ch);
        }
    }
    gui_board
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    let app_weak = app.as_weak();
    app.on_generate(move || {
        let board = generate_random_board(8, 8, 5, 5, 5);
        board.print_board();
        let score = evaluate_board(&board);
        println!("[GUI 評分] 分數: {}", score);

        let gui_board:Vec<SharedString> = convert_board_to_gui(&board);
        if let Some(app) = app_weak.upgrade() {
            let model = Rc::new(VecModel::from(gui_board));
            app.set_board(ModelRc::new(model));
        }
    });

    app.run()
}