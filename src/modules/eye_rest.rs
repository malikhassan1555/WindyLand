//! Eye-rest reminder (the 20-20-20 rule).
//! Every 20 minutes, remind the user to look at something 20 feet away for 20 seconds.

use crate::gfx::{col, Color, Rect, Weight, Align};
use crate::modules::{Ctx, Module, Toast};
use crate::time::unix_now;
use crate::ui::Ui;

pub const ACCENT: Color = col::ACCENT;

pub struct EyeRest {
    last_reminder: f64,
    resting: bool,
    rest_until: f64,
}

impl EyeRest {
    pub fn load() -> Self {
        Self {
            last_reminder: unix_now() as f64,
            resting: false,
            rest_until: 0.0,
        }
    }
}

impl Module for EyeRest {
    fn title(&self) -> &'static str {
        "Eye Rest"
    }

    fn icon(&self) -> &'static str {
        "\u{E710}" // eye icon
    }

    fn accent(&self) -> Color {
        ACCENT
    }

    fn size(&self) -> (f32, f32) {
        (400.0, 100.0)
    }

    fn tick(&mut self, ctx: &mut Ctx) {
        let t = unix_now() as f64;
        let interval = 20.0 * 60.0; // 20 minutes

        if !self.resting && t - self.last_reminder >= interval {
            self.resting = true;
            self.rest_until = t + 20.0;
            ctx.toast(Toast::new(
                "eye_rest", 
                "\u{E710}", 
                ACCENT, 
                "Eye Rest Time!", 
                "Look 20 feet away for 20 seconds to reduce strain."
            ).secs(10.0));
        }

        if self.resting && t >= self.rest_until {
            self.resting = false;
            self.last_reminder = t;
        }
    }

    fn draw(&mut self, ui: &mut Ui, r: Rect) {
        let t = unix_now() as f64;
        let mut c = r.inset_xy(20.0, 20.0);
        
        ui.text("Eye Rest Reminder", c.cut_top(24.0, 10.0), 16.0, col::TEXT, Weight::Semi, Align::Center);
        
        let status = if self.resting {
            format!("Resting... (ends in {:.0}s)", (self.rest_until - t).max(0.0))
        } else {
            let remaining = (20.0 * 60.0 - (t - self.last_reminder)).max(0.0);
            format!("Next break in {:.0}m {:.0}s", remaining / 60.0, remaining % 60.0)
        };
        
        ui.text(&status, c.cut_top(40.0, 10.0), 14.0, col::DIM, Weight::Regular, Align::Center);
    }
}
