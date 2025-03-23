use std::collections::HashMap;

use super::piece::Piece;
use super::types::{Position, Tile, PieceType::*};

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>, // 棋盤格子狀態：Empty / Blocked
    pub pieces: HashMap<u32, Piece>, // 棋子編號 → 棋子
    pub next_id: u32, // 棋子 ID 自動遞增用
}

impl Board {
    /// 建立新的棋盤，預設所有格子都是空的
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::Empty; height]; width];
        Board {
            width,
            height,
            tiles,
            pieces: HashMap::new(),
            next_id: 1,
        }
    }

    /// 標記禁區（破洞）
    pub fn set_blocked(&mut self, pos: Position) {
        if self.in_bounds(pos) {
            self.tiles[pos.0][pos.1] = Tile::Blocked;
        }
    }

    /// 將所有格子重設為空白（不影響棋子）
    pub fn clear_tiles(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.tiles[x][y] = Tile::Empty;
            }
        }
    }

    /// 新增一個棋子到指定位置，並自動分配 ID
    pub fn add_piece(&mut self, mut piece: Piece) -> u32 {
        if !self.is_tile_free(piece.pos) {
            return u32::MAX; // 表示錯誤或無法放置
        }
        piece.id = self.next_id;
        self.pieces.insert(self.next_id, piece);
        self.next_id += 1;
        self.next_id - 1
    }

    /// 移除指定棋子 ID
    pub fn remove_piece(&mut self, id: u32) {
        self.pieces.remove(&id);
    }

    /// 取得某格子的棋子（如果有）
    pub fn get_piece_at(&self, pos: Position) -> Option<&Piece> {
        self.pieces.values().find(|p| p.pos == pos)
    }

    /// 判斷是否在邊界內
    pub fn in_bounds(&self, pos: Position) -> bool {
        pos.0 < self.width && pos.1 < self.height
    }

    /// 該格是否可以進入（無論有無棋子）
    pub fn is_tile_accessible(&self, pos: Position) -> bool {
        self.in_bounds(pos) && self.tiles[pos.0][pos.1] != Tile::Blocked
    }

    /// 該格是否為空（可進入 & 無棋子）
    pub fn is_tile_free(&self, pos: Position) -> bool {
        if !self.in_bounds(pos) {
            return false;
        }
        match self.tiles[pos.0][pos.1] {
            Tile::Blocked => false,
            _ => self.get_piece_at(pos).is_none() && self.is_tile_accessible(pos),
        }
    }    

    /// 清除所有棋子
    pub fn clear_pieces(&mut self) {
        self.pieces.clear();
        self.next_id = 1;
    }

    /// 將棋盤狀態印出至終端機（除錯用）
    pub fn print_board(&self) {
        println!("\n棋盤狀態 (左上為 (0,0)):");
        for y in 0..self.height {
            for x in 0..self.width {
                let symbol = match self.tiles[x][y] {
                    Tile::Blocked => '#',
                    Tile::Empty => {
                        match self.get_piece_at((x, y)) {
                            Some(p) => piece_symbol(p),
                            None => '.',
                        }
                    }
                };
                print!("{} ", symbol);
            }
            println!();
        }
        println!();
    }
}

fn piece_symbol(piece: &Piece) -> char {
    let base = match piece.kind {
        King => 'K', Queen => 'Q', Rook => 'R',
        Bishop => 'B', Knight => 'N', Pawn => 'P',
        Custom(_) => '?',
    };
    match piece.color {
        super::types::Color::White => base,
        super::types::Color::Black => base.to_ascii_lowercase(),
    }
}
