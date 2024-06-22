mod ui;
pub mod resource;
pub mod cursor;

use std::io::{stdout, Stdout};
use std::ops::Drop;
use std::sync::{Arc, Mutex};

use clap::Parser;
use crossterm::{event::*, execute, terminal::*};
use tui::{backend::CrosstermBackend, Terminal};

use crate::resource::Resource;
use crate::cursor::{Pointer, EntryBox};

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
                let entry = res.get_mut::<EntryBox>();
                entry.toggle();
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
    resource.insert(EntryBox::new());

    Ok(resource)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let panic_buff = Arc::new(Mutex::new(String::new()));

    let old_hook = std::panic::take_hook();

    std::panic::set_hook({
        let panic_buff = panic_buff.clone();
        Box::new(move |info| {
            let mut panic_buff = panic_buff.lock().unwrap();
            let msg = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<dyn Any>",
                },
            };
            panic_buff.push_str(msg);
        })
    });

    let res = std::panic::catch_unwind(|| {
        let mut main = match App::new() {
            Ok(main) => main,
            Err(err) => panic!("Couldn't create App object: {err}"),
        };

        if let Err(err) = main.run() {
            panic!("Ran into issue while running the application: {err}");
        }
    });

    std::panic::set_hook(old_hook);

    match res {
        Ok(res) => res,
        Err(_) => return Err(anyhow::anyhow!("{}", panic_buff.lock().unwrap())),
    }

    Ok(())
}
