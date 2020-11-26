#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        let new_direction = match self.direction {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        };

        self.direction = new_direction;
        self
    }

    pub fn turn_left(mut self) -> Self {
        let new_direction = match self.direction {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        };

        self.direction = new_direction;
        self
    }

    pub fn advance(mut self) -> Self {
        let new_position = match self.direction {
            Direction::East => (self.position.0 + 1, self.position.1),
            Direction::North => (self.position.0, self.position.1 + 1),
            Direction::West => (self.position.0 - 1, self.position.1),
            Direction::South => (self.position.0, self.position.1 - 1),
        };

        self.position = new_position;
        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, x| {
            match x {
                'L' => acc.turn_left(),
                'R' => acc.turn_right(),
                'A' => acc.advance(),
                _ => acc,
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
