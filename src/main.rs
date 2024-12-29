use markdown_ast::{ast_to_markdown, markdown_to_ast};
use std::fs;

fn main() {
    let markdown = fs::read_to_string("TODO.md").unwrap();

    let ast = markdown_to_ast(&markdown);
    println!("{:#?}", ast);

    let mark2 = ast_to_markdown(&ast);

    println!("{}", mark2);
}
