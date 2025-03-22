use super::types::{Color, PieceType, Position};

#[derive(Debug, Clone)]
pub struct Piece {
    pub id: u32,  // 唯一編號
    pub kind: PieceType,
    pub color: Color,
    pub pos: Position,
}