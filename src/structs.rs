
pub mod conf;

pub use conf::*;

pub mod input;
// pub use input::*;

use salah::NaiveDate;

use crate::{mv_dataset::TimeSetData, salah_calc::SalahCalcConfig, Rect, Screen, SettingsState};
use crate::{timeset_for_island, DEFAULT_MV_ISLAND_KEY};

const NOTIF_PRAYERS: [&str; 6] = ["Fajr", "Sunrise", "Dhuhr", "Asr", "Maghrib", "Isha"];

/// last-known layout rects for mouse support; populated each frame during draw
#[derive(Debug, Default)]
pub struct UiMouse {
    /// settings split panes: [sections, fields]
    pub settings_panes: [Rect; 2],
    /// whole popup window, for click-outside detection
    pub popup_rect: Rect,
    /// popup entry list area, for click to select/apply
    pub popup_list: Rect,
}

#[derive(Debug, Default)]
pub struct NotifState {
    pub day: Option<NaiveDate>,
    pub fired: [bool; NOTIF_PRAYERS.len()],
}

#[derive(Debug, Default)]
pub struct AppState {
    pub fullscreen: bool,
    pub prayertime: PrayerTimes,
    pub input_map: input::InputMap,
    pub input_char: char,
    pub config: Config,
    pub provider: Provider,
    pub timeset_data: TimeSetData,
    pub day_offset: i64,
    pub screen: Screen,
    pub notif: NotifState,
    pub settings: SettingsState,
    pub message: Option<String>,
    pub ui_mouse: UiMouse,
}
// Struct Provider
// - Provider
// - fn match provider => return PrayerTime struct

#[derive(Debug)]
pub enum Provider {
    DataSet(TimeSetData),
    Calculation(SalahCalcConfig),
}
impl Default for Provider {
    fn default() -> Self {
        Self::DataSet(TimeSetData::default())
    }
}
impl Provider {
    pub fn get_prayer_times(&self, date: NaiveDate) -> PrayerTimes {
        match self {
            Provider::DataSet(time_set_data) => {
                time_set_data.data_from_day(time_set_data.day_index(date))
            }
            Provider::Calculation(salah_calc_config) => {
                salah_calc_config.get_prayer_times(date)
            }
        }
    }
}


impl AppState {
    pub fn init_provider(&mut self) {
        let (provider, warning) = match self.config.provider {
            ProviderKind::Calculation => {
                let method = self.config.calculation.method.to_runtime_config();
                let madhab = self.config.calculation.madhab.to_runtime_config();
                let coordinates = self.config.coordinates.to_runtime_config();
                (Provider::Calculation(SalahCalcConfig::new(method, madhab, coordinates)), None)
            }
            ProviderKind::SalatMv => {
                match timeset_for_island(&self.config.salatmv.island) {
                    Some(timeset) => (Provider::DataSet(timeset), None),
                    None => {
                        // unknown island in config: fall back instead of crashing
                        let requested = self.config.salatmv.island.clone();
                        match timeset_for_island(DEFAULT_MV_ISLAND_KEY) {
                            Some(timeset) => (
                                Provider::DataSet(timeset),
                                Some(format!("unknown island \"{requested}\", using {DEFAULT_MV_ISLAND_KEY}")),
                            ),
                            None => {
                                let method = Method::default().to_runtime_config();
                                let madhab = Madhab::default().to_runtime_config();
                                let coordinates = Coords::default().to_runtime_config();
                                (
                                    Provider::Calculation(SalahCalcConfig::new(method, madhab, coordinates)),
                                    Some("salatmv data unavailable".to_string()),
                                )
                            }
                        }
                    }
                }
            }
        };
        self.provider = provider;
        if warning.is_some() {
            self.message = warning;
        }
    }
    pub fn get_prayer_times(&self) -> PrayerTimes{
        let date: NaiveDate = self.get_offset_date();
        self.provider.get_prayer_times(date)
    }

    pub fn get_offset_date(&self) -> chrono::NaiveDate {
        chrono::offset::Local::now().date_naive() + chrono::Duration::days(self.day_offset)
    }

    pub fn check_notifications(&mut self) {
        if !self.config.notifications.enabled {
            return;
        }
        let today = chrono::offset::Local::now().date_naive();
        let now_min = local_now_minutes() as i64;
        let offset = self.config.notifications.offset as i64;
        let minutes = self.provider.get_prayer_times(today).to_vec();

        let mut targets = [0i64; NOTIF_PRAYERS.len()];
        for (target, minute) in targets.iter_mut().zip(minutes) {
            *target = minute as i64 + offset;
        }

        if self.notif.day != Some(today) {
            self.notif.day = Some(today);
            self.notif.fired = [false; NOTIF_PRAYERS.len()];
            for (i, target) in targets.iter().enumerate() {
                if now_min >= *target {
                    self.notif.fired[i] = true;
                }
            }
        }

        for i in due_notifications(now_min, targets, self.notif.fired) {
            send_notification(NOTIF_PRAYERS[i]);
            self.notif.fired[i] = true;
        }
    }
}

