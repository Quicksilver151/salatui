# salatui

A TUI application to show and manage islamic prayer times.

## Features

- [x] Prayer time calculation (salah crate backend)
- [x] Config model + load/save (confy)
- [x] Notifications (notify-rust; global offset)
- [x] CLI output mode (`--output`; array/json/pretty-json formats)
- [ ] Raw output: custom + TOML modes
- [x] TUI salat display (menu screen: prayer list, current-prayer indicator, day/month navigation, clock seconds toggle, location line)
- [x] Built-in salatmv dataset (embedded via build.rs statics; island picker in settings; parse-on-demand into memory)
- [ ] Website source (salat.com, future)

## File organisation

- `main.rs` — entry point, event loop (`run_app`), CLI output path
- `parsers/args.rs` — clap arg parsing
- `structs.rs` — runtime state (`AppState`, `Provider`, `PrayerTimes`, notifications)
  - `structs/conf.rs` — config models (serde/confy)
  - `structs/input.rs` — input maps
- `backends/`
  - `salah_calc.rs` — prayer time calculation via salah crate
  - `mv_dataset.rs` — embedded salatmv dataset (build.rs statics) + island/timeset lookup; legacy disk load/save kept unused
  - `salah_com.rs` — salah.com website source (future)
- `ui.rs` — frame dispatch per screen; layout containers
  - `ui/menu.rs`, `ui/settings/` — implemented screens
  - `ui/calender.rs` — stub
  - `ui/structs.rs` — UIState, screen rects
  - `ui/utils.rs` — helpers

## TODO

### provider
- [x] built in salatmv dataset (build.rs embeds csvs as statics; `timeset_for_island` parses the island's 366-row year into memory on demand; island picker popup)
- [x] fix day index error (366-row tables: non-leap years skip the Feb 29 slot; legacy 365-row tables compress leap years)
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
- [x] settings window phase 2 (popups: location picker over `cities` crate's 10k embedded world cities w/ live filter, island picker over embedded salatmv islands; picking a city sets location + coordinates)
- [x] flattened provider schema (`provider` kind selector + always-stored `[calculation]` / `[coordinates]` / `[salatmv]` sections; per-provider locations persist across switches and restarts)

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

- Dataset rows: salatmv timesets hold 366 rows (leap-year layout, Feb 29 slot at row 59). `TimeSetData::day_index()`: non-leap years on 366-row tables skip the Feb 29 slot (+1 after index 58); legacy 365-row tables compress leap years (−1 after index 59).
- Island keys: `"<atoll>. <island>"` (e.g. `"K. Male'"`); atoll references in islands data are 1-based ids into ATOLL_DATA. Keys are unique across all 205 islands.
- Provider schema: flat config — `provider` selects the active kind; `[calculation]`, `[coordinates]`, `[salatmv]` all persist independently. Runtime assembles them into a `Provider` (`SalahCalcConfig` / in-memory `TimeSetData`); unknown island falls back to `K. Male'` with a settings message.
- Timezone: system-local only, no config option. Display and internal logic share `Local::now()`.
- Notifications: notify-rust (dbus on linux); failures print to stderr, which is invisible under the TUI.
- Data flow: UI receives data and renders only; all processing/state mutation happens in `run_app` (`main.rs`).
- Storage: confy — config at `~/.config/salatui/` (`config-dev.toml` in dev builds). salatmv data is embedded in the binary via build.rs; legacy disk datasets at `~/.local/share/salatui/` (load/save kept but unused).
