use Chess_::engine::generator::generate_random_board;
use Chess_::engine::generator::generate_by_difficulty;
// use Chess_::engine::evaluator::evaluate_board;
use Chess_::engine::evaluator::evaluate_board_verbose;

#[test]
fn test_random_generation_and_evaluation() {
    // 生成 8x8 棋盤，含 5 白子、5 黑子、5 禁區
    let board = generate_random_board(8, 8, 5, 5, 5);

    // 印出棋盤（含棋子與禁區）
    board.print_board();

    // 評估棋局分數
    let score = evaluate_board_verbose(&board);

    println!("[評分結果] 該殘局分數為：{}", score);

    // 測試成功條件：只是確認能正常執行、不 panic
    assert!(true);
}

#[test]
fn test_generate_by_difficulty() {
    let difficulty_threshold = -5; // 對白方越困難
    let board = generate_by_difficulty(8, 8, 5, 5, 5, difficulty_threshold);

    println!("[根據難度閾值生成殘局，白方分數 <= {}]", difficulty_threshold);
    board.print_board();

    let score = evaluate_board_verbose(&board);
    println!("[評分結果] 該殘局分數為：{}", score);

    assert!(score <= difficulty_threshold, "評分應符合難度閾值");
}