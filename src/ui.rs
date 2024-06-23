use ratatui::{border, prelude::*, widgets::*};

use crate::cursor::*;
use crate::input::*; use crate::resource::*;
use crate::RectVec;

pub const FG: Color = Color::Rgb(221, 221, 221);
pub const BG: Color = Color::Rgb(53, 53, 53);
pub const DBG: Color = Color::Rgb(30, 30, 30);
pub const POINT: Color = Color::Rgb(127, 127, 127);

pub const BLOCK: Style = Style {
    fg: Some(POINT),
    bg: None,
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const INVISIBLE: Style = Style {
    fg: None,
    bg: None,
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const FADE: Style = Style {
    fg: Some(BG),
    bg: None,
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

fn headers<'a>() -> Line<'a> {
    Line::from(vec![
        Span::from("(Esc or q) quit"),
        Span::from("  │  "),
        Span::from("(n) new file"),
        Span::from("  │  "),
        Span::from("(d) delete file"),
        Span::from("  │  "),
        Span::from("(ctrl) + (j or ↑) up"),
        Span::from("  │  "),
        Span::from("(ctrl) + (k or ↓) down"),
        Span::from("  │  "),
        Span::from("(t) toggle tailing"),
    ])
}

pub fn ui(frame: &mut Frame, res: &Resource) {
    let size = frame.size();

    let vflex = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Percentage(100)])
        .split(size);

    ui_main(frame, vflex, res);
}

fn ui_main(frame: &mut Frame, vflex: RectVec, res: &Resource) {
    let hflex = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(40), Constraint::Percentage(100)])
        .split(vflex[1]);

    ui_text(frame, hflex, res);
    ui_header(frame, vflex[0]);
}

fn ui_header(frame: &mut Frame, fflex: Rect) {
    frame.render_widget(
        Paragraph::new(headers())
        .centered()
        .block(Block::default().borders(border!(ALL)).border_style(Style::default().bg(BG)))
        .style(Style::default().bg(BG).fg(FG)),
        fflex,
    );
}

fn ui_text(frame: &mut Frame, hflex: RectVec, res: &Resource) {
    let cursor = res.get::<Pointer>();
    let text_shade = if cursor.cursor_at::<View>() {
        BLOCK
    } else {
        FADE
    };
    let list_shade = if cursor.cursor_at::<Files>() {
        BLOCK
    } else {
        FADE
    };

    frame.render_widget(
        Block::default()
            .borders(border!(ALL))
            .border_style(list_shade)
            .border_type(BorderType::QuadrantOutside)
            .style(Style::default().bg(DBG).fg(FG)),
        hflex[0],
    );

    ui_list_box(frame, hflex[0], res);

    frame.render_widget(
        Paragraph::new(vec![]).block(
            Block::default()
                .borders(border!(ALL))
                .border_style(text_shade)
                .border_type(BorderType::QuadrantOutside)
                .style(Style::default().bg(DBG).fg(FG)),
        ),
        hflex[1],
    );
}

fn ui_list_box(frame: &mut Frame, hflex: Rect, res: &Resource) {
    let lflex = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100), Constraint::Min(3)])
        .split(hflex);

    let items = res.get::<FileBuff>().names();
    let list = List::new(
        items
            .map(|i| ListItem::new(i.as_str()))
            .collect::<Vec<ListItem>>(),
    )
    .block(Block::default().border_style(INVISIBLE))
    .highlight_symbol(">");

    frame.render_widget(list, lflex[0]);

    ui_entry_box(frame, lflex[1], res);
}

fn ui_entry_box(frame: &mut Frame, lflex: Rect, res: &Resource) {
    if !res.get::<EntryBox>().bool() { return }

    let mut len = res.get::<EntryBox>().len();
    let width = lflex.width.saturating_sub(2) as usize;

    if len >= width {
        len = width.saturating_sub(1);
    }

    let entry_text = res.get::<EntryBox>().get_span(width);

    let entry_box = Paragraph::new(entry_text)
        .block(
            Block::default()
                .borders(border!(ALL))
                .border_style(BLOCK)
                .style(Style::default()),
        )
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    frame.render_widget(entry_box, lflex);
    frame.set_cursor(
        lflex.left() + len.saturating_add(1) as u16,
        lflex.top() + 1,
    );
}
