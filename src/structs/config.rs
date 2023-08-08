use crate::*;
// [Provider] ===================================

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CalculationMethod {
    pub name: String,
    pub location: String,
    pub coordinates: String  //TODO: change to actual coord struct
    
}

#[derive(Debug, Eq, Serialize, Deserialize)]
pub struct TimeSetMeta {
    pub name: String,
    pub details: String,
    pub coordinates: (String, String),
}
impl Default for TimeSetMeta {
    fn default() -> Self {
        let name = String::from("");
        let details = String::from("");
        let coordinates = (String::from(""), String::from(""));
        
        TimeSetMeta { name, details, coordinates}
    }
}
impl PartialEq for TimeSetMeta {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        && self.details == other.details
        && self.coordinates == other.coordinates
        // && self.data.content[0..2] == other.data.content[0..2]
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Provider {
    #[default]
    Manual,
    Data(TimeSetMeta),
    Calculation(CalculationMethod),
}

// [Display] ===================================

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum TimeFormat {
    #[default]
    Twelve,
    TwentyFour,
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
    Json,
    #[default]
    RawData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RawOutput {
    pub mode: RawOutputMode,
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
        let raw_seperator = String::default();
        let custom_string = String::from("[%fhmp, %shmp, &dhmp, %ahmp, %mhmp, %ihmp]");
        
        RawOutput { mode, raw_seperator,custom_string}
    }
}


// ============================================
//                  [CONFIG] 
// ============================================

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config{
    pub provider: Provider,
    pub display: Display,
    pub notifications: Notifications,
    pub raw_output: RawOutput,
}















