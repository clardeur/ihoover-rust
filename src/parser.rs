use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fmt;

struct Grid {
    x: u8,
    y: u8
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid({}, {})", self.x, self.y)
    }
}

pub fn parse_file(path: &Path) {
    let file = File::open(path).unwrap();

    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(|result| result.expect("Error reading lines"));

    let grid = parse_grid(lines.next().expect("No grid input line"));

    println!("{}", grid);
}

fn parse_grid(line: String) -> Grid {
    let mut iter = line.split_whitespace();

    let x = iter.next()
                .map(|n| n.parse().expect("Invalid (x) grid input"))
                .expect("Missing (x) grid input");

    let y = iter.next()
                .map(|n| n.parse().expect("Invalid (x) grid input"))
                .expect("Missing (y) grid input");

    return Grid {x:x, y:y};
}
