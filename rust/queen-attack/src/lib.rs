#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(i32, i32);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition(rank, file)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position.0, position.1)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        dx == 0 || dy == 0 || dx.abs() == dy.abs()
    }
}
