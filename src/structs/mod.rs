
pub mod config;
pub use config::*;

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
}

