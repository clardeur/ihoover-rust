use std::str::FromStr;

// Grid
#[derive(PartialEq, Debug)]
pub struct Grid {
    x: u8,
    y: u8
}

impl Grid {
    pub fn new(x: u8, y: u8) -> Grid {
        Grid {x: x, y: y}
    }

    pub fn is_valid(&self, x: i8, y: i8) -> bool {
        if x < 0 || x as u8 > self.x {
            return false;
        }
        if y < 0 || y as u8 > self.y {
            return false;
        }
        return true;
    }
}

// Orientation
#[derive(PartialEq, Debug)]
pub enum Orientation {
    North, East, South, West
}

impl Orientation {
    pub fn radian(&self) -> i16 {
        match *self {
            Orientation::North  => 0,
            Orientation::East   => 90,
            Orientation::South  => 180,
            Orientation::West   => 270
        }
    }

    pub fn rotate(&self, degree: i16) -> Result<Orientation, String> {
        match self.radian() + degree {
            0 | 360    => Ok(Orientation::North),
            90         => Ok(Orientation::East),
            180        => Ok(Orientation::South),
            270 | -90  => Ok(Orientation::West),
            _          => Err(format!("rotate of {}° is not supported", degree))
        }
    }
}

impl FromStr for Orientation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Orientation::North),
            "E" => Ok(Orientation::East),
            "S" => Ok(Orientation::South),
            "W" => Ok(Orientation::West),
            _   => Err(format!("failed to parse orientation <{}>", s))
        }
    }
}

// Position
#[derive(PartialEq, Debug)]
pub struct Position {
    x: i8,
    y: i8,
    orientation: Orientation
}

impl Position {
    pub fn new(x: i8, y: i8, orientation: Orientation) -> Position {
        Position {x: x, y: y, orientation: orientation}
    }
}

// Command
#[derive(PartialEq, Debug)]
pub enum Command {
    Forward, RotateLeft, RotateRight
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Command::Forward),
            "D" => Ok(Command::RotateRight),
            "G" => Ok(Command::RotateLeft),
            _   => Err(format!("failed to parse command <{}>", s))
        }
    }
}

// Hoover
#[derive(PartialEq, Debug)]
pub struct Hoover {
    grid: Grid,
    position: Position
}

impl Hoover {
    pub fn new(grid: Grid, position: Position) -> Hoover {
        Hoover {grid: grid, position: position}
    }

    pub fn execute (&mut self, cmd: &Command) {
        match *cmd {
            Command::RotateLeft  => self.rotate(-90),
            Command::RotateRight => self.rotate(90),
            Command::Forward     => self.forward()
        }
    }

    fn rotate(&mut self, degree: i16) {
        self.position.orientation = self.position.orientation.rotate(degree).unwrap();
    }

