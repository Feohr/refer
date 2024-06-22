use std::io::Stdout;

use tui::{backend::CrosstermBackend, layout::*, style::*, text::*, widgets::*, Frame};

use crate::cursor::*;
use crate::input::*;
use crate::resource::*;

pub const BLOCK: Style = Style {
    fg: Some(Color::White),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const INVISIBLE: Style = Style {
    fg: Some(Color::Black),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const FADE: Style = Style {
    fg: Some(Color::Rgb(89, 89, 89)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

fn headers<'a>() -> Vec<Spans<'a>> {
    vec![
        Spans::from("Press 'ctrl + q'/'ctrl + c' to quit"),
        Spans::from("'ctrl + n' to add a new file"),
        Spans::from("'ctrl + up' to go to top of the file"),
        Spans::from("'ctrl + down' to go to bottom of the file"),
        Spans::from("'ctrl + t' to toggle tail mode"),
    ]
}

pub fn ui(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, res: &Resource) {
    let size = frame.size();

    let vflex = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(95)])
        .split(size);

    ui_main(frame, vflex, res);
}

fn ui_main(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, vflex: Vec<Rect>, res: &Resource) {
    let hflex = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(vflex[1]);

    ui_text(frame, hflex, res);
    ui_header(frame, vflex[0]);
}

fn ui_header(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, fflex: Rect) {
    frame.render_widget(Tabs::new(headers()), fflex);
}

fn ui_text(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, hflex: Vec<Rect>, res: &Resource) {
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
            .borders(Borders::ALL)
            .border_style(list_shade)
            .border_type(BorderType::Thick)
            .style(Style::default()),
        hflex[0],
    );

    ui_list_box(frame, hflex[0], res);

    frame.render_widget(
        Paragraph::new(vec![]).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(text_shade)
                .border_type(BorderType::Thick)
                .style(Style::default()),
        ),
        hflex[1],
    );
}

fn ui_list_box(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, hflex: Rect, res: &Resource) {
    let lflex = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(92), Constraint::Percentage(8)])
        .split(hflex);

    let items = res.get::<FileBuff>().iter();
    let list = List::new(
        items
            .map(|(i, _)| ListItem::new(i.as_str()))
            .collect::<Vec<ListItem>>(),
    )
    .block(Block::default().border_style(INVISIBLE))
    .highlight_symbol(">");

    frame.render_widget(list, lflex[0]);

    if res.get::<EntryBox>().bool() {
        let mut len = res.get::<EntryBox>().len();
        let width = lflex[1].width.saturating_sub(2) as usize;

        if len >= width {
            len = width.saturating_sub(1);
        }

        let entry_text = res.get::<EntryBox>().get_span(width);

        let entry_box = Paragraph::new(entry_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(BLOCK)
                    .style(Style::default()),
            )
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);

        frame.render_widget(entry_box, lflex[1]);
        frame.set_cursor(
            lflex[1].left() + len.saturating_add(1) as u16,
            lflex[1].top() + 1,
        );
    }
}
