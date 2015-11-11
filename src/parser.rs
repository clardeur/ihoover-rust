use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use model::Grid;
use model::Position;
use model::Command;

#[derive(PartialEq, Debug)]
pub struct ParsingResult {
    pub grid: Grid,
    pub position: Position,
    pub commands: Vec<Command>
}

pub fn parse_file(path: &Path) -> ParsingResult {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|result| result.expect("Error reading lines"));
    let grid = parse_grid(&lines.next().expect("No grid input line"));
    let position = parse_position(&lines.next().expect("No position input line"));
    let commands = parse_commands(&lines.next().expect("No commands input line"));
    ParsingResult {grid: grid, position: position, commands: commands}
}

fn parse_grid(line: &str) -> Grid {
    let mut iter = line.split_whitespace();

    let x = iter.next()
                .map(|n| n.parse().expect("Invalid (x) grid input"))
                .expect("Missing (x) grid input");

    let y = iter.next()
                .map(|n| n.parse().expect("Invalid (y) grid input"))
                .expect("Missing (y) grid input");

    Grid::new(x, y)
}

fn parse_position(line: &str) -> Position {
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

    Position::new(x, y, orientation)
}

fn parse_commands(line: &str) -> Vec<Command> {
    line.chars()
        .map(|c| c.to_string().parse().expect("Invalid command input"))
        .collect()
}

#[cfg(test)]
mod tests {
    use parser;
    use model::*;

    #[test]
    fn should_parse_a_grid() {
        assert_eq!(Grid::new(10 ,10), parser::parse_grid("10 10"));
        assert_eq!(Grid::new(0, 5), parser::parse_grid("0 5"));
    }

    #[test]
    #[should_panic(expected = "Missing (x) grid input")]
    fn should_failed_to_parse_a_grid_when_x_is_missing() {
        parser::parse_grid("");
    }

    #[test]
    #[should_panic(expected = "Missing (y) grid input")]
    fn should_failed_to_parse_a_grid_when_y_is_missing() {
        parser::parse_grid("10");
    }

    #[test]
    #[should_panic(expected = "Invalid (x) grid input")]
    fn should_failed_to_parse_a_grid_when_x_is_not_a_number() {
        parser::parse_grid("x 10");
    }

    #[test]
    #[should_panic(expected = "Invalid (y) grid input")]
    fn should_failed_to_parse_a_grid_when_y_is_not_a_number() {
        parser::parse_grid("10 y");
    }

    #[test]
    fn should_parse_a_position() {
        assert_eq!(Position::new(5, 5, Orientation::Nord), parser::parse_position("5 5 N"));
        assert_eq!(Position::new(8, 1, Orientation::West), parser::parse_position("8 1 W"));
    }

    #[test]
    #[should_panic(expected = "Missing (x) position input")]
    fn should_failed_to_parse_a_position_when_x_is_missing() {
        parser::parse_position("");
    }

    #[test]
    #[should_panic(expected = "Missing (y) position input")]
    fn should_failed_to_parse_a_position_when_y_is_missing() {
        parser::parse_position("10");
    }

    #[test]
    #[should_panic(expected = "Missing orientation input")]
    fn should_failed_to_parse_a_position_when_orientation_is_missing() {
        parser::parse_position("10 10");
    }

    #[test]
    #[should_panic(expected = "Invalid orientation input: \"failed to parse orientation <X>\"")]
    fn should_failed_to_parse_a_position_when_orientation_is_invalid() {
        parser::parse_position("10 10 X");
    }

    #[test]
    fn should_parse_commands() {
        assert_eq!(vec![Command::Forward, Command::RotateLeft, Command::RotateRight], parser::parse_commands("AGD"));
        assert_eq!(vec![Command::Forward, Command::Forward], parser::parse_commands("AA"));
    }

    #[test]
    #[should_panic(expected = "Invalid command input: \"failed to parse command <X>\"")]
    fn should_failed_to_parse_commands_when_a_command_is_invalid() {
        parser::parse_commands("X");
    }

}
