use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(PartialEq, Debug)]
pub struct Grid {
    x: u8,
    y: u8
}

pub fn parse_file(path: &Path) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|result| result.expect("Error reading lines"));
    let grid = parse_grid(&lines.next().expect("No grid input line"));
    println!("{:?}", grid);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_a_grid() {
        assert_eq!(Grid {x:10, y:10}, parse_grid("10 10"));
    }

    #[test]
    #[should_panic(expected = "Missing (x) grid input")]
    fn should_failed_to_parse_when_x_is_missing() {
        parse_grid("");
    }


    #[test]
    #[should_panic(expected = "Missing (y) grid input")]
    fn should_failed_to_parse_when_y_is_missing() {
        parse_grid("10 ");
    }

    #[test]
    #[should_panic(expected = "Invalid (x) grid input")]
    fn should_failed_to_parse_when_x_is_not_a_number() {
        parse_grid("x 10");
    }

    #[test]
    #[should_panic(expected = "Invalid (y) grid input")]
    fn should_failed_to_parse_when_y_is_not_a_number() {
        parse_grid("10 y");
    }
}
