extern crate itertools;

use itertools::Itertools;
use model::Hoover;
use std::path::Path;

mod parser;
mod model;

fn main() {
    let input = parser::parse_file(Path::new("E:\\Workspace\\ihoover-rust\\data\\input.txt"));
    let hoover = Hoover::new(input.grid, input.position);
    input.commands.iter().foreach(|cmd| hoover.execute(cmd));
}
