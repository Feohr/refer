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
