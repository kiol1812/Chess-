# Chess?
```plain
chess_endgame_generator/
│
├── gui/                         # 💡 GUI 視覺呈現（Slint 語法）
│   └── main_window.slint        #   主視窗 GUI 定義
│
├── assets/                      # 🎨 靜態資源（圖片、棋子素材）
│
├── src/                         # 🧠 Rust 核心程式碼
│   ├── lib.rs                   #   公共函式 / 模組匯出點（共用邏輯）
│   └── bin/
│       ├── engine/              #   棋局與 AI 模組
│       │   ├── board.rs         #     棋盤資料結構與禁區支援
│       │   ├── evaluator.rs     #     棋局評估器（可接 Stockfish 或靜態評估）
│       │   ├── generator.rs     #     隨機生成合法殘局
│       │   └── ai.rs            #     Minimax / Alpha-Beta 實作
│       └── main.rs              #   主程式進入點，呼叫 GUI + 運算邏輯
│
└── Cargo.toml                   # 📦 套件依賴與設定
```