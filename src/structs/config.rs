use crate::*;
// [Provider] ===================================

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CalculationMethod {
    pub name: String,
    pub location: String,
    pub coordinates: (String, String)  //TODO: change to actual coord struct
    
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Provider {
    #[default]
    Manual,
    Data(String),
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
















