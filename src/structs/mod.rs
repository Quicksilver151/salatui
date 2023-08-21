
pub mod conf;
pub use conf::*;

pub mod input;
pub use input::*;

pub mod data;
pub use data::*;

#[derive(Debug, Default)]
pub struct UI {
    name: String,
    index: u8,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub ui: UI,
    pub fullscreen: bool,
    pub prayertime: PrayerTime,
    pub input_map: InputMap,
    pub input_char: char,
}

use serde::*;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrayerTime {
    pub index: u32,
    pub day: u32,
    pub fajr: u32,
    pub sun: u32,
    pub dhuhur: u32,
    pub asr: u32,
    pub magrib: u32,
    pub isha: u32,
}

impl PrayerTime {
    pub fn from_vec(list:Vec<u32>) -> PrayerTime {
        PrayerTime {
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
            TimeFormat::Twelve => {
                return time_list.iter().map(to_time).map(|t| format!("{: >2}:{:0>2} {}",{if t.0 >13{t.0%12}else{t.0}},t.1, {if t.0 > 11{"PM"} else{"AM"} })).collect();
            },
            TimeFormat::TwentyFour => {
                return time_list.iter().map(to_time).map(|t| format!("{: >2}:{:0>2}",t.0,t.1)).collect();
            },
            TimeFormat::Minutes => {
                return time_list.iter().map(|t| format!("{}",t)).collect();
            },
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
                    string.push_str(&format!("{}{}",time,outconf.raw_seperator))
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
    let value = PrayerTime { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let mut config = Config::default();
    
    config.display.format = TimeFormat::Twelve;
    let expected:Vec<String> = vec!["77", "225", " 4:53 AM", " 6:05 AM", "12:16 PM", " 3:32 PM", " 6:18 PM", " 7:31 PM"]
        .into_iter().map(|x|x.to_owned()).collect();
    let result = value.format(&config);
    
    config.display.format = TimeFormat::TwentyFour;
    let expected2:Vec<String> = vec!["77", "225", " 4:53", " 6:05", "12:16", "15:32", "18:18", "19:31"]
        .into_iter().map(|x|x.to_owned()).collect();
    let result2 = value.format(&config);

    assert_eq!(result , expected );
    assert_eq!(result2, expected2);
}

#[test]
fn test_prayertime() {
    let expected = PrayerTime { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let result = PrayerTime::from_vec(vec![77, 225, 293, 365, 736, 932, 1098, 1171]);
    let result2 = TimeSetData::load("GDh. Vilingili").unwrap().data_from_day(225);
    assert_eq!(expected, result);
    assert_eq!(expected, result2);
}





