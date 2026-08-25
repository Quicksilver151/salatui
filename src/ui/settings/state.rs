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
    pub calc_cache: Option<CalculationConfig>,
    pub data_cache: Option<String>,
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
    Dataset,
}

#[derive(Debug)]
pub enum PopupEntry {
    City(&'static cities::City),
    Dataset(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldId {
    ProviderType,
    ProviderName,
    ProviderLocation,
    ProviderMethod,
    ProviderMadhab,
    ProviderLatitude,
    ProviderLongitude,
    DatasetName,

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
            match &config.provider {
                ProviderConfig::Calculation(_) => rows.extend([
                    FieldRow { id: FieldId::ProviderName, label: "name", kind: FieldKind::Text { numeric: false } },
                    FieldRow { id: FieldId::ProviderLocation, label: "location", kind: FieldKind::Pick(PopupKind::Location) },
                    FieldRow { id: FieldId::ProviderMethod, label: "method", kind: FieldKind::Cycle },
                    FieldRow { id: FieldId::ProviderMadhab, label: "madhab", kind: FieldKind::Cycle },
                    FieldRow { id: FieldId::ProviderLatitude, label: "latitude", kind: FieldKind::Text { numeric: true } },
                    FieldRow { id: FieldId::ProviderLongitude, label: "longitude", kind: FieldKind::Text { numeric: true } },
                ]),
                ProviderConfig::Data(_) => rows.extend([FieldRow {
                    id: FieldId::DatasetName,
                    label: "dataset",
                    kind: FieldKind::Pick(PopupKind::Dataset),
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
                | FieldId::ProviderName
                | FieldId::ProviderLocation
                | FieldId::ProviderMethod
                | FieldId::ProviderMadhab
                | FieldId::ProviderLatitude
                | FieldId::ProviderLongitude
                | FieldId::DatasetName
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

    fn calc(config: &Config) -> Option<&CalculationConfig> {
        match &config.provider {
            ProviderConfig::Calculation(c) => Some(c),
            ProviderConfig::Data(_) => None,
        }
    }

    fn calc_mut(config: &mut Config) -> Option<&mut CalculationConfig> {
        match &mut config.provider {
            ProviderConfig::Calculation(c) => Some(c),
            ProviderConfig::Data(_) => None,
        }
    }

    pub fn value_str(self, config: &Config) -> String {
        match self {
            FieldId::ProviderType => match &config.provider {
                ProviderConfig::Data(_) => "Data".to_string(),
                ProviderConfig::Calculation(_) => "Calculation".to_string(),
            },
            FieldId::ProviderName => Self::calc(config).map(|c| c.name.clone()).unwrap_or_default(),
            FieldId::ProviderLocation => Self::calc(config).map(|c| c.location.clone()).unwrap_or_default(),
            FieldId::ProviderMethod => Self::calc(config).map(|c| method_name(&c.method).to_string()).unwrap_or_default(),
            FieldId::ProviderMadhab => Self::calc(config).map(|c| format!("{:?}", c.madhab)).unwrap_or_default(),
            FieldId::ProviderLatitude => Self::calc(config).map(|c| c.coordinates.latitude.to_string()).unwrap_or_default(),
            FieldId::ProviderLongitude => Self::calc(config).map(|c| c.coordinates.longitude.to_string()).unwrap_or_default(),
            FieldId::DatasetName => match &config.provider {
                ProviderConfig::Data(name) => name.clone(),
                ProviderConfig::Calculation(_) => String::new(),
            },

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
            FieldId::ProviderType => match &config.provider {
                ProviderConfig::Calculation(_) => Some(1),
                ProviderConfig::Data(_) => Some(0),
            },
            FieldId::ProviderMethod => Self::calc(config).and_then(|c| METHOD_VARIANTS.iter().position(|v| *v == c.method)),
            FieldId::ProviderMadhab => Self::calc(config).and_then(|c| MADHAB_VARIANTS.iter().position(|v| *v == c.madhab)),
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
                if let Some(c) = Self::calc_mut(config) {
                    c.method = METHOD_VARIANTS[next];
                }
            }
            FieldId::ProviderMadhab => {
                if let Some(c) = Self::calc_mut(config) {
                    c.madhab = MADHAB_VARIANTS[next];
                }
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
                if let Some(c) = Self::calc_mut(config) {
                    c.coordinates.latitude = value;
                }
            }
            FieldId::ProviderLongitude => {
                let value: f64 = text.trim().parse().map_err(|e| format!("invalid longitude: {e}"))?;
                if !value.is_finite() || !(-180.0..=180.0).contains(&value) {
                    return Err(format!("longitude out of range: {value}"));
                }
                if let Some(c) = Self::calc_mut(config) {
                    c.coordinates.longitude = value;
                }
            }
            FieldId::NotifOffset => {
                let value: i32 = text.trim().parse().map_err(|e| format!("invalid offset: {e}"))?;
                config.notifications.offset = value;
            }
            FieldId::ProviderName => {
                if let Some(c) = Self::calc_mut(config) {
                    c.name = text.to_string();
                }
            }
            FieldId::ProviderLocation => {
                if let Some(c) = Self::calc_mut(config) {
                    c.location = text.to_string();
                }
            }
            FieldId::DatasetName => match &mut config.provider {
                ProviderConfig::Data(name) => *name = text.to_string(),
                ProviderConfig::Calculation(_) => return Err("not a dataset provider".to_string()),
            },
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
        PopupKind::Location => cities::all()
            .iter()
            .filter(|c| {
                needle.is_empty()
                    || c.city.to_lowercase().contains(&needle)
                    || c.country.to_lowercase().contains(&needle)
            })
            .map(PopupEntry::City)
            .collect(),
        PopupKind::Dataset => TimeSetData::list()
            .into_iter()
            .filter(|n| needle.is_empty() || n.to_lowercase().contains(&needle))
            .map(PopupEntry::Dataset)
            .collect(),
    }
}

pub fn apply_city(config: &mut Config, city: &cities::City) {
    if let Some(c) = FieldId::calc_mut(config) {
        c.location = format!("{}, {}", city.city, city.country);
        c.coordinates.latitude = city.latitude;
        c.coordinates.longitude = city.longitude;
    }
}

#[test]
fn test_fields_for_counts() {
    let config = Config::default();
    assert_eq!(fields_for(0, &config).len(), 7);
    assert_eq!(fields_for(1, &config).len(), 6);
    assert_eq!(fields_for(2, &config).len(), 2);
    assert_eq!(fields_for(3, &config).len(), 4);
    assert!(fields_for(4, &config).is_empty());

    let data_config = Config {
        provider: ProviderConfig::Data(String::new()),
        ..Config::default()
    };
    assert_eq!(fields_for(0, &data_config).len(), 2);
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
    match &config.provider {
        ProviderConfig::Calculation(c) => assert_eq!(c.madhab, Madhab::Hanafi),
        ProviderConfig::Data(_) => panic!("provider should stay calculation"),
    }
}

#[test]
fn test_numeric_rejects() {
    let mut config = Config::default();

    assert!(FieldId::ProviderLatitude.commit_text(&mut config, "abc").is_err());
    assert!(FieldId::ProviderLatitude.commit_text(&mut config, "999").is_err());
    assert!(FieldId::ProviderLatitude.commit_text(&mut config, "21.4225").is_ok());
    match &config.provider {
        ProviderConfig::Calculation(c) => assert!((c.coordinates.latitude - 21.4225).abs() < f64::EPSILON),
        ProviderConfig::Data(_) => panic!("provider should stay calculation"),
    }

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
    assert!(!FieldId::ProviderName.censored());
    assert!(!FieldId::NotifOffset.censored());
}

#[test]
fn test_pick_rows() {
    let config = Config::default();
    let rows = fields_for(0, &config);
    assert!(matches!(rows[2].kind, FieldKind::Pick(PopupKind::Location)));

    let data_config = Config {
        provider: ProviderConfig::Data(String::new()),
        ..Config::default()
    };
    let rows = fields_for(0, &data_config);
    assert!(matches!(rows[1].kind, FieldKind::Pick(PopupKind::Dataset)));
}

#[test]
fn test_popup_filter() {
    assert_eq!(popup_entries(PopupKind::Location, "").len(), cities::all().len());

    let hits = popup_entries(PopupKind::Location, "LONDON");
    assert!(!hits.is_empty());
    assert!(hits.iter().all(|e| matches!(
        e,
        PopupEntry::City(c)
            if c.city.to_lowercase().contains("london") || c.country.to_lowercase().contains("london")
    )));

    assert!(popup_entries(PopupKind::Dataset, "anything").is_empty());
}

#[test]
fn test_apply_city() {
    let Some(PopupEntry::City(city)) = popup_entries(PopupKind::Location, "london").into_iter().next() else {
        panic!("london should exist in embedded cities");
    };

    let mut config = Config::default();
    apply_city(&mut config, city);
    match &config.provider {
        ProviderConfig::Calculation(c) => {
            assert_eq!(c.location, format!("{}, {}", city.city, city.country));
            assert_eq!(c.coordinates.latitude, city.latitude);
            assert_eq!(c.coordinates.longitude, city.longitude);
        }
        ProviderConfig::Data(_) => panic!("provider should stay calculation"),
    }
}
