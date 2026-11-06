# Dynamic Island Windows user guide

- [Getting started](#getting-started)
- [Open and close](#open-and-close)
- [Move it to any edge](#move-it-to-any-edge)
- [Notch or floating island](#notch-or-floating-island)
- [Notch style](#notch-style)
- [Now playing and lyrics](#now-playing-and-lyrics)
- [Timer and stopwatch](#timer-and-stopwatch)
- [Pomodoro](#pomodoro)
- [To-do with reminders](#to-do-with-reminders)
- [Water reminder](#water-reminder)
- [Prayer times](#prayer-times)
- [Sky: sun and moon](#sky-sun-and-moon)
- [Tools](#tools)
- [AI agents](#ai-agents)
- [System monitor](#system-monitor)
- [Uninstalling](#uninstalling)
- [File shelf](#file-shelf)
- [Clipboard history](#clipboard-history)
- [Mic and camera dots](#mic-and-camera-dots)
- [Windows notifications in the notch](#windows-notifications-in-the-notch)
- [Alerts and overlays](#alerts-and-overlays)
- [The mascot](#the-mascot)
- [Clock styles](#clock-styles)
- [Settings and your data](#settings-and-your-data)
- [Updates](#updates)
- [Troubleshooting](#troubleshooting)
- [Building](#building)

## Getting started

1. Run `dynamic-island-windows-<version>.exe`. Tick **Start Dynamic Island Windows when Windows starts** if you like.
2. The island opens with a short **welcome tour**: pick how it looks when closed (Full, Compact, Pill, Hidden or Floating Island), pick the screen edge, read three quick tips (hover to open, the mouse wheel switches panels, right-drag to move it), and see what's in Settings at a glance. **Open Settings** on the last step goes straight there. You can skip it, and replay it later from the tray icon's menu (**Welcome tour**). Everything you pick can be changed later in Settings.

   <img src="images/tour.png" alt="Welcome tour">

3. A tray icon (a pill with two eyes) appears near the clock.
4. To quit, right-click the tray icon and choose **Exit**, or use **Quit Dynamic Island Windows** in Settings.

## Open and close

- **Hover** the island to open it. Move the mouse away and it closes.
- **Click** the island to open it straight away.
- The icons along the top of the open panel switch between features. The **mouse wheel** does too: roll down for the next panel, up for the previous one. On a list that scrolls (to-do, clipboard, lyrics) the wheel scrolls the list; over the icons it always switches.
- It always opens on the panel you last picked, even after a restart. (A reminder can open another panel for a moment; next time it's back to yours.)
- Don't use a panel? Hide it in **Settings > Panels**.
- The smaller styles (Compact, Pill, Hidden) wait a moment before opening, so reaching for browser tabs does not open them by accident.
- The **Floating Island** shape opens by hovering or clicking anywhere on the floating pill.

<img src="images/music.png" alt="Open panel">

## Move it to any edge

<img src="images/positions.png" alt="Positions">

- **Right-click and drag** the island. It snaps to the nearest edge of whichever screen the mouse is on: top, bottom, left or right. Let go to place it.
- Or open **Settings** and pick an edge under **Position**. **Center** puts it back in the middle.
- On the left and right edges the closed island becomes a vertical tab. Panels always open upright.
- A plain right-click (without dragging) opens Settings.

## Notch or floating island

<img src="images/styles.png" alt="Shape">

Open **Settings** and pick one under **Shape**:

| Shape | Looks like | Best for |
|---|---|---|
| Notch | Attached to the screen edge, flat on top with rounded bottom corners | The classic look, flush with the edge |
| Floating Island | A fully rounded pill that floats about 10 px off the edge, with a soft shadow; it grows when you hover or an alert appears | iPhone Dynamic Island style |

Everything works in both shapes: the clock, mascot, music, timers, prayer countdown and all panels. When the island is open it expands into the same panel.

## Notch style

Open **Settings** and pick one under **Notch style**:

| Style | Looks like | Best for |
|---|---|---|
| Full | Black notch with the clock, music and timers | Seeing everything at a glance |
| Compact | A smaller notch | Less space, still glanceable |
| Pill | A thin bar, tinted while music or a timer runs | Almost invisible |
| Hidden | Nothing | Rest the mouse at the edge to open |

**Maximized windows:** with a maximized window under it (Chrome, Edge, VS Code...), the Full and Compact notch slims down to a thin bar, and clicks go straight through it, so the browser tabs under the notch stay clickable. Rest the mouse on the bar for a moment to open the notch. Alerts still drop down as usual. Turn this off with **Slim down over maximized apps** in Settings > General.

## Now playing and lyrics

<img src="images/lyrics.png" alt="Lyrics">

- Works with anything that shows in the Windows media overlay: Spotify, YouTube and other sites in Chrome, Edge or Firefox, Media Player and more.
- **Click the progress bar** to seek. Use the previous, play/pause and next buttons.
- If a player turns down a skip (some web players), Dynamic Island Windows presses the media key for you instead. The previous and next buttons look dimmed when the player says it can't skip right now (a single video, the first track). Most players restart the song when you press previous a few seconds in; press it twice to go back a track.
- **Lyrics**: click the lines button on the right. The panel splits and the current line lights up. Click a line to jump to it, or scroll to look ahead.
- In the open lyrics view you can pick the **text size** (Small / Medium / Large), turn **Auto-scroll** on or off, and turn **Highlight line** on or off.
- When music plays, the closed notch shows the album art and a small equaliser.
- Some songs have no lyrics on LRCLIB yet; the panel then says *Lyrics not available*.

Lyrics are provided by [LRCLIB](https://lrclib.net) — that's the one network request the music panel makes. Nothing else is sent.

## Timer and stopwatch

<img src="images/timer.png" alt="Timer">

**Countdown**

1. Open the **Timer** tab (the clock icon).
2. Pick a preset (1 min to 1 h) or use **-1 min** / **+1 min**. While it is stopped you can also scroll over the panel to change it by a minute.
3. Press **Start**. The time left and *Ends at* show in the panel, and a countdown ring stays in the closed notch.
4. **+1 min / -1 min** also work while it is running, without restarting.
5. When it finishes it chimes and drops an alert with **Restart** and **Dismiss**.

Dynamic Island Windows remembers your last duration.

<img src="images/stopwatch.png" alt="Stopwatch">

**Stopwatch**: switch to **Stopwatch**, then **Start**, **Lap** and **Reset**. Laps show the split and the total.

## Pomodoro

<img src="images/pomodoro.png" alt="Pomodoro">

- 25 minutes of focus, then a 5 minute break. Every fourth break is 15 minutes.
- **Start / Pause**, **Reset** and **Skip** control the current phase.
- At the end of each phase it chimes and offers **Start** for the next one, or **Later**.
- The dots show your progress through the cycle of four.

## To-do with reminders

<img src="images/todo.png" alt="To-do">

- **Add Task**, type a title, and optionally give it a time.
- Click a task to edit it. Click the circle to tick it off. Hover a task to delete it.
- When a task is due, the notch drops down with **Done** and **Snooze 10m**.
- Tasks you complete are cleared the next day. **Clear done** removes them now.

<img src="images/todo-editor.png" alt="Add a task">

**Setting a time:** tap **+15m**, **+30m**, **+1h** or **+2h** (they add up), then fine-tune with **-** and **+**. **No time** means no reminder. Press **Enter** to save or **Esc** to cancel. You can paste with **Ctrl+V**.

## Water reminder

<img src="images/water.png" alt="Water">

- **Drink** logs one sip. The arrow button undoes the last one.
- The chart shows what you drank each hour today.
- Reminders are spread evenly across your active hours until you reach your goal. Each one offers **Drank** or **Snooze**.
- When you have drunk a bottle's worth, it asks you to refill.

<img src="images/water-settings.png" alt="Water settings">

The gear button opens settings: bottle size, daily goal, sip size, active hours and reminders on or off. **Reset today** asks before clearing.

## Prayer times

<img src="images/prayer.png" alt="Prayer times">

- Today's Fajr, Sunrise, Dhuhr, Asr, Maghrib and Isha, with the next prayer, a countdown and the current prayer highlighted.
- Times are calculated offline on your PC from the sun's position for your place and date, so they follow the seasons and need **no internet**. They use Asia/Karachi (UTC+5, no daylight saving).
- The **bell** beside a prayer turns its reminder on. Reminders are off until you turn them on.
- The gear opens three pages:
  - **Location**: one of 10 Pakistani cities (Bhakkar, Lahore, Islamabad, Rawalpindi, Karachi, Faisalabad, Multan, Peshawar, Quetta, Hyderabad), defaulting to **Bhakkar**, or **Custom location** to type a latitude and longitude (for example `31.52, 74.36`) and press Enter.
  - **Method**: Karachi (University of Islamic Sciences, Fajr and Isha at 18°), MWL, ISNA, Egypt or Makkah; Shafi or Hanafi Asr (Hanafi is the default); reminders on time or 5, 10 or 15 minutes before; and a countdown in the closed island for the last 15 minutes before a prayer.
  - **Adjust**: move any prayer a minute earlier or later (up to 30) to match your mosque or calendar.
  - **Alert style**: pick how the next-prayer header animates — None, Fade, Slide, Bounce, Pop, Glow, Rainbow, Waves, Pulse, Clock tick or Star.

The calculations match published aladhan.com values to within about a minute. No custom adhan or audio file is bundled; reminders use the built-in notification sound you pick in Settings.

## Sky: sun and moon

<img src="images/sky.png" alt="Sky">

Open the **Sky** tab to see, for your chosen city (default Bhakkar, Asia/Karachi):

- **Sun**: altitude and azimuth (compass direction), sunrise and sunset times, and a countdown to the next sunrise or sunset.
- **Moon**: phase (new, crescent, quarter, gibbous, full), illumination percentage, age in days, and a countdown to the next full or new moon.

Like the prayer times, this is worked out on your PC from the date and your city's coordinates — nothing is fetched from the internet. Pick a different city with the chips at the top of the tab.

## Tools

The Tools tab has four small tools; the chips at the top switch between them. Everything works offline.

<img src="images/tools.png" alt="Color picker">

- **Color picker**: **Pick from screen**, then click anywhere. A magnifier under the cursor shows the exact pixel; arrow keys move it one pixel (Shift for ten), and Esc or a right-click cancels. The color is copied as HEX straight away; click HEX, RGB or HSL to copy that form instead. It also shows how readable text in that color is on white and on black, and keeps your last eight colors.

<img src="images/tools-calc.png" alt="Calculator">

- **Calculator**: type a sum and the answer appears as you type. `1250*15%` is 15 % of 1250, `200+10%` adds 10 %, and `2^10`, `sqrt(2)`, `sin(30)` (degrees), `5!`, `pi`, `0xFF` and `0b1010` work too. Enter keeps the result in the history (click one to reuse it), Esc clears, Ctrl+C copies the answer.

<img src="images/tools-qr.png" alt="QR code">

- **QR code**: shows what you last copied as a QR code, so a link gets to your phone with one scan. **Type text** makes one from your own text, and **Copy image** puts the code on the clipboard as a picture.

<img src="images/tools-convert.png" alt="Converter">

- **Converter**: length, weight, temperature, area, volume, speed and data. Type a number (or a sum), click the units, **Swap** flips them, and clicking the answer copies it.
- **Zones** shows the time in 12 cities, using Windows' own daylight-saving rules. Click a city to compare from it, and use the hour buttons to see "what time will it be there at 6 pm here".

## AI agents

<img src="images/agents.png" alt="AI agents">

If you use **Hermes**, **Claude Code** or **Codex CLI**, the island shows what they're doing while you're in another window.

- Open the **AI agents** tab and switch on **Hermes**, **Claude Code** and/or **Codex**. Dynamic Island Windows adds a hook to the agent's own settings: Hermes shell hooks through `hermes config set` (and Hermes' hook allowlist, so it doesn't ask again), `~/.claude/settings.json`, `~/.codex/config.toml`. Hermes picks it up within a minute; Claude Code and Codex from their next session. Switching it off removes exactly what was added, and so does uninstalling.
- **Working**: the closed island shows the agent icon and how long it has been at it, with the project folder on the other side.
- **Needs you**: when Hermes wants your OK for a command, asks a question or needs a password, or Claude Code asks for permission, the island pops up (with your chime) and turns orange until you answer.
- **Done**: a short "finished, took 3 min" notice when a turn ends. Codex reports only this one. Stopping Hermes with Esc shows Stopped, without a notice.
- The tab lists recent sessions; click one to clear it. A session that goes quiet for 20 minutes drops back to Ready.
- Privacy: the hook passes on only the event name, session id, project folder, model and tool name, and the agent's notice text. Prompts, code and tool output are never read or stored, and nothing leaves your PC. If Codex already runs its own `notify` program, Dynamic Island Windows leaves it alone and says so.

## System monitor

<img src="images/monitor.png" alt="System monitor">

- **CPU**, **memory** and **network** (download and upload) with a one-minute graph, plus free space on the Windows drive and how long the PC has been on.
- **Show on closed island** puts CPU and download speed on the closed island all the time. It's off by default.
- It reads Windows' own counters once a second, so it adds no load.

## Tools (continued): Agenda and Eye rest

### Agenda

The **Agenda** tab keeps a simple list of local calendar events and to-dos:

- **Add** an event: type a title and press **Add** (or Enter). It's saved to `agenda.txt` and stamped with "about an hour from now" by default.
- Upcoming events are listed in time order. An event starting within a minute pops a toast reminder.
- Events are kept on your PC only; nothing is sent anywhere.

### Eye rest

The **Eye rest** tab implements the 20-20-20 rule: every 20 minutes it suggests you look at something about 20 feet away for 20 seconds.

- A toast appears when a break is due, and the tab shows a live countdown to the next one.
- Turn it on or off, and change the interval, in **Settings > General**.

## File shelf

<img src="images/shelf.png" alt="Shelf">

- **Drag files, folders or browser links onto the island.** It opens the Shelf and shows *Release to add*.
- **Click** an item to open it. **Drag** it out into a chat, an email or a folder.
- Hover an item and click the **x** to remove it. **Clear** empties the shelf.
- The shelf keeps links to your files, not copies.
- **Screenshot**: the island hides and the screen freezes; drag over the part you want (or just click for the whole screen), Esc cancels. The picture is saved as a PNG in **Pictures\Screenshots**, added to the Shelf (drag it into a chat) and copied to the clipboard.

## Clipboard history

<img src="images/clipboard.png" alt="Clipboard history">

- Clipboard history is **off** until you turn it on: open the **Clipboard** tab and click **Turn on clipboard history**, or use the switch in Settings.
- **What it records** (Settings > Clipboard history): **Text** (the default), **Text + files** (files you copy in Explorer), or **Everything** (also pictures, shown as thumbnails).
- What you copy appears here, newest first. The last 30 are kept, and up to 10 pictures.
- **Search** (magnifier icon): type to filter the history; **Enter** pastes the first match, **Esc** clears, Esc again closes.
- **Snippets** (+ icon): type text you paste often (your address, an email signature) and press Enter. It's kept like a pinned entry, even with recording off.
- **Click** an entry to paste it straight into the app you were using (it presses Ctrl+V for you). Hold **Shift** while clicking to paste as plain text in apps that support Ctrl+Shift+V. Files paste into Explorer too.
- To only copy it, hover the entry and click the **copy** icon; then paste wherever you like with Ctrl+V.
- Apps running as administrator don't accept typing from normal apps, so for those the entry is only copied and the island says **press Ctrl+V**.
- Hover an entry to **copy** it without pasting, **pin** it (pinned entries stay at the top and are kept after a restart) or **remove** it. **Clear** removes everything that isn't pinned. The switch in the tab header stops and starts recording.
- **Show what I copy** (Settings > Privacy, off by default): every time you copy something, a small toast shows it for two seconds, like on Android: the text, the file names, or a thumbnail of the picture. It works even with clipboard history off and keeps nothing. Dynamic Island Windows' own copies (pasting an entry, a color, a screenshot) don't show.
- Privacy: unpinned entries and all pictures live in memory only and are gone when Dynamic Island Windows exits. Pinned text and files are saved in `clipboard.txt`. Passwords and codes copied from password managers that mark them private (KeePass, Bitwarden, 1Password and others) are never recorded.

## Mic and camera dots

- While any app is using your **microphone**, an orange dot appears beside the island; a green dot means the **camera** is on. The island also drops a short notice naming the app when it starts.
- The dots show in every island style, including Pill and Hidden, and next to the clock when the panel is open.
- Dynamic Island Windows reads this from Windows' own privacy records; it does not listen to or watch anything itself. Turn the dots off with **Mic & camera dots** in Settings.

## Windows notifications in the notch

<img src="images/notifications.png" alt="Windows notifications">

Dynamic Island Windows can show other apps' notifications (messages, mail, calendar, browser sites) in the island, with the app's icon and colour, instead of Windows' banner in the corner.

- Turn it on in **Settings > Privacy > Show them in the notch**. It's off until you do.
- Windows keeps showing its own banner too, unless you turn banners off: click **Windows banners** in the same place (or open Windows **Settings > System > Notifications**) and turn off **Show notification banners** for the apps you want in the island only. Notifications still go to Notification Center as usual.
- **Click** a notification (or **Open**) to open the app that sent it. **Mute app** stops that app's notifications appearing in the island; **Unmute** in Settings brings them all back. Buttons inside a notification (Reply, Accept) only work in Windows' own banner.
- **Include message text** off shows only the app's name, handy if you share your screen.
- Notifications that arrive while **Do Not Disturb** or focus is on, or during a full-screen game or video, wait until it's over, then the newest shows with a count of the rest ("+2 more"). Windows 11 turns focus on by itself during games and full-screen apps.
- How it works: Dynamic Island Windows asks Windows for the notifications in Notification Center, the same permission other notification tools use (Windows **Settings > Privacy & security > Notifications**). It checks only when Windows records a new one. Nothing is stored or sent anywhere.

## Alerts and overlays

<img src="images/alerts.png" alt="Alerts">

The island briefly drops down for:

- charger connected or unplugged, low battery (20%), critical battery (10%) and fully charged
- internet connected, no internet access, or disconnected
- Caps Lock on or off, and keyboard language changes
- volume changes (turn off with **Volume overlay** in Settings)
- Bluetooth devices connecting or disconnecting

## The mascot

<img src="images/mascot.png" alt="Mascot">

In the **Full** style, after a while with nothing going on, the clock turns into a little companion that follows your mouse with its eyes. After a few minutes away it dozes off, and it waves when you come back.

Pick one in **Settings > Mascot**. There are 14 characters to choose from (Clock turns the mascot off). Hover a tile to see it wave.

- **Always** shows the mascot all the time instead of waiting for a quiet moment, even next to music or a timer.
- **Left**, **Center** or **Right** places it in the island. At the left or right, the clock (or the music, timer or prayer countdown) sits beside it; the island grows a little to make room. This applies on the top and bottom edges.

The mascot uses no CPU while the island is fully asleep.

## Clock styles

Open **Settings** and pick one under **Clock**:

| Style | Looks like |
|---|---|
| 12h | 4:20 PM |
| 24h | 16:20 |
| Analog | A small clock face with hour, minute and second hands |
| Fancy | Bright digits with a soft coloured glow behind them |
| Minimal | Hours and minutes with a quiet colon |

12h / 24h is used everywhere in Dynamic Island Windows.

## Settings and your data

<img src="images/settings.png" alt="Settings">

<img src="images/settings-privacy.png" alt="Privacy and alerts">

Open Settings with the **gear** at the end of the tab bar, a plain **right-click** on the island, or a **left-click** on the tray icon. The island opens into a large settings panel. Style, position, shape, mascot and sound are on the left; the right side has three pages, **General**, **Privacy** and **Panels**:

| Item | What it does |
|---|---|
| Notch style | Full, Compact, Pill or Hidden |
| Shape | Notch or Floating Island |
| Position | Top, Bottom, Left, Right, Center |
| Mascot | Which companion shows, or Clock for none; **Always** and **Left / Center / Right** |
| Sound for reminders | Chime, Bell, Pop, Marimba, Windows (your system sound) or Off; picking one plays it |
| Clock | 12h, 24h, Analog, Fancy or Minimal |
| **General** page: Start with Windows | Launch at sign-in |
| Volume overlay | Show volume changes |
| Eye rest | 20-20-20 breaks on or off, and the interval |
| Check for updates | Looks for a newer version now |
| Check for updates automatically | Asks GitHub for the latest version about twice a day. On by default; also offered by the installer |
| Open data folder | Opens `%APPDATA%\DynamicIslandWindows` |
| GitHub page / Privacy policy / Quit Dynamic Island Windows | The privacy policy is [PRIVACY.md](../PRIVACY.md) |
| **Privacy** page: Windows notifications | Show them in the island, include message text, Windows banners, unmute apps |
| Mic & camera dots | Dots while the mic or camera is in use |
| Clipboard history | Record on/off, and Text, Text + files or Everything |
| Show what I copy | A small preview of each copy (off by default) |
| **Panels** page | Show or hide each panel. A hidden panel is off: no tab and no reminders. One always stays |

Right-clicking the tray icon shows a short menu: Settings, Welcome tour, Check for updates, About and Exit. Resting the mouse on any icon button shows what it does.

Everything is stored as plain text in `%APPDATA%\DynamicIslandWindows` (`settings.txt`, `todo.txt`, `water.txt`, `prayer.txt`, `sky.txt`, `agenda.txt`, `tools.txt`, `shelf.txt`, `clipboard.txt`). Uninstalling from **Settings > Apps** asks whether to delete it too.

## Uninstalling

- **Settings > Apps > Dynamic Island Windows > Uninstall**, as usual, or **Uninstall Dynamic Island Windows...** in the tray icon's menu.
- The tray item works even where Windows blocks the unsigned uninstaller (Smart App Control). It asks whether to keep your data, closes Dynamic Island Windows, takes it out of startup, disconnects Claude Code and Codex, then removes the app. From a command line: `dynamic-island-windows.exe --uninstall` (`--keep-data`, `--quiet`).
- Screenshots in Pictures\Screenshots are never deleted.

## Updates

Dynamic Island Windows checks GitHub for a new version shortly after it starts and then twice a day. When there is one, the island shows **Update available**:

- **Update** downloads the installer, upgrades in place and starts the new version. Your data and settings stay.
- **Later** asks again at the next check.
- **Skip** ignores that version (newer ones are still offered).

To check yourself, open Settings and click **Check for updates**. The check only asks GitHub for the latest release number; nothing about you is sent. If GitHub is busy or you're offline, it tries again within the hour. Before running a downloaded update, Dynamic Island Windows checks it against the fingerprint (SHA-256) GitHub publishes for it.

## Troubleshooting

- **Nothing in Now playing.** Make sure the player shows up in the Windows media overlay (press a media key and look at the volume flyout).
- **The island disappeared.** It hides while a full-screen game, video or presentation is running, and comes back as soon as you leave it. Reminders that arrive meanwhile wait and show (with their sound) when it returns.
- **It opens when I reach for my tabs.** Switch to the Compact, Pill or Hidden style, or move it to another edge.
- **Found a bug or have an idea?** Open an [issue](https://github.com/malikhassan1555/WindyLand/issues/new/choose); the form asks for what helps.
- **Lyrics not available.** The song is not on LRCLIB yet, or the title from the player is too unusual to match.

## Building

Needs Rust (stable, GNU or MSVC toolchain). The release build produces a single portable executable.

```powershell
cargo build --release                                       # target\release\dynamic-island-windows.exe
.\target\release\dynamic-island-windows.exe --selftest      # headless checks
.\build.ps1                                                 # build + icon + self-test + installer (Inno Setup 6)
```

A fresh-data-folder self-test (`WINDYLAND_DATA_DIR` pointed at an empty folder) should report OK across the board; the only expected failures are the frame-rate timing note and the GitHub release check when this fork has no published releases.

The installer needs **Inno Setup 6**.
