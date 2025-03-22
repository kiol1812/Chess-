use rand::Rng;
use super::types::Position;

pub fn random_position(width: usize, height: usize) -> Position {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..width), rng.gen_range(0..height))
}