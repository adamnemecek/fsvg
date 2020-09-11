mod command;
pub use command::*;

mod definitions;
pub use definitions::*;

mod generator;
pub use generator::*;


mod parser;
pub use parser::*;



fn main() {
    let path = std::path::PathBuf::from("/Users/adamnemecek/Code/ngrid/main/vendor/ngrid10deps/nsvg/test.svg");
    let svg_parser = SVGParser::new();
    println!("Hello, world!");
}
