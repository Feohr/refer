use std::any::TypeId;

pub struct Pointer {
    toggle: Option<TypeId>,
    curr: TypeId,
}

pub enum Mode {
    Text,
    List,
}

impl Pointer {
    pub fn new() -> Self {
        Pointer {
            toggle: None,
            curr: TypeId::of::<Self>(),
        }
    }

    pub fn set_cursor<W: 'static>(&mut self) {
        if self.toggle.is_none() {
            self.curr = TypeId::of::<W>();
        }
    }

    pub fn toggle(&mut self) {
        if let Some(toggle) = self.toggle.take() {
            self.curr = toggle;
            return;
        }
        self.toggle = Some(self.curr);
        self.curr = TypeId::of::<Self>();
    }

    pub fn cursor_at<W: 'static>(&self) -> bool {
        if self.toggle.is_none() {
            return self.curr == TypeId::of::<W>();
        }
        false
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

    pub fn bool(&self) -> bool {
        self.0
    }
}

pub struct Files;
pub struct View;
