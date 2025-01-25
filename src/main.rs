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

pub mod cursor;
pub mod input;
pub mod io;
pub mod resource;
mod ui;
mod utils;

use std::{
    fs::{create_dir_all, OpenOptions},
    io::{stdout, Stdout},
    ops::Drop,
    rc::Rc,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crossterm::{event::*, execute, terminal::*};
use ratatui::prelude::*;
use simplelog::WriteLogger;

use crate::input::*;
use crate::resource::*;

/*
 * fps: 30
 * 1 second in millis: 1000
 * */
const SLEEP_DURATION: u64 = 1000 / 30;

const LOGFILE_NAME: &str = "refer.log";
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

        let mut resource = Resource::new()?;
        create_logger()?;

        'draw: loop {
            trigger_view_update(&mut resource);
            self.terminal.draw(|f| ui::ui(f, &mut resource))?;
            detrigger_view_update(&mut resource);

            let key_listen_reponse = key_listener(&mut resource)?;
            if key_listen_reponse.should_exit() {
                break 'draw;
            }

            state_update(&mut resource);

            sleep(Duration::from_millis(SLEEP_DURATION));
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
            // Some arbitrary type message if we cannot get it from the error.
            let default_type = "Box<dyn Any>";
            let mut panic_buff = panic_buff
                .lock()
                .expect("Couldn't get lock on error buffer");
            let msg = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => &default_type,
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
        Err(err) => {
            return Err(anyhow::anyhow!(
                "{err:?}:{}",
                panic_buff
                    .lock()
                    .expect("Couldn't get lock on error buffer")
            ))
        }
    }

    Ok(())
}

pub fn bounded_add(value: usize, other: usize, bound: usize) -> usize {
    if value < bound {
        return value.saturating_add(other);
    }
    value
}

fn create_logger() -> anyhow::Result<()> {
    let log_dir = dirs::cache_dir()
        .ok_or_else(|| anyhow::anyhow!("Couldn't find cache directory"))?
        .join("refer")
        .join("log");

    if !log_dir.exists() {
        create_dir_all(&log_dir)
            .map_err(|err| anyhow::anyhow!("Couldn't create the directory due to: {err}"))?;
    }

    let log_path = log_dir.join(LOGFILE_NAME);

    let log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(log_path)
        .map_err(|err| anyhow::anyhow!("Couldn't create the log file due to: {err}"))?;

    WriteLogger::init(
        simplelog::LevelFilter::Trace,
        simplelog::Config::default(),
        log_file,
    )?;

    Ok(())
}
