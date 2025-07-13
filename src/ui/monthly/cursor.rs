pub(super) struct Cursor {
    shown: bool,

    x: i32,
    y: i32,

    w: i32,
    h: i32,

    maxx: i32,
}

impl Cursor {
    pub(super) fn new() -> Self {
        Self {
            shown: false,
            w: 0,
            h: 0,
            maxx: 0,
            x: 0,
            y: 0,
        }
    }

    pub fn set_w(&mut self, w: i32) {
        self.w = w;
    }

    pub(super) fn set_h(&mut self, h: i32) {
        self.h = h;
    }

    pub(super) fn set_max_day(&mut self, max_day: i32) {
        self.maxx = max_day;
    }

    pub(super) fn change_shown(&mut self) {
        self.shown = !self.shown;
    }

    pub(super) fn with_w(mut self, w: i32) -> Self {
        self.w = w;
        self
    }

    pub(super) fn with_h(mut self, h: i32) -> Self {
        self.h = h;
        self
    }

    pub(super) fn with_max_day(mut self, max_day: i32) -> Self {
        self.maxx = max_day;
        self
    }

    pub(super) fn move_left(&mut self) {
        if !self.shown {
            return;
        }

        if self.x > 0 {
            self.x -= 1;
        } else if self.y > 0 {
            self.y -= 1;
            self.x = self.w - 1;
        }
        self.clamp_to_maxx();
    }

    pub(super) fn move_right(&mut self) {
        if !self.shown {
            return;
        }

        if self.x < self.w - 1 {
            self.x += 1;
        } else {
            self.x = 0;
            self.y += 1;
        }
        self.clamp_to_maxx();
    }

    pub(super) fn move_top(&mut self) {
        if !self.shown {
            return;
        }

        if self.y > 0 {
            self.y -= 1;
            self.clamp_to_maxx();
        }
    }

    pub(super) fn move_bottom(&mut self) {
        if !self.shown {
            return;
        }

        self.y += 1;
        self.clamp_to_maxx();
    }

    fn clamp_to_maxx(&mut self) {
        let day = self.y * self.w + self.x + 1;
        if day > self.maxx {
            let new_day = self.maxx - 1;
            self.y = new_day / self.w;
            self.x = new_day % self.w;
        }
    }

    pub(super) fn current_day(&self) -> Option<i32> {
        if !self.shown {
            return None;
        }

        let day = self.y * self.w + self.x + 1;
        if day > 0 && day <= self.maxx {
            Some(day)
        } else {
            None
        }
    }
}
