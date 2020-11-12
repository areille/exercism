#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None;
        } else {
            return Some(Self(rank, file));
        }
    }

    pub fn is_same_row(&self, other: &ChessPosition) -> bool {
        self.0 == other.0
    }

    pub fn is_same_column(&self, other: &ChessPosition) -> bool {
        self.1 == other.1
    }

    pub fn is_diagonal(&self, other: &ChessPosition) -> bool {
        let r = (other.0 - self.0) / (other.1 - self.1);
        r == -1 || r == 1
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.is_same_row(&other.0)
            || self.0.is_same_column(&other.0)
            || self.0.is_diagonal(&other.0)
    }
}
