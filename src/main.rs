#![allow(unused)]

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use std::env;
use std::fs;

mod rangeset;

use rangeset::{Range, RangeSet};

macro_rules! print_helper {
    ($ident: expr, $text: expr) => {
        println!("{:ident$}{:?}", "", $text, ident = $ident);
    };
    ("-", $ident: expr, $text: expr) => {
        println!("{:ident$} - {:?}", "", $text, ident = $ident);
    };
}

#[derive(Debug)]
struct Span<I> {
    element: I,
    range: Range,
}

#[derive(Debug, Default)]
struct Item {
    checkbox: Option<Span<bool>>,
    contents: RangeSet,
    nested_list: Option<Span<List>>,
}

impl Item {
    fn span(self, range: Range) -> Span<Self> {
        Span {
            element: self,
            range,
        }
    }
}

impl Span<Item> {
    fn print(&self, source: &str, ident: usize, verbose: bool) {
        print_helper!(
            "-",
            ident,
            self.element.checkbox.as_ref().map(|x| x.element)
        );
        for range in &self.element.contents {
            print_helper!(ident + 2, &source[range.clone()]);
        }
        if let Some(nested) = &self.element.nested_list {
            nested.print(source, ident + 2, verbose);
        }
        if verbose {
            println!("(item source: {:?})", &source[self.range.clone()]);
            println!();
        }
    }
}

#[derive(Debug, Default)]
struct List {
    items: Vec<Span<Item>>,
}

impl List {
    fn span(self, range: Range) -> Span<Self> {
        Span {
            element: self,
            range,
        }
    }
}

impl Span<List> {
    fn print(&self, source: &str, ident: usize, verbose: bool) {
        for item in &self.element.items {
            item.print(source, ident, verbose);
        }

        if verbose {
            println!("(list source: {:?})", &source[self.range.clone()]);
            println!();
        }
    }
}

#[derive(Debug, Default)]
struct Board {
    lists: Vec<Span<List>>,
}

fn parse(input: &str) -> Board {
    let mut board = Board::default();

    let mut list_stack: Vec<List> = vec![];

    let mut item_stack: Vec<Item> = vec![];

    let parser = Parser::new_ext(input, Options::ENABLE_TASKLISTS).into_offset_iter();

    for (event, range) in parser {
        match event {
            Event::Start(Tag::List(None)) => {
                println!("Found unordered list start\n");

                list_stack.push(List::default());
            }
            Event::End(TagEnd::List(false)) => {
                println!("Found unordered list end\n");

                let current_list = list_stack.pop().unwrap();

                if let Some(current_item) = item_stack.last_mut() {
                    if current_item.nested_list.is_some() {
                        panic!("a list item should not have two sub lists");
                    }
                    current_item.nested_list = Some(current_list.span(range.clone()));
                } else {
                    board.lists.push(current_list.span(range.clone()));
                }
            }
            Event::Start(Tag::Item) => {
                println!("Found item start\n");

                item_stack.push(Item::default());
            }
            Event::End(TagEnd::Item) => {
                println!("Found item end\n");

                let current_list = list_stack.last_mut().unwrap();
                let current_item = item_stack.pop().unwrap();
                current_list.items.push(current_item.span(range.clone()));
            }
            Event::TaskListMarker(marked) => {
                println!("Found task list marker: {marked}\n");

                let current_item = item_stack.last_mut().unwrap();
                current_item.checkbox = Some(Span {
                    element: marked,
                    range: range.clone(),
                });
            }
            _ => {
                if let Some(current_item) = item_stack.last_mut() {
                    println!("Found something else inside item\n");
                    current_item.contents.insert_range(range.clone());
                }
            }
        }

        println!("{:?}\n", &input[range.clone()]);
    }

    board
}

fn dothing() {
    let file = env::args().nth(1).unwrap_or(String::from("TODO.md"));
    let markdown = fs::read_to_string(file).unwrap();

    let board = parse(&markdown);

    println!("");

    for list in &board.lists {
        list.print(&markdown, 0, true);
        println!();
        println!();
    }
}

fn main() {
    dothing();
}
