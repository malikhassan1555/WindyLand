//! Sky widget: sun position, sunrise/sunset, moon phase and illumination for
//! the chosen city (default Bhakkar, Asia/Karachi). Everything is calculated
//! on this PC — nothing is fetched from the internet.
//!
//! Sun maths: standard solar declination + equation of time (same formulas as
//! the prayer module). Moon phase: days since a reference new moon divided by
//! the synodic month (29.5306 days).

use crate::gfx::*;
use crate::modules::{Ctx, Module, Rel};
use crate::store;
use crate::time::Now;
use crate::ui::Ui;

pub const ACCENT: Color = hex(0x63E6BE);

const MOON_NEW: &str = "🌑";
const MOON_WAXING: &str = "🌒";
const MOON_FIRST: &str = "🌓";
const MOON_WAXING_GIB: &str = "🌔";
const MOON_FULL: &str = "🌕";
const MOON_WANING_GIB: &str = "🌖";
const MOON_LAST: &str = "🌗";
const MOON_WANING: &str = "🌘";

// Reuse the prayer city list (default Bhakkar).
pub const CITIES: [(&str, f64, f64); 10] = crate::modules::prayer::CITIES;

const A_CITY: u32 = 10; // + index

fn dsin(d: f64) -> f64 { d.to_radians().sin() }
fn dcos(d: f64) -> f64 { d.to_radians().cos() }
fn darcsin(x: f64) -> f64 { x.asin().to_degrees() }
fn darccos(x: f64) -> f64 { x.acos().to_degrees() }
fn darctan2(y: f64, x: f64) -> f64 { y.atan2(x).to_degrees() }
fn fix(a: f64, b: f64) -> f64 { a.rem_euclid(b) }

fn julian(y: i32, m: i32, d: i32) -> f64 {
    let (y, m) = if m <= 2 { (y - 1, m + 12) } else { (y, m) };
    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    (365.25 * (y as f64 + 4716.0)).floor() + (30.6001 * (m as f64 + 1.0)).floor() + d as f64 + b - 1524.5
}

/// Sun declination (degrees) and equation of time (hours) for a Julian date.
fn sun(jd: f64) -> (f64, f64) {
    let d = jd - 2451545.0;
    let g = fix(357.529 + 0.98560028 * d, 360.0);
    let q = fix(280.459 + 0.98564736 * d, 360.0);
    let l = fix(q + 1.915 * dsin(g) + 0.020 * dsin(2.0 * g), 360.0);
    let e = 23.439 - 0.00000036 * d;
    let ra = darctan2(dcos(e) * dsin(l), dcos(l)) / 15.0;
    let eqt = q / 15.0 - fix(ra, 24.0);
    (darcsin(dsin(e) * dsin(l)), eqt)
}

/// Sun altitude & azimuth (degrees) at a local hour-fraction for a date.
fn sun_pos(lat: f64, lon: f64, tz: f64, y: i32, m: i32, d: i32, hf: f64) -> (f64, f64) {
    let jd = julian(y, m, d) - 0.5 + hf / 24.0;
    let (dec, eqt) = sun(jd);
    // Hour angle (degrees) = 15 * (apparent solar time - 12)
    let ha = 15.0 * (hf - tz + lon / 15.0 - 12.0) + eqt * 15.0;
    let alt = darcsin(dsin(lat) * dsin(dec) + dcos(lat) * dcos(dec) * dcos(ha));
    let az = darctan2(dsin(ha), dcos(ha) * dsin(lat) - dtan(dec) * dcos(lat));
    let az = fix(az + 360.0, 360.0);
    (alt, az)
}
fn dtan(d: f64) -> f64 { d.to_radians().tan() }

fn sunrise_sunset(lat: f64, lon: f64, tz: f64, y: i32, m: i32, d: i32) -> (f64, f64) {
    let jd = julian(y, m, d);
    let (decl, _) = sun(jd);
    let noon = fix(12.0 - sun(jd).1, 24.0);
    let x = (-dsin(0.833) - dsin(decl) * dsin(lat)) / (dcos(decl) * dcos(lat));
    if x > 1.0 { return (f64::NAN, f64::NAN); } // polar day
    if x < -1.0 { return (0.0, 24.0); } // polar night
    let h = darccos(x) / 15.0;
    let adj = tz - lon / 15.0;
    (fix(noon - h + adj, 24.0), fix(noon + h + adj, 24.0))
}

/// Compass direction name for an azimuth (0 = N, 90 = E, 180 = S, 270 = W).
fn dir_name(az: f64) -> &'static str {
    let i = ((az / 22.5 + 0.5) as i64).rem_euclid(16);
    match i {
        0 => "N", 1 => "NNE", 2 => "NE", 3 => "ENE", 4 => "E", 5 => "ESE",
        6 => "SE", 7 => "SSE", 8 => "S", 9 => "SSW", 10 => "SW", 11 => "WSW",
        12 => "W", 13 => "WNW", 14 => "NW", 15 => "NNW", _ => "N",
    }
}

