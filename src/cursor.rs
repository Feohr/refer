use std::ops::Deref;

pub struct Pointer {
    index: usize,
    mode: [Mode; 2],
}

pub enum Mode {
    Text,
    List,
}

impl Pointer {
    pub fn new() -> Self {
        Pointer {
            index: 1,
            mode: [Mode::List, Mode::Text],
        }
    }

    pub fn shift_left(&mut self) {
        if self.index == 0 {
            self.index = 1;
            return;
        }
        self.index = self.index - 1;
    }

    pub fn shift_rigth(&mut self) {
        if self.index == 1 {
            self.index = 0;
            return;
        }
        self.index = self.index + 1;
    }

    pub fn is_list(&self) -> bool {
        match self.mode[self.index] {
            Mode::List => true,
            Mode::Text => false,
        }
    }

    pub fn is_text(&self) -> bool {
        match self.mode[self.index] {
            Mode::List => false,
            Mode::Text => true,
        }
    }
}

pub struct EntryBox(bool);

impl EntryBox {
    pub fn new() -> Self {
        EntryBox(false)
    }

    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}

impl Deref for EntryBox {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