    fn forward(&mut self) {
        let (x, y) = match self.position.orientation {
            Orientation::North => (self.position.x, self.position.y + 1),
            Orientation::East  => (self.position.x + 1, self.position.y),
            Orientation::South => (self.position.x, self.position.y - 1),
            Orientation::West  => (self.position.x - 1, self.position.y)
        };

        if !self.grid.is_valid(x, y) {
            panic!(format!("Invalid position (x: {}, y: {}) on {:?}", x, y, self.grid));
        }

        self.position.x = x;
        self.position.y = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const X_GRID: u8 = 10;
    const Y_GRID: u8 = 10;

    const X_POS: i8 = 5;
    const Y_POS: i8 = 5;

    fn new_hoover(orientation: Orientation) -> Hoover {
        let grid = Grid::new(X_GRID, Y_GRID);
        let position = Position::new(X_POS, Y_POS, orientation);
        Hoover::new(grid, position)
    }

    #[test]
    fn should_rotate_left_the_hoover_from_the_north() {
        // Given
        let mut hoover = new_hoover(Orientation::North);
        // When
        hoover.execute(&Command::RotateLeft);
        // Then
        assert_eq!(Orientation::West, hoover.position.orientation);
    }

    #[test]
    fn should_rotate_left_the_hoover_from_the_east() {
        // Given
        let mut hoover = new_hoover(Orientation::East);
        // When
        hoover.execute(&Command::RotateLeft);
        // Then
        assert_eq!(Orientation::North, hoover.position.orientation);
    }

    #[test]
    fn should_rotate_left_the_hoover_from_the_south() {
        // Given
        let mut hoover = new_hoover(Orientation::South);
        // When
        hoover.execute(&Command::RotateLeft);
        // Then
        assert_eq!(Orientation::East, hoover.position.orientation);
    }

    #[test]
    fn should_rotate_left_the_hoover_from_the_west() {
        // Given
        let mut hoover = new_hoover(Orientation::West);
        // When
        hoover.execute(&Command::RotateLeft);
        // Then
        assert_eq!(Orientation::South, hoover.position.orientation);
    }

    #[test]
    fn should_rotate_right_the_hoover_from_the_north() {
        // Given
        let mut hoover = new_hoover(Orientation::North);
        // When
        hoover.execute(&Command::RotateRight);
        // Then
        assert_eq!(Orientation::East, hoover.position.orientation);
    }

    #[test]
    fn should_rotate_right_the_hoover_from_the_east() {
        // Given
        let mut hoover = new_hoover(Orientation::East);
        // When
        hoover.execute(&Command::RotateRight);
        // Then
        assert_eq!(Orientation::South, hoover.position.orientation);
    }

    #[test]
    fn should_rotate_right_the_hoover_from_the_south() {
        // Given
        let mut hoover = new_hoover(Orientation::South);
        // When
        hoover.execute(&Command::RotateRight);
        // Then
        assert_eq!(Orientation::West, hoover.position.orientation);
    }

    #[test]
    fn should_rotate_right_the_hoover_from_the_west() {
        // Given
        let mut hoover = new_hoover(Orientation::West);
        // When
        hoover.execute(&Command::RotateRight);
        // Then
        assert_eq!(Orientation::North, hoover.position.orientation);
    }

    #[test]
    fn should_forward_the_hoover_from_the_north() {
        // Given
        let mut hoover = new_hoover(Orientation::North);
        // When
        hoover.execute(&Command::Forward);
        // Then
        assert_eq!(X_POS, hoover.position.x);
        assert_eq!(Y_POS + 1, hoover.position.y);
    }

    #[test]
    fn should_forward_the_hoover_from_the_east() {
        // Given
        let mut hoover = new_hoover(Orientation::East);
        // When
        hoover.execute(&Command::Forward);
        // Then
        assert_eq!(X_POS + 1, hoover.position.x);
        assert_eq!(Y_POS, hoover.position.y);
    }

    #[test]
    fn should_forward_the_hoover_from_the_south() {
        // Given
        let mut hoover = new_hoover(Orientation::South);
        // When
        hoover.execute(&Command::Forward);
        // Then
        assert_eq!(X_POS, hoover.position.x);
        assert_eq!(Y_POS - 1, hoover.position.y);
    }

    #[test]
    fn should_forward_the_hoover_from_the_west() {
        // Given
        let mut hoover = new_hoover(Orientation::West);
        // When
        hoover.execute(&Command::Forward);
        // Then
        assert_eq!(X_POS - 1, hoover.position.x);
        assert_eq!(Y_POS, hoover.position.y);
    }

    #[test]
    #[should_panic(expected = "Invalid position (x: 5, y: 11) on Grid { x: 10, y: 10 }")]
    fn should_not_forward_when_the_new_position_is_oustide_of_the_grid() {
        // Given
        let mut hoover = new_hoover(Orientation::North);
        // When / Then
        for _ in 0..Y_GRID {
            hoover.execute(&Command::Forward);
        }
    }
}