/// Moon phase: 0 = new, 0.25 = first quarter, 0.5 = full, 0.75 = last quarter.
fn moon_phase(jd: f64) -> f64 {
    // Reference new moon: 6 Jan 2000, 18:14 UTC.
    let ref_nm = 2451550.1;
    let synodic = 29.530588853;
    fix((jd - ref_nm) / synodic, 1.0)
}

fn moon_name(phase: f64) -> &'static str {
    match phase {
        p if p < 0.03 || p > 0.97 => "New Moon",
        p if p < 0.22 => "Waxing Crescent",
        p if p < 0.28 => "First Quarter",
        p if p < 0.47 => "Waxing Gibbous",
        p if p < 0.53 => "Full Moon",
        p if p < 0.72 => "Waning Gibbous",
        p if p < 0.78 => "Last Quarter",
        _ => "Waning Crescent",
    }
}

fn moon_emoji(phase: f64) -> &'static str {
    match phase {
        p if p < 0.06 => MOON_NEW,
        p if p < 0.19 => MOON_WAXING,
        p if p < 0.31 => MOON_FIRST,
        p if p < 0.44 => MOON_WAXING_GIB,
        p if p < 0.56 => MOON_FULL,
        p if p < 0.69 => MOON_WANING_GIB,
        p if p < 0.81 => MOON_LAST,
        _ => MOON_WANING,
    }
}

fn moon_illumination(phase: f64) -> f64 {
    (1.0 - (2.0 * std::f64::consts::PI * phase).cos()) / 2.0
}

pub struct Sky {
    city: Option<usize>,
    lat: f64,
    lon: f64,
}

impl Sky {
    pub fn load() -> Self {
        let kv = store::parse_kv(&store::read("sky.txt"));
        let city = match kv.get("city").map(String::as_str) {
            Some("custom") => None,
            Some(name) => Some(CITIES.iter().position(|c| c.0 == name).unwrap_or(0)),
            None => Some(0),
        };
        let (dlat, dlon) = city.map_or((CITIES[0].1, CITIES[0].2), |i| (CITIES[i].1, CITIES[i].2));
        Sky { city, lat: kv.get("lat").and_then(|v| v.parse().ok()).unwrap_or(dlat), lon: kv.get("lon").and_then(|v| v.parse().ok()).unwrap_or(dlon) }
    }

    fn save(&mut self) {
        let mut kv = std::collections::BTreeMap::new();
        kv.insert("city".into(), self.city.map_or("custom".into(), |i| CITIES[i].0.to_string()));
        kv.insert("lat".into(), format!("{:.4}", self.lat));
        kv.insert("lon".into(), format!("{:.4}", self.lon));
        store::write("sky.txt", &store::write_kv(&kv));
    }

    fn place(&self) -> String {
        match self.city { Some(i) => CITIES[i].0.to_string(), None => format!("{:.2}, {:.2}", self.lat, self.lon) }
    }

    fn hhmm(h: f64) -> String {
        if !h.is_finite() { return "--:--".into(); }
        let m = ((h * 60.0).round() as i64).rem_euclid(24 * 60);
        format!("{:02}:{:02}", m / 60, m % 60)
    }

    fn sun_card(&mut self, ui: &mut Ui, mut r: Rect, now: &Now) {
        let hf = now.hour as f64 + now.min as f64 / 60.0 + now.sec as f64 / 3600.0;
        let (alt, az) = sun_pos(self.lat, self.lon, 5.0, now.year as i32, now.month as i32, now.day as i32, hf);
        let (sr, ss) = sunrise_sunset(self.lat, self.lon, 5.0, now.year as i32, now.month as i32, now.day as i32);

        ui.text("Sun", r.cut_top(18.0, 6.0), 14.0, col::TEXT, Weight::Semi, Align::Left);
        let mut row = r.cut_top(30.0, 6.0);
        let alt_txt = if alt.is_finite() { format!("{:.0}°", alt) } else { "—".into() };
        let alt_lbl = format!("Altitude {}", alt_txt);
        ui.text(&alt_lbl, row.cut_left(120.0, 8.0), 12.0, col::TEXT, Weight::Regular, Align::Left);
        ui.text(dir_name(az), row.cut_left(80.0, 8.0), 12.0, col::DIM, Weight::Regular, Align::Left);
        let az_lbl = format!("{:.0}°", az);
        ui.text(&az_lbl, row, 12.0, col::DIM, Weight::Regular, Align::Left);

        let mut row = r.cut_top(24.0, 6.0);
        let sr_lbl = format!("Sunrise {}", Self::hhmm(sr));
        ui.text(&sr_lbl, row.cut_left(120.0, 6.0), 11.0, col::TEXT, Weight::Regular, Align::Left);
        let ss_lbl = format!("Sunset {}", Self::hhmm(ss));
        ui.text(&ss_lbl, row, 11.0, col::TEXT, Weight::Regular, Align::Left);

        // Next event countdown.
        let m = now.hour as f64 * 60.0 + now.min as f64 + now.sec as f64 / 60.0;
        let next = if !sr.is_finite() { None } else {
            let srm = sr * 60.0; let ssm = ss * 60.0;
            if m < srm { Some(("Sunrise", srm - m)) } else if m < ssm { Some(("Sunset", ssm - m)) } else { None }
        };
        if let Some((name, left)) = next {
            let (h, mm) = ((left / 60.0).floor() as i64, (left % 60.0).floor() as i64);
            let next_lbl = format!("Next {name} in {h}h {mm:02}m");
            ui.text(&next_lbl, r.cut_top(20.0, 0.0), 11.0, ACCENT, Weight::Semi, Align::Left);
        }
    }

