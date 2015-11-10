use std::path::Path;

mod model;
mod parser;

fn main() {
    let parsing_result = parser::parse_file(Path::new("E:\\Workspace\\ihoover-rust\\data\\input.txt"));
    println!("{:?}", parsing_result);
}
