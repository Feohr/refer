use std::io::Stdout;

use tui::{backend::CrosstermBackend, layout::*, style::*, widgets::*, Frame};

const IDLE: Style = Style {
    fg: Some(Color::Rgb(105, 105, 105)),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
const HOVR: Style = Style {
    fg: Some(Color::White),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};

fn footer() -> Vec<Paragraph<'static>> {
    vec![
        Paragraph::new("`ctrl + q`/`ctrl + c` - quit"),
        Paragraph::new("`ctrl + n` - add a new file"),
        Paragraph::new("`ctrl + up` - move cursor to top"),
        Paragraph::new("`ctrl + down` - move cursor to bottom"),
        Paragraph::new("`ctrl + t` - toggle tail mode"),
    ]
}

fn basic_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Black)
}

pub fn ui(frame: &mut Frame<'_, CrosstermBackend<Stdout>>) {
    let size = frame.size();

    let footer = footer();

    let vflex = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(97), Constraint::Percentage(3)])
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

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(IDLE)
            .style(Style::default()),
        hflex[0],
    );
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(IDLE)
            .style(Style::default()),
        hflex[1],
    );

    for (widget, size) in footer.into_iter().zip(fflex) {
        frame.render_widget(
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
        );
    }
}
