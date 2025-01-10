use itertools::Itertools;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use std::fs;
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
    Text(Vec<Span<()>>), // TODO: should be Text(Span<()>)
    NestedList(List),
}

impl Content {
    // TODO: should be moved to Item, Contnent::Text's vec always has one element
    fn merge_texts(&mut self) {
        let mut new_texts = vec![];

        match self {
            Content::Text(texts) => {
                let mut current_range = 0..0;

                if let Some(a) = texts.first() {
                    current_range = a.range.clone();
                }

                println!("will process {texts:?}");

                for (pos, text) in texts.into_iter().with_position() {
                    let is_last =
                        matches!(pos, itertools::Position::Last | itertools::Position::Only);

                    println!(
                        "comparing text.range: {:?} and current_range: {:?}",
                        text.range, current_range
                    );

                    let should_merge = text.range.start == current_range.end;

                    if should_merge {
                        println!("merging {:?} and {:?}", text.range, current_range);
                        current_range.end = text.range.end;
                    }

                    if !should_merge || is_last {
                        new_texts.push(Span::new((), current_range));
                        current_range = text.range.clone();
                    }
                }
            }
            Content::NestedList(list) => {
                list.merge_texts();
                return;
            }
        }

        *self = Content::Text(new_texts);
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

#[derive(Debug)]
struct List {
    items: Vec<Span<Item>>,
}

impl List {
    fn new() -> Self {
        List { items: vec![] }
    }

    fn merge_texts(&mut self) {
        for item in self.items.iter_mut() {
            for content in item.element.contents.iter_mut() {
                content.merge_texts();
            }
        }
    }
}

fn parse(input: &str) -> Vec<List> {
    let mut top_level_lists = vec![];

    let mut list_stack: Vec<List> = vec![];

    let mut item_stack: Vec<Item> = vec![];

    let parser = Parser::new_ext(input, Options::ENABLE_TASKLISTS).into_offset_iter();

    for (event, range) in parser {
        match event {
            Event::Start(Tag::List(_)) => {
                println!("Found list start");

                list_stack.push(List::new());
            }
            Event::End(TagEnd::List(_)) => {
                println!("Found list end");

                let current_list = list_stack.pop().unwrap();

                if let Some(current_item) = item_stack.last_mut() {
                    current_item
                        .contents
                        .push(Content::NestedList(current_list));
                } else {
                    top_level_lists.push(current_list);
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
            Event::Text(text) => {
                if let Some(current_item) = item_stack.last_mut() {
                    println!("Found text inside item: {}", text);
                    current_item
                        .contents
                        .push(Content::Text(vec![Span::new((), range)]));
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
        list.merge_texts();
    }

    top_level_lists
}

fn main() {
    let markdown = fs::read_to_string("TODO.md").unwrap();

    let x = parse(&markdown);

    println!("");

    println!("{x:#?}");
}
