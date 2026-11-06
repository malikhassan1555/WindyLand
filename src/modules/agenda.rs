//! Agenda/Calendar: simple local event tracking.

use crate::gfx::{col, Color, Rect, Weight, Align};
use crate::modules::{Ctx, Module, Rel, Toast};
use crate::store::{self, esc, unesc};
use crate::time::unix_now;
use crate::ui::Ui;

pub const ACCENT: Color = col::ACCENT;

#[derive(Clone, Debug)]
pub struct Event {
    pub title: String,
    pub time: f64, // unix timestamp
}

pub struct Agenda {
    events: Vec<Event>,
    input: String,
    editing: Option<String>,
}

impl Agenda {
    pub fn load() -> Self {
        let mut events = Vec::new();
        for line in store::read("agenda.txt").lines() {
            let p: Vec<&str> = line.splitn(2, '\t').collect();
            if p.len() == 2 {
                events.push(Event {
                    title: unesc(p[0]),
                    time: p[1].parse().unwrap_or(0.0),
                });
            }
        }
        events.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        Self { events, input: String::new(), editing: None }
    }

    fn save(&self) {
        let s: String = self.events.iter()
            .map(|e| format!("{}\t{}\n", esc(&e.title), e.time))
            .collect();
        store::write("agenda.txt", &s);
    }

    fn add_event(&mut self, title: String, time: f64) {
        self.events.push(Event { title, time });
        self.events.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self.save();
    }
}

impl Module for Agenda {
    fn title(&self) -> &'static str {
        "Agenda"
    }

    fn icon(&self) -> &'static str {
        "\u{E783}" // calendar icon
    }

    fn accent(&self) -> Color {
        ACCENT
    }

    fn size(&self) -> (f32, f32) {
        (440.0, 150.0)
    }

    fn tick(&mut self, ctx: &mut Ctx) {
        let now = unix_now() as f64;
        if let Some(e) = self.events.iter().find(|e| e.time > now) {
            let diff = e.time - now;
            if diff < 60.0 && diff > 0.0 {
                ctx.toast(Toast::new(
                    "agenda_reminder",
                    "\u{E783}",
                    ACCENT,
                    "Upcoming Event",
                    format!("{} starting soon!", e.title)
                ).secs(10.0));
            }
        }
    }

    fn action(&mut self, id: u32, _rel: Rel, ctx: &mut Ctx) {
        if id == 101 {
            let text = self.input.trim().to_string();
            if !text.is_empty() {
                let now = unix_now() as f64;
                self.add_event(text, now + 3600.0);
                self.input.clear();
                ctx.toast(Toast::new("agenda", "\u{E783}", ACCENT, "Event Added", "Saved to your agenda").secs(3.0));
            }
        }
    }

    fn tip(&self, id: u32) -> &'static str {
        match id {
            101 => "Add this event to your agenda",
            _ => "",
        }
    }

    fn draw(&mut self, ui: &mut Ui, r: Rect) {
        let mut c = r.inset_xy(20.0, 20.0);
        
        ui.text("My Agenda", c.cut_top(24.0, 10.0), 16.0, col::TEXT, Weight::Semi, Align::Center);
        
        let mut input_row = c.cut_top(30.0, 10.0);
        ui.fill_rr(input_row.cut_left(input_row.w - 60.0, 0.0), 6.0, col::BTN);
        ui.text(&self.input, input_row.cut_left(input_row.w - 60.0, 0.0), 13.0, col::TEXT, Weight::Regular, Align::Left);
        ui.button_accent(input_row.cut_right(60.0, 0.0), "Add", 101, col::ACCENT);
        
        if self.editing.is_some() {
            ui.text(&self.editing.as_ref().unwrap(), c.cut_top(30.0, 10.0), 14.0, col::TEXT, Weight::Regular, Align::Center);
        } else if self.events.is_empty() {
            ui.text("No events scheduled", c.cut_top(40.0, 10.0), 14.0, col::DIM, Weight::Regular, Align::Center);
        } else {
            for e in self.events.iter().take(3) {
                let mut row = c.cut_top(24.0, 8.0);
                ui.text(&e.title, row.cut_left(100.0, 0.0), 13.0, col::TEXT, Weight::Regular, Align::Left);
                ui.text(&crate::time::clock_full((e.time as u32 % 86400) / 3600, (e.time as u32 % 3600) / 60), row, 13.0, col::DIM, Weight::Regular, Align::Right);
            }
        }
    }
}
