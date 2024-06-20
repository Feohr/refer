use std::io::Stdout;

use tui::{backend::*, layout::*, style::*, terminal::*, widgets::*};

use crate::ui::*;

pub struct Main<'a> {
    pub file_list: FileList<'a>,
    pub file_buff: Node<Block<'a>>,
    pub footers: Vec<Node<Paragraph<'a>>>,
}

pub struct FileList<'a> {
    pub _files: Vec<String>,
    pub file_list: Node<List<'a>>,
}

impl<'a> FileList<'a> {
    pub fn new(filename: Vec<String>, size: Rect) -> Self {
        FileList {
            _files: filename.clone(),
            file_list: Node::new(
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

pub struct Node<W: Widget + Clone> {
    size: Rect,
    widget: W,
}

impl<W: Widget + Clone> Node<W> {
    pub fn new(widget: W, size: Rect) -> Self {
        Node { size, widget }
    }
}

pub trait Visit<'a> {
    fn visit(&self, frame: &mut Frame<'a, CrosstermBackend<Stdout>>);
}

impl<'a> Visit<'a> for Main<'a> {
    fn visit(&self, frame: &mut Frame<'a, CrosstermBackend<Stdout>>) {
        self.file_list.visit(frame);
        self.file_buff.visit(frame);
        for footer in self.footers.iter() {
            footer.visit(frame);
        }
    }
}

impl<'a, W: Widget + Clone> Visit<'a> for Node<W> {
    fn visit(&self, frame: &mut Frame<'a, CrosstermBackend<Stdout>>) {
        frame.render_widget(self.widget.clone(), self.size);
    }
}

impl<'a> Visit<'a> for FileList<'a> {
    fn visit(&self, frame: &mut Frame<'a, CrosstermBackend<Stdout>>) {
        self.file_list.visit(frame);
    }
}
