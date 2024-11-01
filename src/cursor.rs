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
