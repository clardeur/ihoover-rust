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
        let radian = self.radian() + degree;
        match radian {
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
    x: u8,
    y: u8,
    orientation: Orientation
}

impl Position {
    pub fn new(x: u8, y: u8, orientation: Orientation) -> Position {
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
            Command::RotateLeft => self.rotate(-90),
            Command::RotateRight => self.rotate(90),
            Command::Forward => self.forward()
        }
    }

    fn rotate(&mut self, degree: i16) {
        self.position.orientation = self.position.orientation.rotate(degree).unwrap();
    }

    fn forward(&self) {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn new_hoover(orientation: Orientation) -> Hoover {
        let grid = Grid::new(0, 0);
        let position = Position::new(0, 0, orientation);
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
}
