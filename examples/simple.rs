extern crate termo;
use std::io;

fn main() {
    let stdout = io::stdout();
    let term = termo::Terminal::new(&stdout);
    let foo = term.text().bold().pos(2, 2).text("blah");
}
