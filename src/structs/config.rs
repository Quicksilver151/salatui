
// [Provider] ===================================

#[derive(Debug, Default)]
pub struct CalculationMethod {
    pub name: String,
    pub location: String,
    pub coordinates: String  //TODO: change to actual coord struct
    
}
#[derive(Debug, Default)]
pub struct TimeSet{
    pub name: String,
    pub location: String,
    pub coordinates: String,
}
#[derive(Debug, Default)]
pub enum Provider {
    #[default]
    Manual,
    Data(TimeSet),
    Calculation(CalculationMethod),
}

// [Display] ===================================

#[derive(Debug, Default)]
pub enum TimeFormat {
    #[default]
    Twelve,
    TwentyFour,
}
#[derive(Debug, Default)]
pub enum TimeIndicator {
    Empty,
    Current,
    #[default]
    Inbetween,
    Next,
}
#[derive(Debug, Default)]
pub struct Display {
    pub format: TimeFormat,
    pub seconds: bool,
    pub indicator: TimeIndicator,
    pub location: bool,
    pub coordinates: bool,
}

// [Notifications] ===================================

#[derive(Debug)]
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

#[derive(Debug, Default)]
pub enum OutputMode {
    Json,
    #[default]
    RawData,
    Array,
    Custom,
}
#[derive(Debug)]
pub struct RawOutput {
    pub mode: OutputMode,
    pub raw_seperator: String,
    pub custom_string: String
}
#[allow(dead_code)]
impl RawOutput {
    fn add_separator(&mut self, sep: &str) {
        self.raw_seperator = sep.to_string();
    }
}
impl Default for RawOutput {
    fn default() -> Self {
        let mode = OutputMode::default();
        let raw_seperator = String::default();
        let custom_string = String::from("[%fhmp, %shmp, &dhmp, %ahmp, %mhmp, %ihmp]");
        
        RawOutput { mode, raw_seperator,custom_string}
    }
}


// ============================================
//                  [CONFIG] 
// ============================================

#[derive(Debug, Default)]
pub struct Config{
    pub provider: Provider,
    pub display: Display,
    pub notifications: Notifications,
    pub raw_output: RawOutput,
}















