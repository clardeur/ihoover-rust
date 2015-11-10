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
    Nord, Est, Sud, West
}

impl FromStr for Orientation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Orientation::Nord),
            "E" => Ok(Orientation::Est),
            "S" => Ok(Orientation::Sud),
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
