
// ===================
// general data store:
// ===================

// #[derive(Debug)]
// pub struct TimeSet{
//     pub name: String,
//     pub location: String,
//     pub coordinates: String,
//     pub data: Box<[i32; 366]>,
// }
// impl Default for TimeSet{
//     fn default() -> TimeSet {
//         TimeSet{
//             name: String::from(""),
//             location: String::from(""),
//             coordinates: String::from(""),
//             data: Box::new([0; 366]),
//         }
//     }
// }




// ===================
// built in support:
// ===================

// SALAT_MV:

pub struct MVRawData {
    pub pt: String,
    pub atoll: String,
    pub island: String,
}
impl MVRawData {
    pub fn from(pt: String, atoll: String, island: String) -> MVRawData {
        MVRawData {pt, atoll, island}
    }
    pub fn parse_timeset(&self, index:u8) -> MVTimeSet{
        todo!("{}",index)
    }
}


pub struct MVTimeSet {
    pub island: String,
    pub atoll: String,
    pub coordinates: (String, String),
    pub data: Vec<Vec<u32>>,
}




