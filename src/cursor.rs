use std::any::TypeId;

/// The cursor is on file list.
pub struct Files;
/// the cursor is on text view.
pub struct View;

/// To denote where the current cursor is located.
pub struct KeyboardCursor {
    toggle: Option<TypeId>,
    curr: TypeId,
}

impl KeyboardCursor {
    #[inline]
    pub fn new() -> Self {
        KeyboardCursor::default()
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

impl Default for KeyboardCursor {
    fn default() -> Self {
        KeyboardCursor {
            toggle: None,
            curr: TypeId::of::<View>(),
        }
    }
}
