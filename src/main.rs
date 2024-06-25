pub mod cursor;
pub mod input;
pub mod resource;
mod ui;

use std::io::{stdout, Stdout};
use std::ops::Drop;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crossterm::{event::*, execute, terminal::*};
use ratatui::prelude::*;

use crate::input::*;
use crate::resource::*;

pub type RectVec = Rc<[Rect]>;

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}
impl App {
    pub fn new() -> anyhow::Result<Self> {
        enable_raw_mode().expect("Couldn't enable raw terminal mode");

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

        let mut resource = Resource::new();

        loop {
            if key_listener(&mut resource)? {
                break;
            }
            self.terminal.draw(|f| ui::ui(f, &mut resource))?;
        }

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        disable_raw_mode().expect("Couldn't disable raw terminal mode");
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )
        .expect("Ran into issue while leaving the alternate screen");
        self.terminal
            .show_cursor()
            .expect("Couldn't toggle the cursor back");
    }
}

fn main() -> anyhow::Result<()> {
    let panic_buff = Arc::new(Mutex::new(String::new()));

    let old_hook = std::panic::take_hook();

    std::panic::set_hook({
        let panic_buff = panic_buff.clone();
        Box::new(move |info| {
            let mut panic_buff = panic_buff
                .lock()
                .expect("Couldn't get lock on error buffer");
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
        Err(_) => {
            return Err(anyhow::anyhow!(
                "{}",
                panic_buff
                    .lock()
                    .expect("Couldn't get lock on error buffer")
            ))
        }
    }

    Ok(())
}
