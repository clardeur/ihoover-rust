use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Grid {
    x: u8,
    y: u8
}

#[derive(PartialEq, Debug)]
pub struct Position {
    x: u8,
    y: u8,
    orientation: Orientation
}

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

pub fn parse_file(path: &Path) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|result| result.expect("Error reading lines"));
    let grid = parse_grid(&lines.next().expect("No grid input line"));
    let pos = parse_position(&lines.next().expect("No position input line"));
    let commands = parse_commands(&lines.next().expect("No commands input line"));
    println!("{:?}", grid);
    println!("{:?}", pos);
    println!("{:?}", commands);
}

pub fn parse_grid(line: &str) -> Grid {
    let mut iter = line.split_whitespace();

    let x = iter.next()
                .map(|n| n.parse().expect("Invalid (x) grid input"))
                .expect("Missing (x) grid input");

    let y = iter.next()
                .map(|n| n.parse().expect("Invalid (y) grid input"))
                .expect("Missing (y) grid input");

    return Grid {x:x, y:y};
}

pub fn parse_position(line: &str) -> Position {
    let mut iter = line.split_whitespace();

    let x = iter.next()
                .map(|n| n.parse().expect("Invalid (x) position input"))
                .expect("Missing (x) position input");

    let y = iter.next()
                .map(|n| n.parse().expect("Invalid (y) position input"))
                .expect("Missing (y) position input");

    let orientation = iter.next()
                          .map(|o| o.parse().expect("Invalid orientation input"))
                          .expect("Missing orientation input");

    return Position {x:x, y:y, orientation: orientation};
}

pub fn parse_commands(line: &str) -> Vec<Command> {
    return line.chars()
               .map(|c| c.to_string().parse().expect("Invalid command input"))
               .collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_a_grid() {
        assert_eq!(Grid {x:10, y:10}, parse_grid("10 10"));
        assert_eq!(Grid {x:0, y:5}, parse_grid("0 5"));
    }

    #[test]
    #[should_panic(expected = "Missing (x) grid input")]
    fn should_failed_to_parse_a_grid_when_x_is_missing() {
        parse_grid("");
    }

    #[test]
    #[should_panic(expected = "Missing (y) grid input")]
    fn should_failed_to_parse_a_grid_when_y_is_missing() {
        parse_grid("10");
    }

    #[test]
    #[should_panic(expected = "Invalid (x) grid input")]
    fn should_failed_to_parse_a_grid_when_x_is_not_a_number() {
        parse_grid("x 10");
    }

    #[test]
    #[should_panic(expected = "Invalid (y) grid input")]
    fn should_failed_to_parse_a_grid_when_y_is_not_a_number() {
        parse_grid("10 y");
    }

    #[test]
    fn should_parse_a_position() {
        assert_eq!(Position {x:5, y:5, orientation: Orientation::Nord}, parse_position("5 5 N"));
        assert_eq!(Position {x:8, y:1, orientation: Orientation::West}, parse_position("8 1 W"));
    }

    #[test]
    #[should_panic(expected = "Missing (x) position input")]
    fn should_failed_to_parse_a_position_when_x_is_missing() {
        parse_position("");
    }

    #[test]
    #[should_panic(expected = "Missing (y) position input")]
    fn should_failed_to_parse_a_position_when_y_is_missing() {
        parse_position("10");
    }

    #[test]
    #[should_panic(expected = "Missing orientation input")]
    fn should_failed_to_parse_a_position_when_orientation_is_missing() {
        parse_position("10 10");
    }

    #[test]
    #[should_panic(expected = "Invalid orientation input: \"failed to parse orientation <X>\"")]
    fn should_failed_to_parse_a_position_when_orientation_is_invalid() {
        parse_position("10 10 X");
    }

    #[test]
    fn should_parse_commands() {
        assert_eq!(vec![Command::Forward, Command::RotateLeft, Command::RotateRight], parse_commands("AGD"));
        assert_eq!(vec![Command::Forward, Command::Forward], parse_commands("AA"));
    }

    #[test]
    #[should_panic(expected = "Invalid command input: \"failed to parse command <X>\"")]
    fn should_failed_to_parse_commands_when_a_command_is_invalid() {
        parse_commands("X");
    }

}
