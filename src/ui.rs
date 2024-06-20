use std::io::Stdout;

use crate::widget::*;

use tui::{backend::CrosstermBackend, layout::*, style::*, widgets::*, Frame};

pub const IDLE: Style = Style {
    fg: Some(Color::Rgb(105, 105, 105)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const HOVR: Style = Style {
    fg: Some(Color::White),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

fn footer<'a>() -> Vec<Paragraph<'a>> {
    vec![
        Paragraph::new("`ctrl + q`/`ctrl + c` - quit"),
        Paragraph::new("`ctrl + n` - add a new file"),
        Paragraph::new("`ctrl + up` - move cursor to top"),
        Paragraph::new("`ctrl + down` - move cursor to bottom"),
        Paragraph::new("`ctrl + t` - toggle tail mode"),
    ]
}

pub fn basic_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Black)
}

pub fn get_ui_tree(size: Rect, filename: Vec<String>) -> Node {
    let footer = footer();

    let vflex = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(98), Constraint::Percentage(2)])
        .split(size);
    let hflex = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(vflex[0]);
    let fflex = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(vflex[1]);

    let flist = Node::new(FileList::new(filename, hflex[0]));

    let fwin = Node::new(Item::new(
        Block::default()
            .borders(Borders::ALL)
            .border_style(IDLE)
            .style(Style::default()),
        hflex[1],
    ));

    let footer = footer
        .into_iter()
        .zip(fflex)
        .map(|(widget, size)| {
            Item::new(
                widget
                    .block(
                        Block::default()
                            .borders(Borders::LEFT)
                            .border_style(HOVR)
                            .style(Style::default()),
                    )
                    .style(basic_style())
                    .wrap(Wrap { trim: false })
                    .alignment(Alignment::Left),
                size,
            )
        })
        .collect::<Vec<Item<Paragraph>>>();
    let footer = Node::new(FootList(footer));

    Node(
        Inner::Root,
        vec![
            flist,
            fwin,
            footer,
        ]
    )
}

pub fn ui<'a>(frame: &mut Frame<'a, CrosstermBackend<Stdout>>, main: &Node) {
    main.visit(frame);
}
