# salatui

A TUI application to show and manage islamic prayer times.

## Features

- [x] Prayer time calculation (salah crate backend)
- [x] Config model + load/save (confy)
- [x] Notifications (notify-rust; global offset)
- [x] CLI output mode (`--output`; array/json/pretty-json formats)
- [ ] Raw output: custom + TOML modes
- [x] TUI salat display (menu screen: prayer list, current-prayer indicator, day/month navigation)
- [ ] Built-in MV dataset (import pipeline missing)
- [ ] Website source (salat.com, future)

## File organisation

- `main.rs` — entry point, event loop (`run_app`), CLI output path
- `parsers/args.rs` — clap arg parsing
- `structs.rs` — runtime state (`AppState`, `Provider`, `PrayerTimes`, notifications)
  - `structs/conf.rs` — config models (serde/confy)
  - `structs/input.rs` — input maps
- `backends/`
  - `salah_calc.rs` — prayer time calculation via salah crate
  - `mv_dataset.rs` — salat-mv dataset storage + csv parsing
  - `salah_com.rs` — salah.com website source (future)
- `ui.rs` — frame dispatch per screen; layout containers
  - `ui/menu.rs`, `ui/settings/` — implemented screens
  - `ui/calender.rs` — stub
  - `ui/structs.rs` — UIState, screen rects
  - `ui/utils.rs` — helpers

## TODO

### provider
- [ ] built in salat-mv dataset (runtime side ported to backends/mv_dataset.rs; csv -> stored dataset import pipeline missing)
- [x] fix day 366 error (leap year)
- [x] calculation methods (salah crate backend)

### display
- [ ] calender
- [ ] ux
  - [x] switch days
  - [x] shift + left/right to switch months
- [ ] indicator variants (TimeIndicator enum has 4 variants; only Next/Current handled distinctly in menu)

### config
- [x] model the config data
- [x] load/save config
- [x] load/save data
- [x] settings window phase 1 (two-pane editor; all 4 sections; toggle/cycle/text editing; coords censored until edited; autosave + live provider reload)
- [ ] settings window phase 2 (popups: location picker w/ embedded city list, dataset picker)

### notifs
- [x] universal notif (notify-rust: linux dbus / windows / macos)
- [x] global notif offset (minutes, negative = early reminder)
- [ ] per-prayer offsets
- [ ] surface notification errors in ui (stderr is invisible under tui)

### general
- [x] make a separate current datetime for display and notifs/internal logic
- [ ] better input handling (currently inputmap is useless as input is used directly) (maybe use enum?)

### optimisations
- [ ] better data parsing

## Design notes

- Leap years: dataset rows are fixed at 365; `TimeSetData::day_index()` shifts post-Feb-29 dates back one slot instead of indexing out of bounds.
- Timezone: system-local only, no config option. Display and internal logic share `Local::now()`.
- Notifications: notify-rust (dbus on linux); failures print to stderr, which is invisible under the TUI.
- Data flow: UI receives data and renders only; all processing/state mutation happens in `run_app` (`main.rs`).
- Storage: confy — config at `~/.config/salatui/` (`config-dev.toml` in dev builds), datasets at `~/.local/share/salatui/`.
