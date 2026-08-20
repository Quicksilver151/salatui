
pub mod conf;

pub use conf::*;

pub mod input;
// pub use input::*;

pub mod data;
pub use data::*;
use salah::{Datelike, NaiveDate};

use crate::{salah_calc::SalahCalcConfig, Screen};

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
                time_set_data.data_from_day(date.ordinal0() as usize)
            }
            Provider::Calculation(salah_calc_config) => {
                salah_calc_config.get_prayer_times(date)
            }
        }
    }
}


impl AppState {
    pub fn init_provider(&mut self) {
        self.provider = match &self.config.provider {
            ProviderConfig::Data(data) => {
                let new_timeset = TimeSetData::load(data).unwrap(); // HACK: unwrap;
                Provider::DataSet(new_timeset)
            }
            ProviderConfig::Calculation(calculation_config) => {
                let method = calculation_config.method.to_runtime_config();
                let madhab = calculation_config.madhab.to_runtime_config();
                let coordinates = calculation_config.coordinates.to_runtime_config();
                let salah_calc_config = SalahCalcConfig::new(method, madhab, coordinates);

                Provider::Calculation(salah_calc_config)
                
            }

        };
        
    }
    pub fn get_prayer_times(&self) -> PrayerTimes{
        let date: NaiveDate = self.get_offset_date();
        self.provider.get_prayer_times(date)
    }

    pub fn get_offset_date(&self) -> chrono::NaiveDate {
        chrono::offset::Utc::now().date_naive() + chrono::Duration::days(self.day_offset)
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
            RawOutputMode::Custom  => todo!(),
            RawOutputMode::TOML    => todo!(),
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
}

fn to_time(minutes: &u32) -> (u32, u32){
    (minutes / 60, minutes % 60)
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
fn test_prayertime() {
    let expected = PrayerTimes { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let result = PrayerTimes::from_vec(vec![77, 225, 293, 365, 736, 932, 1098, 1171]);
    let result2 = TimeSetData::load("GDh. Vilingili").unwrap().data_from_day(225);
    assert_eq!(expected, result);
    assert_eq!(expected, result2);
}





