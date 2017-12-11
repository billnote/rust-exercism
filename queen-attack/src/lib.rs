pub struct ChessPosition {
    x: isize,
    y: isize,
}

impl ChessPosition {
    const LEGAL_POSTION: (isize, isize) = (0, 7);

    pub fn new(x: isize, y: isize) -> Result<Self, ()> {
        let (min, max) = if x < y { (x, y) } else { (y, x) };

        match (min, max) {
            (a, b) if a < ChessPosition::LEGAL_POSTION.0 || b > ChessPosition::LEGAL_POSTION.1 => {
                Err(())
            }
            _ => Ok(ChessPosition { x: min, y: max }),
        }
    }

    pub fn in_same_line(&self, other: &Self) -> bool {
        self.x == other.x || self.y == other.y ||
            ((self.x - other.x).abs() == (self.y - other.y).abs())
    }
}

pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Self) -> bool {
        self.position.in_same_line(&other.position)
    }
}
