use std::io::Stdout;

use tui::{backend::CrosstermBackend, layout::*, style::*, widgets::*, Frame};

use crate::cursor::*;
use crate::resource::Resource;

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

pub fn ui(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, res: &Resource) {
    let size = frame.size();

    let vflex = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(98), Constraint::Percentage(2)])
        .split(size);

    ui_main(frame, vflex, res);
}

fn ui_main(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, vflex: Vec<Rect>, res: &Resource) {
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

    ui_text(frame, hflex, res);
    ui_footer(frame, fflex);
}

fn ui_text(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, hflex: Vec<Rect>, res: &Resource) {
    ui_list_box(frame, hflex[0], res);

    let cursor = res.get::<Pointer>();
    let text_shade = if cursor.is_text() { BLOCK } else { FADE };

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(text_shade)
            .border_type(BorderType::Thick)
            .style(Style::default()),
        hflex[1],
    );
}

fn ui_list_box(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, hflex: Rect, res: &Resource) {
    let lflex = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(hflex);

    let items = res.get::<Vec<String>>();

    let cursor = res.get::<Pointer>();
    let list_shade = if cursor.is_list() { BLOCK } else { FADE };

    let list = List::new(
        items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect::<Vec<ListItem>>(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(list_shade)
            .border_type(BorderType::Thick)
            .style(Style::default()),
    )
    .style(basic_style())
    .highlight_symbol(">");

    frame.render_widget(list, lflex[0]);

    let visible = res.get::<EntryBox>();
    let entry_style = if visible.bool() { BLOCK } else { INVISIBLE };

    let entry_box = Block::default()
        .borders(Borders::ALL)
        .border_style(entry_style)
        .style(Style::default());

    frame.render_widget(entry_box, lflex[1]);
}

fn ui_footer(frame: &mut Frame<'_, CrosstermBackend<Stdout>>, fflex: Vec<Rect>) {
    for (footer, size) in footer().into_iter().zip(fflex) {
        frame.render_widget(
            footer
                .block(
                    Block::default()
                        .borders(Borders::LEFT)
                        .border_style(BLOCK)
                        .style(Style::default()),
                )
                .style(basic_style())
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Left),
            size,
        );
    }
}
