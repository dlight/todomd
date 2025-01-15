use itertools::Itertools;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use std::env;
use std::fs;
use std::mem;
use std::ops::Range;

#[derive(Debug)]
struct Span<I> {
    element: I,
    range: Range<usize>,
}

impl<I> Span<I> {
    fn new(element: I, range: Range<usize>) -> Self {
        Span { element, range }
    }
}

#[derive(Debug)]
enum Content {
    RawMarkdown(Span<()>),
    NestedList(Span<List>),
}

macro_rules! print_helper {
    ($ident: expr, $text: expr) => {
        println!("{:ident$}{:?}", "", $text, ident = $ident);
    };
    ("-", $ident: expr, $text: expr) => {
        println!("{:ident$} - {:?}", "", $text, ident = $ident);
    };
}

impl Content {
    fn print(&self, source: &str, ident: usize, verbose: bool) {
        match self {
            Content::RawMarkdown(span) => {
                print_helper!(ident, &source[span.range.clone()]);
            }
            Content::NestedList(list) => {
                list.print(source, ident, verbose);
            }
        }
    }
}

#[derive(Debug)]
struct Item {
    checkbox: Option<Span<bool>>,
    contents: Vec<Content>,
}

impl Item {
    fn new() -> Self {
        Item {
            checkbox: None,
            contents: vec![],
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
        for content in &self.element.contents {
            content.print(source, ident + 2, verbose);
        }
        if verbose {
            println!("(item source: {:?})", &source[self.range.clone()]);
            println!();
        }
    }
}

impl Item {
    fn merge_texts(&mut self) {
        let contents = mem::take(&mut self.contents);
        let contents = contents
            .into_iter()
            .coalesce(|x, y| match (x, y) {
                (Content::RawMarkdown(x), Content::RawMarkdown(y)) => {
                    let should_merge = x.range.end == y.range.start;

                    if should_merge {
                        Ok(Content::RawMarkdown(Span::new(
                            (),
                            x.range.start..y.range.end,
                        )))
                    } else {
                        Err((Content::RawMarkdown(x), Content::RawMarkdown(y)))
                    }
                }
                (Content::NestedList(mut list1), Content::NestedList(mut list2)) => {
                    list1.element.merge_texts();
                    list2.element.merge_texts();

                    Err((Content::NestedList(list1), Content::NestedList(list2)))
                }
                (Content::NestedList(mut list), y) => {
                    list.element.merge_texts();

                    Err((Content::NestedList(list), y))
                }
                (x, Content::NestedList(mut list)) => {
                    list.element.merge_texts();

                    Err((x, Content::NestedList(list)))
                }
            })
            .collect::<Vec<_>>();
        self.contents = contents;
    }
}

#[derive(Debug)]
struct List {
    items: Vec<Span<Item>>,
}

impl List {
    fn new() -> Self {
        List { items: vec![] }
    }

    fn merge_texts(&mut self) {
        for Span { element, .. } in &mut self.items {
            element.merge_texts();
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

fn parse(input: &str) -> Vec<Span<List>> {
    let mut top_level_lists = vec![];

    let mut list_stack: Vec<List> = vec![];

    let mut item_stack: Vec<Item> = vec![];

    let parser = Parser::new_ext(input, Options::ENABLE_TASKLISTS).into_offset_iter();

    for (event, range) in parser {
        match event {
            Event::Start(Tag::List(None)) => {
                println!("Found unordered list start");

                list_stack.push(List::new());
            }
            Event::End(TagEnd::List(false)) => {
                println!("Found unordered list end");

                let current_list = list_stack.pop().unwrap();

                if let Some(current_item) = item_stack.last_mut() {
                    current_item
                        .contents
                        .push(Content::NestedList(Span::new(current_list, range)));
                } else {
                    top_level_lists.push(Span::new(current_list, range));
                }
            }
            Event::Start(Tag::Item) => {
                println!("Found item start");

                item_stack.push(Item::new());
            }
            Event::End(TagEnd::Item) => {
                println!("Found item end");

                let current_list = list_stack.last_mut().unwrap();
                let current_item = item_stack.pop().unwrap();
                current_list.items.push(Span::new(current_item, range));
            }
            Event::End(_) | Event::Text(_) => {
                if let Some(current_item) = item_stack.last_mut() {
                    println!("Found text or something else inside item");
                    current_item
                        .contents
                        .push(Content::RawMarkdown(Span::new((), range)));
                }
            }
            Event::TaskListMarker(marked) => {
                println!("Found task list marker: {marked}");

                let current_item = item_stack.last_mut().unwrap();
                current_item.checkbox = Some(Span::new(marked, range));
            }
            _ => (),
        }
    }

    for list in top_level_lists.iter_mut() {
        list.element.merge_texts();
    }

    top_level_lists
}

fn main() {
    let file = env::args().nth(1).unwrap_or(String::from("TODO.md"));
    let markdown = fs::read_to_string(file).unwrap();

    let x = parse(&markdown);

    println!("");

    for list in x {
        list.print(&markdown, 0, false);
        println!();
        println!();
    }

    //println!("{x:#?}");
}
