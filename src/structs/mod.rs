
pub mod conf;
pub use conf::*;

pub mod input;
pub use input::*;

pub mod data;
pub use data::*;

pub mod ui;
pub use ui::*;



pub struct AppState {
    pub prayertime: PrayerTime,
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
    
    pub fn format(&self, config: &Config) -> Vec<String> {
        let time_list: Vec<u32> = self.to_vec();
        match config.display.format {
            TimeFormat::Twelve => {
                return time_list.iter().map(to_time).map(|t| format!("{: >2}:{:0>2} {}",{if t.0 >13{t.0%12}else{t.0}},t.1, {if t.0 > 11{"PM"} else{"AM"} })).collect();
            },
            TimeFormat::TwentyFour => {
                return time_list.iter().map(to_time).map(|t| format!("{: >2}:{:0>2}",t.0,t.1)).collect();
            },
        }
    }
    pub fn output_format(&self, config: &Config) -> String {
        // use serde_json::to_writer_pretty;
        let time_list: Vec<String> = self.format(config);
        let outconf = &config.raw_output;
        match config.raw_output.mode {
            RawOutputMode::Array   => format!("{:?}", time_list),
            RawOutputMode::Custom  => {
                todo!();
            },
            RawOutputMode::TOML    => todo!(),
            RawOutputMode::FormattedJson => serde_json::to_string_pretty(self).unwrap_or_default(),
            RawOutputMode::Json    => serde_json::to_string(self).unwrap_or_default(),
            RawOutputMode::RawData => {
                let mut string = "".to_owned();
                for time in time_list {
                    if &time == self.format(config).last().unwrap(){
                        string.push_str(&time);
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

#[test]
fn test_format() {
    let value = PrayerTime { index: 77, day: 225, fajr: 293, sun: 365, dhuhur: 736, asr: 932, magrib: 1098, isha: 1171 };
    let mut config = Config::load().unwrap_or_default();
    
    config.display.format = TimeFormat::Twelve;
    let expected:Vec<String> = vec![" 4:53 AM", " 6:05 AM", "12:16 PM", " 3:32 PM", " 6:18 PM", " 7:31 PM"]
        .into_iter().map(|x|x.to_owned()).collect();
    let result = value.format(&config);
    
    config.display.format = TimeFormat::TwentyFour;
    let expected2:Vec<String> = vec![" 4:53", " 6:05", "12:16", "15:32", "18:18", "19:31"]
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





