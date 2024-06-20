mod ui;

use std::io::{stdout, Stdout};
use std::ops::Drop;

use clap::Parser;
use crossterm::{event::*, execute, terminal::*};
use tui::{backend::CrosstermBackend, Terminal};

pub const DELTA: u64 = 16;

#[derive(Parser)]
#[command(about, long_about=None)]
struct Refer {
    /// Name of the files to open.
    #[arg(short, long)]
    filename: Vec<String>,
}

struct Main {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}
impl Main {
    pub fn new() -> anyhow::Result<Self> {
        enable_raw_mode().unwrap();

        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Main { terminal })
    }

    fn run(&mut self) -> anyhow::Result<()> {
        execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;

        loop {
            self.terminal.draw(ui::ui)?;
            if listen_close_window()? {
                return Ok(());
            }
        }
    }
}

impl Drop for Main {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

fn listen_close_window() -> anyhow::Result<bool> {
    if poll(std::time::Duration::from_millis(DELTA))? {
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => return Ok(true),
            _ => {}
        }
    }

    Ok(false)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = Refer::parse();

    let mut main = Main::new()?;
    main.run()?;

    Ok(())
}
