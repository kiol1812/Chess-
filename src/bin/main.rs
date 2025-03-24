#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use slint::{ModelRc, VecModel, SharedString};

use Chess_::engine;
use engine::board::Board;
use engine::types::Position;
use engine::evaluator::evaluate_board;
use engine::generator::generate_random_board;
use engine::game::{GameState, TurnState, check_game_end};
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

    // ✅ 建立共享狀態
    let game_state = Rc::new(RefCell::new(GameState::new(generate_random_board(8, 8, 5, 5, 5))));
    let selected_index: Rc<RefCell<Option<usize>>> = Rc::new(RefCell::new(None));

    // 🔁 註冊生成回呼
    {
        let game_state = game_state.clone();
        let selected_index = selected_index.clone();
        app.on_generate({
            let app = app_weak.clone();
            move || {
                let mut game = game_state.borrow_mut();
                *game = GameState::new(generate_random_board(8, 8, 5, 5, 5));
                game.board.print_board();

                let score = evaluate_board(&game.board);
                println!("[GUI 評分] 分數: {}", score);

                let gui_board = convert_board_to_gui(&game.board);
                if let Some(app) = app.upgrade() {
                    app.set_gameover(false);
                    app.set_board(ModelRc::new(Rc::new(VecModel::from(gui_board))));
                    app.set_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                    app.set_selected_index(-1);
                    app.set_turn_text(SharedString::from(format!("{:?}", game.turn)));
                    app.set_score_text(SharedString::from(format!("{}", evaluate_board(&game.board))));
                    app.set_from_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                    app.set_to_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                }
                *selected_index.borrow_mut() = None;
            }
        });
    }

    // 🔁 註冊棋子點擊回呼
    {
        let game_state = game_state.clone();
        let selected_index = selected_index.clone();
        app.on_piece_clicked({
            let app = app_weak.clone();
            move |index| {
                let mut game = game_state.borrow_mut();
                let mut selected = selected_index.borrow_mut();
                let (x, y) = index_to_pos(index as usize, game.board.width);
                let pos = (x, y);

                if selected.is_none() {
                    // ✅ 選取棋子
                    if let Some(piece) = game.board.get_piece_at(pos) {
                        if piece.color == game.turn.as_color() {
                            let legal = get_legal_moves(&game.board, piece);
                            let mut highlights = vec![false; game.board.width * game.board.height];
                            for (x, y) in legal {
                                highlights[y * game.board.width + x] = true;
                            }
                            if let Some(app) = app.upgrade() {
                                app.set_highlights(ModelRc::new(Rc::new(VecModel::from(highlights))));
                                app.set_selected_index(index as i32);
                            }
                            *selected = Some(index as usize);
                        }
                    }
                } else {
                    let from_index = selected.unwrap();
                    if from_index == (index as usize) {
                        // ✅ 再次點選同一格 → 取消選取
                        if let Some(app) = app.upgrade() {
                            app.set_selected_index(-1);
                            app.set_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                        }
                        *selected = None;
                        return;
                    }
                    // ✅ 嘗試移動
                    let from_index = selected.unwrap();
                    let from = index_to_pos(from_index, game.board.width);
                    let to = index_to_pos(index as usize, game.board.width);

                    match game.try_move(from, to) {
                        Ok(_) => {
                            let gui_board = convert_board_to_gui(&game.board);
                            if let Some(app) = app.upgrade() {
                                app.set_turn_text(SharedString::from(format!("{:?}", game.turn)));
                                app.set_score_text(SharedString::from(format!("{}", evaluate_board(&game.board))));
                                app.set_board(ModelRc::new(Rc::new(VecModel::from(gui_board))));
                                app.set_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                                app.set_from_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                                app.set_to_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                                app.set_selected_index(-1);
                            }
                            *selected = None;
                            if let Some(result) = check_game_end(&game.board) {
                                println!("對局結束：{}", result);
                                if let Some(app) = app.upgrade() {
                                    app.set_turn_text(SharedString::from("game over"));
                                    app.set_gameover(true);
                                }
                                return;
                            }

                             // ✅ 電腦（黑方）自動行動
                            if game.turn == TurnState::Black {
                                if let Some((from, to)) = game.try_ai_move(3) {
                                    println!("AI move: {:?} -> {:?}", from, to);

                                    let gui_board = convert_board_to_gui(&game.board);
                                    let mut highlight_from = vec![false; game.board.width * game.board.height];
                                    let mut highlight_to = vec![false; game.board.width * game.board.height];
                                    let (fx, fy) = from;
                                    highlight_from[fy * game.board.width + fx] = true;
                                    let (tx, ty) = to;
                                    highlight_to[ty * game.board.width + tx] = true;
                                    if let Some(app) = app.upgrade() {
                                        app.set_board(ModelRc::new(Rc::new(VecModel::from(gui_board))));
                                        app.set_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                                        app.set_selected_index(-1);
                                        app.set_turn_text(SharedString::from(format!("{:?}", game.turn)));
                                        app.set_score_text(SharedString::from(format!("{}", evaluate_board(&game.board))));
                                        app.set_from_highlights(ModelRc::new(Rc::new(VecModel::from(highlight_from))));
                                        app.set_to_highlights(ModelRc::new(Rc::new(VecModel::from(highlight_to))));
                                        // app.set_turn_text(format!("{:?}", game.turn));
                                        // app.set_score_text(format!("{}", evaluate_board(&game.board)));
                                    }

                                    // ✅ AI 走完 → 再次檢查遊戲是否結束
                                    if let Some(result) = check_game_end(&game.board) {
                                        println!("遊戲結束（AI）：{}", result);

                                        // let mut game = game_state.borrow_mut();
                                        // *game = GameState::new(generate_random_board(8, 8, 5, 5, 5));
                                        // let gui_board = convert_board_to_gui(&game.board);
                                        if let Some(app) = app.upgrade() {
                                            // app.set_board(ModelRc::new(Rc::new(VecModel::from(gui_board))));
                                            // app.set_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                                            // app.set_selected_index(-1);
                                            app.set_turn_text(SharedString::from("game over"));
                                            app.set_gameover(true);
                                            app.set_from_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                                            app.set_to_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
                                            // app.set_turn_text(SharedString::from(format!("{:?}", game.turn)));
                                            // app.set_score_text(SharedString::from(format!("{}", evaluate_board(&game.board))));
                                            // app.set_turn_text(format!("{:?}", game.turn));
                                            // app.set_score_text(format!("{}", evaluate_board(&game.board)));
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            println!("非法移動：{}", e);
                            // 可選：清除選取或保留原選擇
                        }
                    }
                }
            }
        });
    }

    // ✅ 初始畫面載入
    {
        let game = game_state.borrow();
        let gui_board = convert_board_to_gui(&game.board);
        app.set_board(ModelRc::new(Rc::new(VecModel::from(gui_board))));
        app.set_highlights(ModelRc::new(Rc::new(VecModel::from(vec![false; 64]))));
        app.set_selected_index(-1);
        app.set_turn_text(SharedString::from(format!("{:?}", game.turn)));
        app.set_score_text(SharedString::from(format!("{}", evaluate_board(&game.board))));
        let size = game.board.width * game.board.height;
        app.set_from_highlights(empty_bool_vec(size));
        app.set_to_highlights(empty_bool_vec(size));
    }

    app.run()
}
fn empty_bool_vec(size: usize) -> ModelRc<bool> {
    ModelRc::new(Rc::new(VecModel::from(vec![false; size])))
}
