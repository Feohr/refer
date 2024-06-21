use std::io::Stdout;

use tui::{backend::*, layout::*, style::*, terminal::*, widgets::*};

use crate::ui::*;

pub enum Inner {
    Root,
    Node(Box<dyn Visit>),
}

pub struct Node {
    pub inner: Inner,
    pub next: Vec<Node>,
}

impl Node {
    pub fn new<V: 'static + Visit>(inner: V) -> Self {
        Node {
            inner: Inner::Node(Box::new(inner)),
            next: vec![],
        }
    }
}

pub struct FileList<'a> {
    pub file_list: Item<List<'a>>,
}

impl<'a> FileList<'a> {
    pub fn new(filename: Vec<String>, size: Rect) -> Self {
        FileList {
            file_list: Item::new(
                List::<'a>::new(
                    filename
                        .into_iter()
                        .map(|i| ListItem::new(i))
                        .collect::<Vec<ListItem>>(),
                )
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(IDLE)
                        .style(Style::default()),
                )
                .style(basic_style())
                .highlight_symbol(">"),
                size,
            ),
        }
    }
}

pub struct Item<W: Widget + Clone> {
    size: Rect,
    widget: W,
}

impl<W: Widget + Clone> Item<W> {
    pub fn new(widget: W, size: Rect) -> Self {
        Item { size, widget }
    }
}

pub trait Visit {
    fn visit(&self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>);
}

impl<W: Widget + Clone> Visit for Item<W> {
    fn visit(&self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>) {
        frame.render_widget(self.widget.clone(), self.size);
    }
}

impl<'a> Visit for FileList<'a> {
    fn visit(&self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>) {
        self.file_list.visit(frame);
    }
}

pub struct FootList<'a>(pub Vec<Item<Paragraph<'a>>>);

impl<'a> Visit for FootList<'a> {
    fn visit(&self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>) {
        for footer in self.0.iter() {
            footer.visit(frame);
        }
    }
}

impl Visit for Node {
    fn visit(&self, frame: &mut Frame<'_, CrosstermBackend<Stdout>>) {
        match &self.inner {
            Inner::Node(ref inner) => inner.visit(frame),
            _ => {},
        }

        for children in self.next.iter() {
            children.visit(frame);
        }
    }
}
