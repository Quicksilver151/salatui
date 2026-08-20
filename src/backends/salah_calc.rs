use salah::prelude::*; //{Configuration, Coordinates, Parameters, Method, Madhab, Prayer, DateTime, Local, NaiveDate, Timelike};
use crate::structs;


#[derive(Debug)]
pub struct SalahCalcConfig {
    pub params: Parameters,
    pub coordinates: Coordinates,
}


impl SalahCalcConfig {
    pub fn new(method: Method, madhab: Madhab, coordinates: Coordinates) -> SalahCalcConfig {
        let params = Configuration::with(method, madhab);
        SalahCalcConfig { params, coordinates}
    }

    pub fn get_prayer_times(&self, local_date: NaiveDate) -> structs::PrayerTimes {
        let params:Parameters = self.params;
        let coordinates = self.coordinates;
        let schedule = salah::PrayerTimes::new(local_date, coordinates, params);
        
        
        let prayer_times = [
            Prayer::Fajr,
            Prayer::Sunrise,
            Prayer::Dhuhr,
            Prayer::Asr,
            Prayer::Maghrib,
            Prayer::Isha,
            Prayer::Qiyam,
        ];

        let mut prayer_list: Vec<u32> = vec![0,0];
        for prayer in prayer_times {
            let datetime: DateTime<Local> = DateTime::from(schedule.time(prayer));
            let hourtime = datetime.hour();
            // if hourtime >= 4 {hourtime-=4} else {hourtime += 12-4} // HACK: TIMEZONE ADJSUTMENT
            let minute_offset = datetime.minute();
            let minutes = hourtime * 60 + minute_offset;
            
            prayer_list.append(&mut vec![minutes]);
        }
        
        // dbg!(prayer_list);
        // todo!();
        
        structs::PrayerTimes::from_vec(prayer_list)
    }

    pub fn get_prayer_times_now(self) -> structs::PrayerTimes {
        let local_date = chrono::offset::Utc::now().naive_utc().date();
        self.get_prayer_times(local_date)
    }
}
