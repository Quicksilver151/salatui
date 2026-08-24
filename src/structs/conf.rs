
use core::panic;
use std::path::PathBuf;

use crate::*;
// [Provider] ===================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationConfig {
    pub name: String,
    pub location: String,
    pub method: Method,
    pub madhab: Madhab,
    pub coordinates: Coords
}

impl Default for CalculationConfig {
    fn default() -> Self {
        let name:String = "Default".into();
        let location: String = "Kaaba".into();
        let method = Method::default();
        let madhab = Madhab::default();
        let coordinates = Coords{latitude: 21.4225, longitude: 39.8262};

        CalculationConfig { name, location, method, madhab, coordinates}
    }
}



#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    #[default]
    MuslimWorldLeague,    
    Egyptian,    
    Karachi,    
    UmmAlQura,    
    Dubai,    
    MoonsightingCommittee,    
    NorthAmerica,    
    Kuwait,    
    Qatar,    
    Singapore,    
    Tehran,    
    Turkey,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Madhab {
    #[default]
    Shafi = 1,
    Hanafi = 2,
}
impl Madhab {
    pub fn to_runtime_config(&self) -> salah::Madhab {
        match self {
            Self::Shafi => salah::Madhab::Shafi,
            Self::Hanafi => salah::Madhab::Hanafi,
        }
    }
}


impl Method {
    pub fn to_runtime_config(&self) -> salah::Method {
        match self {
            Self::MuslimWorldLeague     => salah::Method::MuslimWorldLeague,
            Self::Egyptian              => salah::Method::Egyptian,
            Self::Karachi               => salah::Method::Karachi,
            Self::UmmAlQura             => salah::Method::UmmAlQura,
            Self::Dubai                 => salah::Method::Dubai,
            Self::MoonsightingCommittee => salah::Method::MoonsightingCommittee,
            Self::NorthAmerica          => salah::Method::NorthAmerica,
            Self::Kuwait                => salah::Method::Kuwait,
            Self::Qatar                 => salah::Method::Qatar,
            Self::Singapore             => salah::Method::Singapore,
            Self::Tehran                => salah::Method::Tehran,
            Self::Turkey                => salah::Method::Turkey,

        }

    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Coords {
    pub latitude: f64,
    pub longitude: f64,
}
impl Coords {
    pub fn to_runtime_config(&self) -> salah::Coordinates{
        salah::Coordinates{
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProviderConfig {
    Data(String),
    Calculation(CalculationConfig),
}
impl Default for ProviderConfig {
    fn default() -> Self {
        Self::Calculation(CalculationConfig::default())
    }
}


// [Display] ===================================

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UIMode {
    #[default]
    Normal,
    Fancy,
    Text,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeFormat {
    #[default]
    Twelve,
    TwentyFour,
    Minutes,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeIndicator {
    Empty,
    Current,
    #[default]
    Inbetween,
    Next,
}
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Display {
    pub ui_mode: UIMode,
    pub format: TimeFormat,
    pub indicator: TimeIndicator,
    pub fullscreen: bool,
    pub show_raw_output: bool,
    pub seconds: bool,
    pub location: bool,
    pub coordinates: bool,
}

// [Notifications] ===================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub enabled: bool,
    pub offset: i32, // minutes
}
impl Default for Notifications {
    fn default() -> Self {
        Notifications { enabled: true, offset: 0 }
    }
}

// [Raw Output] ===================================

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RawOutputMode {
    Array,
    Custom,
    #[default]
    PrettyJson,
    Json,
    RawData,
    TOML,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RawOutput {
    pub mode: RawOutputMode,
    pub pool: bool,
    pub raw_separator: String,
    pub custom_string: String
}
#[allow(dead_code)]
impl RawOutput {
    fn set_separator(&mut self, sep: &str) {
        self.raw_separator = sep.to_string();
    }
}
impl Default for RawOutput {
    fn default() -> Self {
        let mode = RawOutputMode::default();
        let pool = false;
        let raw_separator = String::from("\n");
        let custom_string = String::from("[%fh:mp, %sh:mp, %dh:mp, %ah:mp, %mh:mp, %ih:mp]");
        
        RawOutput { mode, pool, raw_separator,custom_string}
    }
}

// ============================================
//                  [CONFIG] 
// ============================================

// CONSTS

const CONFIG_NAME: &str = "config-dev";


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub provider: ProviderConfig,
    pub display: Display,
    pub notifications: Notifications,
    pub raw_output: RawOutput,
}
impl Config {
    
    pub fn load() -> Config {
        match confy::load("salatui", CONFIG_NAME){
            Ok(config) => config,
            Err(err) => {
                println!("{err}\nconfig is broken\nloading a new config from defaults");
                let new_config = Config {
                    provider: ProviderConfig::Calculation(CalculationConfig::default()),
                    ..Config::default()
                };

                match new_config.save() {
                    Ok(_) => {}
                    Err(err) => println!("Failed to create new file due to Error: {}",err),
                }

                new_config
            }
        }
    }

    pub fn load_from_path(path: PathBuf) -> Config {
        match confy::load_path(path){
            Ok(config) => config,
            Err(err) => {
                panic!("{err}\n config is broken or not accessible");
            }
        }
    }


    
    pub fn save(&self) -> Result<(), confy::ConfyError> {
        confy::store("salatui", CONFIG_NAME, self)
    }
    
}