fn local_now_minutes() -> u32 {
    use chrono::Timelike;
    let now = chrono::offset::Local::now();
    now.hour() * 60 + now.minute()
}

fn due_notifications(now_min: i64, targets: [i64; NOTIF_PRAYERS.len()], fired: [bool; NOTIF_PRAYERS.len()]) -> Vec<usize> {
    (0..NOTIF_PRAYERS.len())
        .filter(|&i| !fired[i] && now_min >= targets[i])
        .collect()
}

fn send_notification(prayer: &str) {
    if let Err(err) = notify_rust::Notification::new()
        .summary("salatui")
        .body(&format!("Time for {prayer}"))
        .show()
    {
        eprintln!("notification failed: {err}");
    }
}


use serde::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrayerTimes {
    pub index: u32,
    pub day: u32,
    pub fajr: u32,
    pub sun: u32,
    pub dhuhur: u32,
    pub asr: u32,
    pub magrib: u32,
    pub isha: u32,
}

impl PrayerTimes {
    pub fn from_vec(list:Vec<u32>) -> PrayerTimes {
        PrayerTimes {
            index: list[0],
            day:   list[1],
            fajr:  list[2],
            sun:   list[3],
            dhuhur:list[4],
            asr:   list[5],
            magrib:list[6],
            isha:  list[7],
        }
    }
    
    pub fn to_vec(&self) -> Vec<u32> {
        vec![
            // self.index,
            // self.day,
            self.fajr,
            self.sun,
            self.dhuhur,
            self.asr,
            self.magrib,
            self.isha,
        ]
    }
    
    pub fn get_current_index(&self) -> usize {
        use chrono::Timelike;
        let current_time = chrono::offset::Local::now();
        let minute = current_time.hour() * 60 + current_time.minute();
        
        self.to_vec().into_iter().position(|x| x > minute).unwrap_or(0)
    }
    
    pub fn format(&self, config: &Config) -> Vec<String> {
        let mut data_list: Vec<String> = vec![self.index.to_string(), self.day.to_string()];
        data_list.append(&mut self.format_time(config));
        data_list
    }
    
    pub fn format_time(&self, config: &Config) -> Vec<String> {
        let time_list: Vec<u32> = self.to_vec();
        
        match config.display.format {
            TimeFormat::Twelve =>
                time_list.iter().map(to_time).map(|t| format!("{:0>2}:{:0>2} {}",{if t.0 >12{t.0%12}else{t.0}},t.1, {if t.0 > 11{"PM"} else{"AM"} })).collect(),
            TimeFormat::TwentyFour =>
                time_list.iter().map(to_time).map(|t| format!("{:0>2}:{:0>2}",t.0,t.1)).collect(),
            TimeFormat::Minutes =>
                time_list.iter().map(|t| t.to_string()).collect(),
        }
    }
    
    
    pub fn output_format(&self, config: &Config) -> String {
        // use serde_json::to_writer_pretty;
        let mut time_list: Vec<String> = self.format(config);
        let outconf = &config.raw_output;
        
        let current = self.get_current_index().to_string();
        time_list.append(&mut vec![current]);
        
        match config.raw_output.mode {
            RawOutputMode::Array   => format!("{:?}", time_list),
            RawOutputMode::Custom  => self.format_custom_string(config),
            RawOutputMode::TOML    => self.to_toml(config),
            RawOutputMode::PrettyJson => to_json(time_list, true),
            RawOutputMode::Json    => to_json(time_list, false),
            RawOutputMode::RawData => {
                let mut string = "".to_owned();
                for time in time_list.iter() {
                    if time == time_list.last().unwrap(){
                        string.push_str(time);
                        continue;
                    }
                    string.push_str(&format!("{}{}",time,outconf.raw_separator))
                }
                string
            }
        }
        // let 
    }