    fn moon_card(&mut self, ui: &mut Ui, mut r: Rect) {
        let now = Now::get();
        let jd = julian(now.year as i32, now.month as i32, now.day as i32) - 0.5
            + (now.hour as f64 + now.min as f64 / 60.0 + now.sec as f64 / 3600.0) / 24.0;
        let phase = moon_phase(jd);
        let illum = moon_illumination(phase);

        ui.text("Moon", r.cut_top(18.0, 6.0), 14.0, col::TEXT, Weight::Semi, Align::Left);
        let mut row = r.cut_top(34.0, 6.0);
        ui.text(moon_emoji(phase), row.cut_left(40.0, 6.0), 26.0, col::TEXT, Weight::Regular, Align::Center);
        ui.text(moon_name(phase), row.cut_left(120.0, 6.0), 12.0, col::TEXT, Weight::Regular, Align::Left);
        let ill_lbl = format!("{:.0}%", illum * 100.0);
        ui.text(&ill_lbl, row.cut_left(60.0, 6.0), 12.0, col::DIM, Weight::Regular, Align::Left);

        let age = (phase * 29.5306).round() as i64;
        let age_lbl = format!("Age {age} d  ·  {}", self.place());
        ui.text(&age_lbl, r.cut_top(20.0, 0.0), 10.5, col::DIM, Weight::Regular, Align::Left);

        // Next new/full moon.
        let synodic = 29.530588853;
        let target = if phase < 0.5 { 0.5 } else { 1.0 };
        let days = (target - phase) * synodic;
        let (d, hh) = ((days.floor() as i64), ((days - days.floor()) * 24.0).round() as i64);
        let name = if phase < 0.5 { "Full Moon" } else { "New Moon" };
        let nxt_lbl = format!("Next {name} in {d}d {hh:02}h");
        ui.text(&nxt_lbl, r.cut_top(18.0, 0.0), 10.5, ACCENT, Weight::Semi, Align::Left);
    }
}

impl Module for Sky {
    fn title(&self) -> &'static str { "Sky" }
    fn icon(&self) -> &'static str { "\u{E71B}" } // sun icon
    fn accent(&self) -> Color { ACCENT }
    fn size(&self) -> (f32, f32) { (440.0, 150.0) }

    fn draw(&mut self, ui: &mut Ui, r: Rect) {
        let now = Now::get();
        let mut c = r.inset_xy(18.0, 16.0);
        // Location row.
        let mut loc = c.cut_top(24.0, 8.0);
        ui.text("Location", loc.cut_left(54.0, 6.0), 11.0, col::DIM, Weight::Regular, Align::Left);
        for (i, cell) in loc.columns(4, 6.0).into_iter().enumerate() {
            if i < CITIES.len() {
                ui.chip(cell, CITIES[i].0, A_CITY + i as u32, self.city == Some(i), ACCENT);
            }
        }
        c = Rect::new(c.x, c.y, c.w, c.h);
        let mut rest = c;
        rest.y += 32.0;
        let half = rest.cut_left(rest.w / 2.0 - 10.0, 0.0);
        self.sun_card(ui, half, &now);
        let other = Rect::new(rest.x + half.w + 20.0, rest.y, rest.w - half.w - 20.0, rest.h);
        self.moon_card(ui, other);
    }

    fn action(&mut self, id: u32, _rel: Rel, _ctx: &mut Ctx) {
        if (A_CITY..A_CITY + CITIES.len() as u32).contains(&id) {
            let c = (id - A_CITY) as usize;
            self.city = Some(c);
            self.lat = CITIES[c].1;
            self.lon = CITIES[c].2;
            self.save();
        }
    }

    fn tip(&self, id: u32) -> &'static str {
        if (A_CITY..A_CITY + CITIES.len() as u32).contains(&id) {
            "Use this city's sun and moon times"
        } else { "" }
    }
}
