use ratatui::{border, prelude::*, widgets::*};

use crate::cursor::*;
use crate::resource::*;
use crate::io::*;
use crate::RectVec;

const BORDER: BorderType = BorderType::Rounded;
const RFG: Color = Color::Gray;
const RBG: Color = Color::Rgb(20, 20, 20);
const DFG: Color = Color::Rgb(100, 100, 100);

const BLOCK: Style = Style {
    fg: Some(RFG),
    bg: Some(RBG),
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
const INVISIBLE: Style = Style {
    fg: None,
    bg: None,
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
const FADE: Style = Style {
    fg: Some(DFG),
    bg: Some(RBG),
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

fn headers<'a>() -> Line<'a> {
    Line::from(vec![
        Span::from("(ctrl) + (q) quit"),
        Span::from("  │  "),
        Span::from("(ctrl) + (n) new file"),
        Span::from("  │  "),
        Span::from("(ctrl) + (d) delete file"),
        Span::from("  │  "),
        Span::from("(ctrl) + (j or ↑) up"),
        Span::from("  │  "),
        Span::from("(ctrl) + (k or ↓) down"),
        Span::from("  │  "),
        Span::from("(ctrl) + (t) toggle tailing"),
    ])
}

pub fn ui(frame: &mut Frame, res: &mut Resource) {
    let size = frame.size();

    let vflex = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Percentage(100)])
        .split(size);

    ui_main(frame, vflex, res);
}

fn ui_main(frame: &mut Frame, vflex: RectVec, res: &mut Resource) {
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
            .block(
                Block::default()
                    .borders(border!(ALL))
                    .border_type(BORDER)
                    .border_style(Style::default().fg(RFG).bg(RBG)),
            )
            .style(Style::default().bg(RBG).fg(RFG)),
        fflex,
    );
}

fn ui_text(frame: &mut Frame, hflex: RectVec, res: &mut Resource) {
    let cursor = res.pointer();
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
            .border_type(BORDER)
            .style(Style::default().bg(RBG).fg(RFG)),
        hflex[0],
    );

    ui_list_box(frame, hflex[0], res);

    frame.render_widget(
        Paragraph::new(vec![]).block(
            Block::default()
                .borders(border!(ALL))
                .border_style(text_shade)
                .border_type(BORDER)
                .style(Style::default().bg(RBG).fg(RFG)),
        ),
        hflex[1],
    );
}

fn get_list<'a>(items: &'a [&'a FileName]) -> List<'a> {
    List::new(get_list_items(items))
        .block(Block::default().border_style(INVISIBLE))
        .highlight_symbol(" ► ")
        .highlight_style(Style::default().fg(RBG).bg(DFG))
}

fn get_list_items<'a>(items: &'a [&'a FileName]) -> Vec<ListItem<'a>> {
    items
        .into_iter()
        .map(|i| ListItem::new(i.value()))
        .collect::<Vec<ListItem>>()
}

fn ui_list_box(frame: &mut Frame, hflex: Rect, res: &mut Resource) {
    let lflex = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100), Constraint::Min(3)])
        .split(hflex);

    let list_items = res
        .files()
        .iter()
        .collect::<Vec<&FileName>>();
    let list = get_list(&list_items);

    {
        let mut state = res.file_list_state_mut();
        let state = state.get_mut();
        frame.render_stateful_widget(list, lflex[0], state);
    }

    ui_entry_box(frame, lflex[1], res);
}

fn ui_entry_box(frame: &mut Frame, lflex: Rect, res: &mut Resource) {
    if !res.entry_box().is_visible() {
        return;
    }

    let mut len = res.entry_box().len();
    let width = lflex.width.saturating_sub(2) as usize;

    if len >= width {
        len = width.saturating_sub(1);
    }

    let entry_text = res.entry_box().get_span(width);

    let entry_box = Paragraph::new(entry_text)
        .block(
            Block::default()
                .borders(border!(ALL))
                .border_style(BLOCK)
                .border_type(BORDER)
        )
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    frame.render_widget(entry_box, lflex);
    frame.set_cursor(lflex.left() + len.saturating_add(1) as u16, lflex.top() + 1);
}
