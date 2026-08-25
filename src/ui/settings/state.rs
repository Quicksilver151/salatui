use crate::*;

pub const CATEGORIES: [&str; 4] = ["Provider", "Display", "Notifications", "Raw Output"];

const METHOD_VARIANTS: [Method; 12] = [
    Method::MuslimWorldLeague,
    Method::Egyptian,
    Method::Karachi,
    Method::UmmAlQura,
    Method::Dubai,
    Method::MoonsightingCommittee,
    Method::NorthAmerica,
    Method::Kuwait,
    Method::Qatar,
    Method::Singapore,
    Method::Tehran,
    Method::Turkey,
];
const MADHAB_VARIANTS: [Madhab; 2] = [Madhab::Shafi, Madhab::Hanafi];
const UIMODE_VARIANTS: [UIMode; 3] = [UIMode::Normal, UIMode::Fancy, UIMode::Text];
const TIMEFORMAT_VARIANTS: [TimeFormat; 3] = [TimeFormat::Twelve, TimeFormat::TwentyFour, TimeFormat::Minutes];
const INDICATOR_VARIANTS: [TimeIndicator; 4] = [
    TimeIndicator::Empty,
    TimeIndicator::Current,
    TimeIndicator::Inbetween,
    TimeIndicator::Next,
];
const LOCATION_VARIANTS: [LocationDisplay; 4] = [
    LocationDisplay::Hide,
    LocationDisplay::Country,
    LocationDisplay::City,
    LocationDisplay::CityCountry,
];
const RAWMODE_VARIANTS: [RawOutputMode; 6] = [
    RawOutputMode::Array,
    RawOutputMode::Custom,
    RawOutputMode::PrettyJson,
    RawOutputMode::Json,
    RawOutputMode::RawData,
    RawOutputMode::TOML,
];

