<p align="center"><img src="docs/images/hero.png" alt="Dynamic Island Windows" width="100%"></p>

<p align="center"><a href="docs/USER_GUIDE.md">User guide</a> · <a href="PRIVACY.md">Privacy policy</a> · <a href="LICENSE">License</a></p>

# Dynamic Island Windows

A floating island (or notch) for Windows 10 and 11, like the iPhone Dynamic
Island. Music with synced lyrics, Windows notifications, prayer times, sun and
moon, timers, Pomodoro, to-do, water reminder, file shelf, clipboard history
and mic and camera in-use dots — all in a small pill at the edge of your screen.
Hover to open, move away to close. Free and open source.

<p align="center"><img src="docs/images/demo.gif" alt="Dynamic Island Windows demo: notch and island styles, music and lyrics, every tab, notifications, settings" width="900"></p>

This is a fork of [WindyLand by Shabab](https://github.com/shabab239/WindyLand),
originally inspired by [Nusa](https://rama2555.github.io/nusa/) by rama2555.
The original LICENSE (MIT) is kept and the original author is credited below.

## Features

| | |
|---|---|
| <img src="docs/images/lyrics.png" alt="Synced lyrics"> | <img src="docs/images/notifications.png" alt="Windows notifications"> |
| **Now playing + synced lyrics** from any media app | **Windows notifications** in the notch, with the app's icon |
| <img src="docs/images/timer.png" alt="Timer"> | <img src="docs/images/pomodoro.png" alt="Pomodoro"> |
| **Timer and stopwatch** with presets, alerts and laps | **Pomodoro** focus sessions |
| <img src="docs/images/todo.png" alt="To-do"> | <img src="docs/images/water.png" alt="Water reminder"> |
| **To-do** with timed reminders | **Water reminder** with an hourly chart |
| <img src="docs/images/shelf.png" alt="File shelf"> | <img src="docs/images/clipboard.png" alt="Clipboard history"> |
| **File shelf**: drop in, drag out | **Clipboard history**: text, files or images, pin to keep |
| <img src="docs/images/tools.png" alt="Tools"> | <img src="docs/images/prayer.png" alt="Prayer times"> |
| **Tools**: color picker, calculator, QR codes, unit and time-zone converter | **Prayer times** worked out on your PC, with reminders |
| <img src="docs/images/agents.png" alt="AI agents"> | <img src="docs/images/monitor.png" alt="System monitor"> |
| **AI agents**: Hermes, Claude Code and Codex working, waiting for you, done | **System monitor**: CPU, memory, network, also on the closed notch |
| <img src="docs/images/agenda.png" alt="Agenda"> | <img src="docs/images/eye-rest.png" alt="Eye rest"> |
| **Agenda**: local calendar events and reminders | **Eye rest**: 20-20-20 breaks every 20 minutes |
| <img src="docs/images/sky.png" alt="Sky"> | <img src="docs/images/styles.png" alt="Notch styles"> |
| **Sky**: sun altitude/azimuth, sunrise/sunset, moon phase & illumination | **Notch or floating island**: pill, shadow, 10 px gap, grow on hover |

Battery, network, Caps Lock, keyboard language, volume and Bluetooth alerts are
built in, and a dot shows while any app uses your microphone or camera.

**Prayer times** are calculated offline on your PC (no internet request), for
10 Pakistani cities (Bhakkar, Lahore, Islamabad, Rawalpindi, Karachi,
Faisalabad, Multan, Peshawar, Quetta, Hyderabad) or any latitude and longitude,
defaulting to Bhakkar in Asia/Karachi. Uses the University of Islamic Sciences,
Karachi method (Fajr and Isha at 18°), Hanafi or Standard Asr, per-prayer
minute nudges and optional reminders. Choose from 10 prayer-alert animations
(None plus Fade, Slide, Bounce, Pop, Glow, Rainbow, Waves, Pulse, Clock tick,
Star).

Hide any panel you don't use in Settings > Panels, and roll the mouse wheel on
the open notch to switch panels.

**Mascots**: friendly eyes that follow your mouse, blink, doze and wave hello,
with a Settings switch and 14 characters to choose from. **Clock styles**:
digital 12h/24h, analog, fancy glow digits, minimal. **Lyrics**: choose text
size, auto-scroll and current-line highlighting.

Your data stays on your PC. The only network requests are to
[LRCLIB](https://lrclib.net) for lyrics and to GitHub to check for updates.
Notifications, clipboard history and the mic and camera dots are read
locally, and notifications and clipboard history are off until you turn them
on. See the [privacy policy](PRIVACY.md).

## Install

Download `dynamic-island-windows-<version>.exe` from the
[Releases](https://github.com/malikhassan1555/WindyLand/releases) page and run
it. It installs per user, no admin rights needed. A short welcome tour lets you
pick the look.

**Quick start:** hover the notch/island to open it, right-click it for
settings, right-click and drag to move it. See the **[User
guide](docs/USER_GUIDE.md)**.

**"Smart App Control blocked an app"?** Releases aren't code signed, and
Windows 11's Smart App Control blocks unsigned apps with no way to allow just
one. The only way to install on such a PC is to turn Smart App Control off in
Windows Security > App & browser control. Note that on many
Windows versions it can't be turned back on without resetting Windows. The
blue "Windows protected your PC" screen is different: click **More info** >
**Run anyway**.

## Build

Needs Rust (stable, GNU or MSVC toolchain). The release build produces a
single portable executable.

```powershell
cargo build --release                                   # target\release\dynamic-island-windows.exe
.\target\release\dynamic-island-windows.exe --selftest  # headless checks
.\build.ps1                                             # build + icon + self-test + installer (Inno Setup 6)
python docs\tools\make_images.py                        # regenerate screenshots (Pillow, numpy)
```

A fresh-data-folder self-test (`WINDYLAND_DATA_DIR` pointed at an empty folder)
should report OK across the board; the only expected failures are the
frame-rate timing note and the GitHub release check when this fork has no
published releases.

Releases are built by GitHub Actions from the version tag
(`.github/workflows/release.yml`) and need **Inno Setup 6** for the installer.

## Adding a feature

Each feature is one file in `src/modules/` implementing the `Module` trait,
registered in `modules::all()`. Mascots live in `src/mascot.rs`: add a variant
to `store::Mascot` and a draw function. Keep the app light: draw only when
something moves, no CPU when idle.

## Credits

Dynamic Island Windows is a fork of WindyLand, written by
[Shabab](https://github.com/shabab239) and originally inspired by Nusa by
rama2555. The MIT licence and original LICENSE file are kept; Shabab is
credited as the original author. This fork is maintained by
[Malik Hassan](https://github.com/malikhassan1555).

## License

[MIT](LICENSE)