    /// expand the raw_output `custom_string` template.
    ///
    /// tokens are `%` + code + optional format suffix. codes: `f s d a m i`
    /// (prayer times), `c` (current index), `e` (entry index), `y` (day).
    /// a bare code yields the full time in `display.format`; a suffix is
    /// `h`(12h hour) `H`(24h hour) `m`(minutes) `p`(AM/PM) `M`(raw minutes)
    /// `t`(full time); any other char is emitted literally. `%` followed by an
    /// unknown or missing code is passed through literally.
    pub fn format_custom_string(&self, config: &Config) -> String {
        let custom = &config.raw_output.custom_string;
        let mut out = String::new();
        let mut rest = custom.as_str();
        while let Some(idx) = rest.find('%') {
            out.push_str(&rest[..idx]);
            rest = &rest[idx + 1..];
            let Some(code) = rest.chars().next() else {
                out.push('%');
                break;
            };
            rest = &rest[code.len_utf8()..];
            let value = match code {
                'f' => Some(CustomValue::Time(self.fajr)),
                's' => Some(CustomValue::Time(self.sun)),
                'd' => Some(CustomValue::Time(self.dhuhur)),
                'a' => Some(CustomValue::Time(self.asr)),
                'm' => Some(CustomValue::Time(self.magrib)),
                'i' => Some(CustomValue::Time(self.isha)),
                'c' => Some(CustomValue::Text(self.get_current_index().to_string())),
                'e' => Some(CustomValue::Text(self.index.to_string())),
                'y' => Some(CustomValue::Text(self.day.to_string())),
                _ => None,
            };
            let Some(value) = value else {
                out.push('%');
                out.push(code);
                continue;
            };
            let end = rest.find('%').unwrap_or(rest.len());
            let suffix = &rest[..end];
            rest = &rest[end..];
            match value {
                CustomValue::Time(minutes) => out.push_str(&apply_suffix(minutes, suffix, config)),
                CustomValue::Text(text) => {
                    out.push_str(&text);
                    out.push_str(suffix);
                }
            }
        }
        out.push_str(rest);
        out
    }

    /// TOML output: `index`/`day`/`current` as bare integers, prayer times as
    /// bare strings formatted per `display.format`.
    pub fn to_toml(&self, config: &Config) -> String {
        let t = self.format_time(config);
        format!(
            "index = {}\nday = {}\nfajr = \"{}\"\nsun = \"{}\"\ndhuhur = \"{}\"\nasr = \"{}\"\nmagrib = \"{}\"\nisha = \"{}\"\ncurrent = {}",
            self.index,
            self.day,
            t[0],
            t[1],
            t[2],
            t[3],
            t[4],
            t[5],
            self.get_current_index(),
        )
    }
}

fn to_time(minutes: &u32) -> (u32, u32){
    (minutes / 60, minutes % 60)
}

enum CustomValue {
    Time(u32),
    Text(String),
}

/// 12-hour clock hour, matching the `Twelve` display format (0 stays 0)
fn hour_12(minutes: u32) -> u32 {
    let h = minutes / 60;
    if h > 12 { h % 12 } else { h }
}

/// one prayer time in the current `display.format`
fn format_minute(minutes: u32, config: &Config) -> String {
    let (hour, minute) = to_time(&minutes);
    match config.display.format {
        TimeFormat::Twelve => format!(
            "{:0>2}:{:0>2} {}",
            hour_12(minutes),
            minute,
            if hour > 11 { "PM" } else { "AM" }
        ),
        TimeFormat::TwentyFour => format!("{:0>2}:{:0>2}", hour, minute),
        TimeFormat::Minutes => minutes.to_string(),
    }
}

fn apply_suffix(minutes: u32, suffix: &str, config: &Config) -> String {
    if suffix.is_empty() {
        return format_minute(minutes, config);
    }
    let hour = minutes / 60;
    let mut out = String::new();
    for c in suffix.chars() {
        match c {
            'h' => out.push_str(&format!("{:0>2}", hour_12(minutes))),
            'H' => out.push_str(&format!("{:0>2}", hour)),
            'm' => out.push_str(&format!("{:0>2}", minutes % 60)),
            'p' => out.push_str(if hour > 11 { " PM" } else { " AM" }),
            'M' => out.push_str(&minutes.to_string()),
            't' => out.push_str(&format_minute(minutes, config)),
            other => out.push(other),
        }
    }
    out
}

fn to_json(time_list: Vec<String>, pretty: bool) -> String {
    if pretty {
    format!(
"{{
  \"index\":\"{}\",
  \"day\":\"{}\",
  \"fajr\":\"{}\",
  \"sun\":\"{}\",
  \"dhuhur\":\"{}\",
  \"asr\":\"{}\",
  \"magrib\":\"{}\",
  \"isha\":\"{}\",
  \"current\":\"{}\"
}}",
time_list[0],
time_list[1],
time_list[2],
time_list[3],
time_list[4],
time_list[5],
time_list[6],
time_list[7],
time_list[8],
)
    } else {
    format!("{{\"index\":\"{}\",\"day\":\"{}\",\"fajr\":\"{}\",\"sun\":\"{}\",\"dhuhur\":\"{}\",\"asr\":\"{}\",\"magrib\":\"{}\",\"isha\":\"{}\",\"current\":\"{}\"}}",
            time_list[0],
            time_list[1],
            time_list[2],
            time_list[3],
            time_list[4],
            time_list[5],
            time_list[6],
            time_list[7],
            time_list[8],
            )
    }
}

