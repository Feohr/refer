/*
 * MIT License
 *
 * Copyright (c) 2024 Mohammed Rehaan and contributors
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * */

use ratatui::{border, prelude::*, widgets::*};

use crate::cursor::*;
use crate::resource::*;
use crate::RectVec;

const BORDER: BorderType = BorderType::Thick;
const RFG: Color = Color::Gray;
const RBG: Color = Color::Rgb(20, 20, 20);
const DFG: Color = Color::Rgb(80, 80, 80);
const EFG: Color = Color::LightRed;
const LOG: Color = Color::DarkGray;

const BLOCK: Style = Style {
    fg: Some(RFG),
    bg: Some(RBG),
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
const ERR: Style = Style {
    fg: Some(EFG),
    bg: Some(RBG),
    underline_color: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
const LOG_MSG: Style = Style {
    fg: Some(LOG),
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

const HEADERS: &str = "\
    (ctrl) + (q) quit  │  \
    (ctrl) + (n) new file  │  \
    (ctrl) + (d) delete file  │  \
    (ctrl) + (j or ↑) up  │  \
    (ctrl) + (k or ↓) down  │  \
    (ctrl) + (t) toggle tailing";

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

    ui_main_frame(frame, hflex, res);
    ui_header(frame, vflex[0]);
}

fn ui_header(frame: &mut Frame, fflex: Rect) {
    frame.render_widget(
        Paragraph::new(Line::from(Span::from(HEADERS)))
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

#[inline]
fn get_cursor_shade_from_condition(cond: bool) -> Style {
    if cond {
        BLOCK
    } else {
        FADE
    }
}

fn get_lines_from_buffer(res: &Resource, hflex: Rect) -> Vec<Line> {
    let curr_index = res.file_list_state().index();
    let Some(curr_buff) = res.files().get_file_buff(curr_index) else {
        return Default::default(); // Return default
    };
    let (buffer, nulled) = curr_buff.buffer(hflex);
    buffer
        .into_iter()
        .map(|l| Line::styled(l, if nulled { LOG_MSG } else { BLOCK }))
        .collect::<Vec<Line>>()
}

fn ui_main_frame(frame: &mut Frame, hflex: RectVec, res: &mut Resource) {
    ui_list_box_main(frame, hflex[0], res);
    ui_text_main(frame, hflex[1], res);
}

fn ui_list_box_main(frame: &mut Frame, hflex: Rect, res: &mut Resource) {
    let cursor = res.pointer();

    frame.render_widget(
        Block::default()
            .borders(border!(ALL))
            .title(" Files ")
            .title_alignment(Alignment::Center)
            .border_style(get_cursor_shade_from_condition(cursor.cursor_at::<Files>()))
            .border_type(BORDER)
            .style(Style::default().bg(RBG).fg(RFG)),
        hflex,
    );

    ui_list_box(frame, hflex, res);
}

fn ui_text_main(frame: &mut Frame, hflex: Rect, res: &mut Resource) {
    let cursor = res.pointer();

    frame.render_widget(
        Paragraph::new(get_lines_from_buffer(res, hflex))
            .block(
                Block::default()
                    .borders(border!(ALL))
                    .border_style(get_cursor_shade_from_condition(cursor.cursor_at::<View>()))
                    .border_type(BORDER)
                    .style(Style::default().bg(RBG).fg(RFG)),
            )
            .wrap(Wrap { trim: false }),
        hflex,
    );
}

fn get_list<'a>(items: &[&'a str]) -> List<'a> {
    List::new(get_list_items(items))
        .block(Block::default().border_style(INVISIBLE))
        .highlight_symbol(" ► ")
        .highlight_style(Style::default().fg(RBG).bg(RFG))
}

fn get_list_items<'a>(items: &[&'a str]) -> Vec<ListItem<'a>> {
    items
        .iter()
        .map(|&i| ListItem::new(i))
        .collect::<Vec<ListItem>>()
}

fn ui_list_box(frame: &mut Frame, hflex: Rect, res: &mut Resource) {
    let lflex = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100), Constraint::Min(3)])
        .split(hflex);

    let list_items = res.files().names();
    let list = get_list(&list_items);

    frame.render_stateful_widget(list, lflex[0], res.file_list_state_mut().get_mut());

    ui_entry_box(frame, lflex[1], res);
}

fn ui_entry_box(frame: &mut Frame, lflex: Rect, res: &mut Resource) {
    if !res.entry_box().is_visible() {
        return;
    }

    let width = lflex.width.saturating_sub(3) as usize; // 2 (borders) + 1 (char)
    let len = res.entry_box().len().min(width);

    let is_err = res.entry_box().is_err();

    let entry_text = res.entry_box().get_span(width);
    let entry_box = Paragraph::new(entry_text)
        .block(
            Block::default()
                .borders(border!(ALL))
                .border_style([BLOCK, ERR][is_err as usize])
                .title(" Filename ")
                .title_alignment(Alignment::Left)
                .border_type(BORDER),
        )
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    frame.render_widget(entry_box, lflex);
    frame.set_cursor(
        lflex.left().saturating_add(len.saturating_add(1) as u16),
        lflex.top() + 1,
    );
}
