mod ui;
pub mod resource;
pub mod cursor;

use std::io::{stdout, Stdout};
use std::ops::Drop;

use clap::Parser;
use crossterm::{event::*, execute, terminal::*};
use tui::{backend::CrosstermBackend, Terminal};

use crate::resource::Resource;
use crate::cursor::Pointer;

pub const DELTA: u64 = 16;

#[derive(Parser)]
#[command(about, long_about=None)]
struct Refer {
    filename: Vec<String>,
}

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}
impl App {
    pub fn new() -> anyhow::Result<Self> {
        enable_raw_mode().unwrap();

        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;

        Ok(App { terminal })
    }

    fn run(&mut self) -> anyhow::Result<()> {
        execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;

        let mut resource = init_resource()?;

        loop {
            if key_listener(&mut resource)? {
                return Ok(());
            }

            self.terminal.draw(|f| ui::ui(f, &resource))?;
        }
    }
}

impl Drop for App {
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

fn key_listener(res: &mut Resource) -> anyhow::Result<bool> {
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
            Event::Key(KeyEvent {
                code: KeyCode::Char('n'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                let list = res.get_mut::<Vec<String>>();
                list.push("new_file".to_string());
            },
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                let pointer = res.get_mut::<Pointer>();
                pointer.shift_left();
            },
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                let pointer = res.get_mut::<Pointer>();
                pointer.shift_rigth();
            },
            _ => {}
        }
    }

    Ok(false)
}

fn init_resource() -> anyhow::Result<Resource> {
    let args = Refer::parse();

    let mut resource = Resource::default();
    resource.insert(args.filename);
    resource.insert(Pointer::new());

    Ok(resource)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut main = App::new()?;
    main.run()?;

    Ok(())
}
