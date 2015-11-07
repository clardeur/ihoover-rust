use std::path::Path;

mod parser;

fn main() {
    parser::parse_file(Path::new("E:\\Workspace\\ihoover-rust\\data\\input.txt"));
}