#[derive(Debug, Default)]
pub struct SettingsState {
    pub category: usize,
    pub cursor: usize,
    pub offset: usize,
    pub mode: SettingsMode,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum SettingsMode {
    #[default]
    Normal,
    TextInput { field: FieldId, buffer: String },
    Popup { kind: PopupKind, cursor: usize, offset: usize, filter: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupKind {
    Location,
    Island,
}

#[derive(Debug)]
pub enum PopupEntry {
    City(&'static [&'static str; 4]),
    Island(String),
}

/// case-insensitive subsequence match ("ml'" finds "Male'")
fn fuzzy(haystack: &str, needle: &str) -> bool {
    let hay = haystack.to_lowercase();
    let mut hay = hay.chars();
    needle.to_lowercase().chars().all(|n| hay.any(|c| c == n))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldId {
    ProviderType,
    ProviderLocation,
    ProviderMethod,
    ProviderMadhab,
    ProviderLatitude,
    ProviderLongitude,
    Island,

    UiMode,
    TimeFormat,
    Indicator,
    Fullscreen,
    Seconds,
    Location,

    NotifEnabled,
    NotifOffset,

    RawMode,
    RawPool,
    RawSeparator,
    RawCustomString,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    Toggle,
    Cycle,
    Text { numeric: bool },
    Pick(PopupKind),
}

#[derive(Debug)]
pub struct FieldRow {
    pub id: FieldId,
    pub label: &'static str,
    pub kind: FieldKind,
}

pub fn fields_for(category: usize, config: &Config) -> Vec<FieldRow> {
    match category {
        0 => {
            let mut rows = vec![FieldRow {
                id: FieldId::ProviderType,
                label: "type",
                kind: FieldKind::Cycle,
            }];
            match config.provider {
                ProviderKind::Calculation => rows.extend([
                    FieldRow { id: FieldId::ProviderLocation, label: "location", kind: FieldKind::Pick(PopupKind::Location) },
                    FieldRow { id: FieldId::ProviderMethod, label: "method", kind: FieldKind::Cycle },
                    FieldRow { id: FieldId::ProviderMadhab, label: "madhab", kind: FieldKind::Cycle },
                    FieldRow { id: FieldId::ProviderLatitude, label: "latitude", kind: FieldKind::Text { numeric: true } },
                    FieldRow { id: FieldId::ProviderLongitude, label: "longitude", kind: FieldKind::Text { numeric: true } },
                ]),
                ProviderKind::SalatMv => rows.extend([FieldRow {
                    id: FieldId::Island,
                    label: "island",
                    kind: FieldKind::Pick(PopupKind::Island),
                }]),
            }
            rows
        }
        1 => vec![
            FieldRow { id: FieldId::UiMode, label: "ui mode", kind: FieldKind::Cycle },
            FieldRow { id: FieldId::TimeFormat, label: "time format", kind: FieldKind::Cycle },
            FieldRow { id: FieldId::Indicator, label: "indicator", kind: FieldKind::Cycle },
            FieldRow { id: FieldId::Fullscreen, label: "start fullscreen", kind: FieldKind::Toggle },
            FieldRow { id: FieldId::Seconds, label: "show seconds", kind: FieldKind::Toggle },
            FieldRow { id: FieldId::Location, label: "location", kind: FieldKind::Cycle },
        ],
        2 => vec![
            FieldRow { id: FieldId::NotifEnabled, label: "enabled", kind: FieldKind::Toggle },
            FieldRow { id: FieldId::NotifOffset, label: "offset (min)", kind: FieldKind::Text { numeric: true } },
        ],
        3 => vec![
            FieldRow { id: FieldId::RawMode, label: "mode", kind: FieldKind::Cycle },
            FieldRow { id: FieldId::RawPool, label: "pool", kind: FieldKind::Toggle },
            FieldRow { id: FieldId::RawSeparator, label: "separator", kind: FieldKind::Text { numeric: false } },
            FieldRow { id: FieldId::RawCustomString, label: "custom string", kind: FieldKind::Text { numeric: false } },
        ],
        _ => vec![],
    }
}

impl FieldId {
    pub fn is_provider(self) -> bool {
        matches!(
            self,
            FieldId::ProviderType
                | FieldId::ProviderLocation
                | FieldId::ProviderMethod
                | FieldId::ProviderMadhab
                | FieldId::ProviderLatitude
                | FieldId::ProviderLongitude
                | FieldId::Island
        )
    }

    pub fn censored(self) -> bool {
        matches!(self, FieldId::ProviderLatitude | FieldId::ProviderLongitude)
    }

    pub fn steppable(self) -> bool {
        matches!(self, FieldId::NotifOffset)
    }

    pub fn step(self, config: &mut Config, delta: isize) {
        if self == FieldId::NotifOffset {
            let next = config.notifications.offset as isize + delta;
            config.notifications.offset = next.clamp(i32::MIN as isize, i32::MAX as isize) as i32;
        }
    }

    pub fn value_str(self, config: &Config) -> String {
        match self {
            FieldId::ProviderType => config.provider.label().to_string(),
            FieldId::ProviderLocation => config.calculation.location.clone(),
            FieldId::ProviderMethod => method_name(&config.calculation.method).to_string(),
            FieldId::ProviderMadhab => format!("{:?}", config.calculation.madhab),
            FieldId::ProviderLatitude => config.coordinates.latitude.to_string(),
            FieldId::ProviderLongitude => config.coordinates.longitude.to_string(),
            FieldId::Island => config.salatmv.island.clone(),

            FieldId::UiMode => format!("{:?}", config.display.ui_mode),
            FieldId::TimeFormat => format!("{:?}", config.display.format),
            FieldId::Indicator => format!("{:?}", config.display.indicator),
            FieldId::Fullscreen => config.display.fullscreen.to_string(),
            FieldId::Seconds => config.display.seconds.to_string(),
            FieldId::Location => config.display.location.name().to_string(),

            FieldId::NotifEnabled => config.notifications.enabled.to_string(),
            FieldId::NotifOffset => config.notifications.offset.to_string(),

            FieldId::RawMode => format!("{:?}", config.raw_output.mode),
            FieldId::RawPool => config.raw_output.pool.to_string(),
            FieldId::RawSeparator => config.raw_output.raw_separator.clone(),
            FieldId::RawCustomString => config.raw_output.custom_string.clone(),
        }
    }

    pub fn options_len(self) -> Option<usize> {
        let len = match self {
            FieldId::ProviderType => 2,
            FieldId::ProviderMethod => METHOD_VARIANTS.len(),
            FieldId::ProviderMadhab => MADHAB_VARIANTS.len(),
            FieldId::UiMode => UIMODE_VARIANTS.len(),
            FieldId::TimeFormat => TIMEFORMAT_VARIANTS.len(),
            FieldId::Indicator => INDICATOR_VARIANTS.len(),
            FieldId::Location => LOCATION_VARIANTS.len(),
            FieldId::RawMode => RAWMODE_VARIANTS.len(),
            _ => return None,
        };
        Some(len)
    }

    fn current_index(self, config: &Config) -> Option<usize> {
        match self {
            FieldId::ProviderType => match config.provider {
                ProviderKind::SalatMv => Some(0),
                ProviderKind::Calculation => Some(1),
            },
            FieldId::ProviderMethod => METHOD_VARIANTS.iter().position(|v| *v == config.calculation.method),
            FieldId::ProviderMadhab => MADHAB_VARIANTS.iter().position(|v| *v == config.calculation.madhab),
            FieldId::UiMode => UIMODE_VARIANTS.iter().position(|v| *v == config.display.ui_mode),
            FieldId::TimeFormat => TIMEFORMAT_VARIANTS.iter().position(|v| *v == config.display.format),
            FieldId::Indicator => INDICATOR_VARIANTS.iter().position(|v| *v == config.display.indicator),
            FieldId::Location => LOCATION_VARIANTS.iter().position(|v| *v == config.display.location),
            FieldId::RawMode => RAWMODE_VARIANTS.iter().position(|v| *v == config.raw_output.mode),
            _ => None,
        }
    }

    pub fn cycle(self, config: &mut Config, delta: isize) {
        let len = match self.options_len() {
            Some(len) => len,
            None => return,
        };
        let idx = self.current_index(config).unwrap_or(0);
        let next = (idx as isize + delta).rem_euclid(len as isize) as usize;
        match self {
            FieldId::ProviderMethod => {
                config.calculation.method = METHOD_VARIANTS[next];
            }
            FieldId::ProviderMadhab => {
                config.calculation.madhab = MADHAB_VARIANTS[next];
            }
            FieldId::UiMode => config.display.ui_mode = UIMODE_VARIANTS[next],
            FieldId::TimeFormat => config.display.format = TIMEFORMAT_VARIANTS[next],
            FieldId::Indicator => config.display.indicator = INDICATOR_VARIANTS[next],
            FieldId::Location => config.display.location = LOCATION_VARIANTS[next],
            FieldId::RawMode => config.raw_output.mode = RAWMODE_VARIANTS[next],
            _ => {}
        }
    }

    pub fn toggle(self, config: &mut Config) {
        match self {
            FieldId::Fullscreen => config.display.fullscreen = !config.display.fullscreen,
            FieldId::Seconds => config.display.seconds = !config.display.seconds,
            FieldId::NotifEnabled => config.notifications.enabled = !config.notifications.enabled,
            FieldId::RawPool => config.raw_output.pool = !config.raw_output.pool,
            _ => {}
        }
    }

    pub fn commit_text(self, config: &mut Config, text: &str) -> Result<(), String> {
        match self {
            FieldId::ProviderLatitude => {
                let value: f64 = text.trim().parse().map_err(|e| format!("invalid latitude: {e}"))?;
                if !value.is_finite() || !(-90.0..=90.0).contains(&value) {
                    return Err(format!("latitude out of range: {value}"));
                }
                config.coordinates.latitude = value;
            }
            FieldId::ProviderLongitude => {
                let value: f64 = text.trim().parse().map_err(|e| format!("invalid longitude: {e}"))?;
                if !value.is_finite() || !(-180.0..=180.0).contains(&value) {
                    return Err(format!("longitude out of range: {value}"));
                }
                config.coordinates.longitude = value;
            }
            FieldId::NotifOffset => {
                let value: i32 = text.trim().parse().map_err(|e| format!("invalid offset: {e}"))?;
                config.notifications.offset = value;
            }
            FieldId::ProviderLocation => {
                config.calculation.location = text.to_string();
            }
            FieldId::RawSeparator => config.raw_output.raw_separator = text.to_string(),
            FieldId::RawCustomString => config.raw_output.custom_string = text.to_string(),
            _ => return Err("field is not editable as text".to_string()),
        }
        Ok(())
    }
}

fn method_name(method: &Method) -> &'static str {
    match method {
        Method::MuslimWorldLeague => "MuslimWorldLeague",
        Method::Egyptian => "Egyptian",
        Method::Karachi => "Karachi",
        Method::UmmAlQura => "UmmAlQura",
        Method::Dubai => "Dubai",
        Method::MoonsightingCommittee => "MoonsightingCommittee",
        Method::NorthAmerica => "NorthAmerica",
        Method::Kuwait => "Kuwait",
        Method::Qatar => "Qatar",
        Method::Singapore => "Singapore",
        Method::Tehran => "Tehran",
        Method::Turkey => "Turkey",
    }
}

pub fn popup_entries(kind: PopupKind, filter: &str) -> Vec<PopupEntry> {
    let needle = filter.trim().to_lowercase();
    match kind {
        PopupKind::Location => CITIES
            .iter()
            .filter(|c| needle.is_empty() || fuzzy(c[0], &needle) || fuzzy(c[1], &needle))
            .map(PopupEntry::City)
            .collect(),
        PopupKind::Island => island_keys()
            .into_iter()
            .filter(|n| needle.is_empty() || fuzzy(n, &needle))
            .map(PopupEntry::Island)
            .collect(),
    }
}

pub fn apply_city(config: &mut Config, city: &'static [&'static str; 4]) {
    config.calculation.location = format!("{}, {}", city[0], city[1]);
    config.coordinates.latitude = city[2].parse().unwrap_or_default();
    config.coordinates.longitude = city[3].parse().unwrap_or_default();
}

#[test]
fn test_fields_for_counts() {
    let config = Config::default();
    assert_eq!(fields_for(0, &config).len(), 6);
    assert_eq!(fields_for(1, &config).len(), 6);
    assert_eq!(fields_for(2, &config).len(), 2);
    assert_eq!(fields_for(3, &config).len(), 4);
    assert!(fields_for(4, &config).is_empty());

    let mv_config = Config {
        provider: ProviderKind::SalatMv,
        ..Config::default()
    };
    assert_eq!(fields_for(0, &mv_config).len(), 2);
}

#[test]
fn test_cycle_wraps() {
    let mut config = Config::default();

    FieldId::Indicator.cycle(&mut config, -1);
    assert_eq!(config.display.indicator, TimeIndicator::Current);
    for _ in 0..3 {
        FieldId::Indicator.cycle(&mut config, 1);
    }
    assert_eq!(config.display.indicator, TimeIndicator::Empty);

    FieldId::ProviderMadhab.cycle(&mut config, 1);
    assert_eq!(config.calculation.madhab, Madhab::Hanafi);
}

#[test]
fn test_numeric_rejects() {
    let mut config = Config::default();

    assert!(FieldId::ProviderLatitude.commit_text(&mut config, "abc").is_err());
    assert!(FieldId::ProviderLatitude.commit_text(&mut config, "999").is_err());
    assert!(FieldId::ProviderLatitude.commit_text(&mut config, "21.4225").is_ok());
    assert!((config.coordinates.latitude - 21.4225).abs() < f64::EPSILON);

    assert!(FieldId::NotifOffset.commit_text(&mut config, "").is_err());
    assert!(FieldId::NotifOffset.commit_text(&mut config, "-14").is_ok());
    assert_eq!(config.notifications.offset, -14);
    assert!(FieldId::NotifOffset.commit_text(&mut config, "99999999999").is_err());

    FieldId::NotifOffset.step(&mut config, 4);
    assert_eq!(config.notifications.offset, -10);
    FieldId::NotifOffset.step(&mut config, -20);
    assert_eq!(config.notifications.offset, -30);
}

#[test]
fn test_steppable_flags() {
    assert!(FieldId::NotifOffset.steppable());
    assert!(!FieldId::ProviderLatitude.steppable());
    assert!(!FieldId::Indicator.steppable());
}

#[test]
fn test_censor_flags() {
    assert!(FieldId::ProviderLatitude.censored());
    assert!(FieldId::ProviderLongitude.censored());
    assert!(!FieldId::ProviderLocation.censored());
    assert!(!FieldId::NotifOffset.censored());
}

#[test]
fn test_pick_rows() {
    let config = Config::default();
    let rows = fields_for(0, &config);
    assert!(matches!(rows[1].kind, FieldKind::Pick(PopupKind::Location)));

    let mv_config = Config {
        provider: ProviderKind::SalatMv,
        ..Config::default()
    };
    let rows = fields_for(0, &mv_config);
    assert!(matches!(rows[1].kind, FieldKind::Pick(PopupKind::Island)));
}

#[test]
fn test_fuzzy_matcher() {
    // subsequence: chars in order
    assert!(fuzzy("Male'", "ml'"));
    assert!(fuzzy("Makkah", "mkh"));
    assert!(fuzzy("Saudi Arabia", "saudiarabia"));
    assert!(fuzzy("London", "LON"));
    // order matters
    assert!(!fuzzy("Oman", "am"));
    // missing char fails
    assert!(!fuzzy("Male'", "malex"));
}

#[test]
fn test_popup_filter() {
    assert_eq!(popup_entries(PopupKind::Location, "").len(), CITIES.len());

    let hits = popup_entries(PopupKind::Location, "london");
    assert!(!hits.is_empty());
    assert!(hits.iter().all(|e| matches!(
        e,
        PopupEntry::City(c)
            if fuzzy(c[0], "london") || fuzzy(c[1], "london")
    )));

    // countries missing from the old cities crate are now available;
    // fuzzy may also match other names, so require hits overall + at least one Saudi one
    let saudi = popup_entries(PopupKind::Location, "saudi");
    assert!(saudi.iter().all(|e| matches!(
        e,
        PopupEntry::City(c) if fuzzy(c[0], "saudi") || fuzzy(c[1], "saudi")
    )));
    assert!(saudi.iter().any(|e| matches!(
        e,
        PopupEntry::City(c) if c[1] == "Saudi Arabia"
    )));
    let makkah = popup_entries(PopupKind::Location, "makkah");
    assert!(makkah.iter().any(|e| matches!(
        e,
        PopupEntry::City(c) if c[1] == "Saudi Arabia"
    )));

    assert!(popup_entries(PopupKind::Island, "anything").is_empty());
}

#[test]
fn test_apply_city() {
    let city: &'static [&'static str; 4] = &CITIES
        .iter()
        .find(|c| c[0] == "London" && c[1] == "United Kingdom")
        .expect("london should exist in embedded cities");

    let mut config = Config::default();
    apply_city(&mut config, city);
    assert_eq!(config.calculation.location, "London, United Kingdom");
    assert_eq!(config.coordinates.latitude, city[2].parse::<f64>().unwrap());
    assert_eq!(config.coordinates.longitude, city[3].parse::<f64>().unwrap());
}

#[test]
fn test_island_popup_entries() {
    let entries = popup_entries(PopupKind::Island, "");
    assert_eq!(entries.len(), ISLAND_DATA.len());
    assert!(entries.iter().any(|e| matches!(
        e,
        PopupEntry::Island(key) if key == DEFAULT_MV_ISLAND_KEY
    )));
}