#[test]
fn test_format() {
    let value = PrayerTimes { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let mut config = Config::default();
    
    config.display.format = TimeFormat::Twelve;
    let expected:Vec<String> = vec!["77", "225", "04:53 AM", "06:05 AM", "12:16 PM", "03:32 PM", "06:18 PM", "07:31 PM"]
        .into_iter().map(|x|x.to_owned()).collect();
    let result = value.format(&config);
    
    config.display.format = TimeFormat::TwentyFour;
    let expected2:Vec<String> = vec!["77", "225", "04:53", "06:05", "12:16", "15:32", "18:18", "19:31"]
        .into_iter().map(|x|x.to_owned()).collect();
    let result2 = value.format(&config);

    assert_eq!(expected, result);
    assert_eq!(expected2,result2);
}

#[test]
fn test_custom_format_default_string() {
    let value = PrayerTimes { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let mut config = Config::default();
    config.display.format = TimeFormat::Twelve;
    config.raw_output.mode = RawOutputMode::Custom;
    config.raw_output.custom_string = "[%fh:mp, %sh:mp, %dh:mp, %ah:mp, %mh:mp, %ih:mp]".into();
    assert_eq!(
        value.output_format(&config),
        "[04:53 AM, 06:05 AM, 12:16 PM, 03:32 PM, 06:18 PM, 07:31 PM]"
    );
}

#[test]
fn test_custom_format_tokens() {
    let value = PrayerTimes { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let mut config = Config::default();
    config.display.format = TimeFormat::TwentyFour;
    config.raw_output.mode = RawOutputMode::Custom;

    config.raw_output.custom_string = "%e-%y %fM %st %m".into();
    assert_eq!(value.output_format(&config), "77-225 293 06:05 18:18");

    config.raw_output.custom_string = "at %sh:m and %dH".into();
    assert_eq!(value.output_format(&config), "at 06:05 and 12");

    config.raw_output.custom_string = "%dH:%dm:%dp".into();
    config.display.format = TimeFormat::Minutes;
    assert_eq!(value.output_format(&config), "12:16: PM");
}

#[test]
fn test_toml_mode() {
    let value = PrayerTimes { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let mut config = Config::default();
    config.display.format = TimeFormat::TwentyFour;
    config.raw_output.mode = RawOutputMode::TOML;

    let out = value.output_format(&config);
    let mut lines = out.lines();
    assert_eq!(lines.next(), Some("index = 77"));
    assert_eq!(lines.next(), Some("day = 225"));
    assert_eq!(lines.next(), Some("fajr = \"04:53\""));
    assert_eq!(lines.next(), Some("sun = \"06:05\""));
    assert_eq!(lines.next(), Some("dhuhur = \"12:16\""));
    assert_eq!(lines.next(), Some("asr = \"15:32\""));
    assert_eq!(lines.next(), Some("magrib = \"18:18\""));
    assert_eq!(lines.next(), Some("isha = \"19:31\""));
    assert!(lines.next().unwrap().starts_with("current = "));
}

#[test]
fn test_toml_minutes_format() {
    let value = PrayerTimes { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let mut config = Config::default();
    config.display.format = TimeFormat::Minutes;
    config.raw_output.mode = RawOutputMode::TOML;

    let out = value.output_format(&config);
    assert!(out.contains("fajr = \"293\""));
    assert!(out.contains("isha = \"1171\""));
}

#[test]
fn test_due_notifications() {
    let times = [293u32, 365, 736, 932, 1098, 1171];
    let targets = |offset: i64| [
        times[0] as i64 + offset, times[1] as i64 + offset, times[2] as i64 + offset,
        times[3] as i64 + offset, times[4] as i64 + offset, times[5] as i64 + offset,
    ];

    assert_eq!(due_notifications(800, targets(0), [false; 6]), vec![0, 1, 2]);
    assert_eq!(due_notifications(800, targets(0), [true, false, false, false, false, false]), vec![1, 2]);
    assert_eq!(due_notifications(290, targets(-10), [false; 6]), vec![0]);
    assert_eq!(due_notifications(290, targets(0), [false; 6]), Vec::<usize>::new());
    assert_eq!(due_notifications(1300, targets(0), [false; 6]), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_prayertime() {
    let expected = PrayerTimes { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let result = PrayerTimes::from_vec(vec![77, 225, 293, 365, 736, 932, 1098, 1171]);
    assert_eq!(expected, result);
}

#[test]
fn test_config_defaults() {
    let config = Config::default();
    assert!(config.display.seconds);
    assert_eq!(config.provider, ProviderKind::Calculation);
    assert_eq!(config.salatmv.island, DEFAULT_MV_ISLAND_KEY);
}





