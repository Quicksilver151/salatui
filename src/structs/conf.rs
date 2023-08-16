use crate::*;
// [Provider] ===================================

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CalculationMethod {
    pub name: String,
    pub location: String,
    pub coordinates: (String, String)  //TODO: change to actual coord struct
    
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Provider {
    Data(String),
    Calculation(CalculationMethod),
}
impl Default for Provider {
    fn default() -> Self {
        Self::Data(String::from(""))
    }
}


// [Display] ===================================

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum TimeFormat {
    #[default]
    Twelve,
    TwentyFour,
    Minutes,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum TimeIndicator {
    Empty,
    Current,
    #[default]
    Inbetween,
    Next,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Display {
    pub show_raw_output: bool,
    pub format: TimeFormat,
    pub seconds: bool,
    pub indicator: TimeIndicator,
    pub location: bool,
    pub coordinates: bool,
}

// [Notifications] ===================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub command: String,
    pub offset: i32, // minutes
}
impl Default for Notifications {
    fn default() -> Self {
        let command:String = String::from("notify-send --urgency=critical");
        let offset:i32 = 0;
        
        Notifications { command, offset }
    }
}

// [Raw Output] ===================================

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum RawOutputMode {
    Array,
    Custom,
    PrettyJson,
    Json,
    #[default]
    RawData,
    TOML,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RawOutput {
    pub mode: RawOutputMode,
    pub pool: bool,
    pub raw_seperator: String,
    pub custom_string: String
}
#[allow(dead_code)]
impl RawOutput {
    fn set_separator(&mut self, sep: &str) {
        self.raw_seperator = sep.to_string();
    }
}
impl Default for RawOutput {
    fn default() -> Self {
        let mode = RawOutputMode::default();
        let pool = false;
        let raw_seperator = String::from("\n");
        let custom_string = String::from("[%fhmp, %shmp, &dhmp, %ahmp, %mhmp, %ihmp]");
        
        RawOutput { mode, pool, raw_seperator,custom_string}
    }
}

// ============================================
//                  [CONFIG] 
// ============================================

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub provider: Provider,
    pub display: Display,
    pub notifications: Notifications,
    pub raw_output: RawOutput,
}
impl Config {
    
    pub fn init() -> Result<(), confy::ConfyError> {
        
        let dirs = directories::ProjectDirs::from("", "", "salatui").unwrap();
        let mut confdir = dirs.config_dir().to_path_buf();
        confdir.push("config.toml");
        
        if std::path::Path::exists(&confdir){
            Ok(())
        } else {
            println!("config is missing\ncreating new config with defaults");
            let defaults = Config::default();
            confy::store("salatui", "config", defaults)
        }
    }
    
    pub fn load() -> Result<Config, confy::ConfyError> {
        confy::load("salatui", "config")
    }
    pub fn load_valid() -> Config {
        match Self::load() {
            Ok(config) => config,
            Err(err) => {
                println!("failed to load due to\n{err}");
                println!("using default data");
                Self::default()
            }
        }
    }
    
    pub fn save(&self) -> Result<(), confy::ConfyError> {
        confy::store("salatui", "config", self)
    }
    
}















