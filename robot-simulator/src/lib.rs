// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let dire = match self.direction {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        };

        Robot {
            x: self.x,
            y: self.y,
            direction: dire,
        }
    }

    pub fn turn_left(self) -> Self {
        let dire = match self.direction {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        };

        Robot {
            x: self.x,
            y: self.y,
            direction: dire,
        }
    }

    pub fn advance(self) -> Self {
        let mut x = self.x;
        let mut y = self.y;
        match self.direction {
            Direction::East => {
                x += 1;
            }
            Direction::South => {
                y -= 1;
            }
            Direction::West => {
                x -= 1;
            }
            Direction::North => {
                y += 1;
            }
        }

        Robot {
            x: x,
            y: y,
            direction: self.direction,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, c| match c {
            'A' => acc.advance(),
            'R' => acc.turn_right(),
            'L' => acc.turn_left(),
            _ => acc,
        })
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x.clone(), self.y.clone())
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
