
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

#[test]
fn test(){
    use std::fs;
    
    let atoll = fs::read_to_string("./data/atolls.csv")
        .expect("Should have been able to read the file");
    let pt = fs::read_to_string("./data/ptdata.csv")
        .expect("Should have been able to read the file");
    let island = fs::read_to_string("./data/islands.csv")
        .expect("Should have been able to read the file");
    let test_data = MVRawData {atoll, pt, island};
    
    let parsed = test_data.parse_timeset(177).unwrap();
    let expected = TimeSet {
        name: String::from("GDh. Vilingili"),
        details: String::new(),
        coordinates: (
            String::from("0.755293"),
            String::from("73.434885")
        ),
        data: vec![
            vec![77, 0, 289, 366, 734, 937, 1095, 1172],
            vec![77, 1, 290, 366, 735, 937, 1095, 1173],
        ]
    };
    
    assert_eq!(parsed, expected);
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MVRawData {
    pub pt: String,
    pub atoll: String,
    pub island: String,
}
impl MVRawData {
    pub fn from(pt: String, atoll: String, island: String) -> MVRawData {
        MVRawData {pt, atoll, island}
    }
    pub fn parse_timeset(&self, island_index:usize) -> Option<TimeSet>{
        if self.pt.is_empty(){
            return None
        }
        
        // Island
        let island_data:Vec<String> = {
            let columns:Vec<String> = self.island.split('\n').map(str::to_string).collect();
            let mut table:Vec<Vec<String>> = columns.into_iter().map(|column| column.split(';').map(str::to_string).collect()).collect();
            table.pop();
            table.into_iter().filter(|column| column[1] == island_index.to_string()).collect::<Vec<Vec<String>>>()[0].to_owned()
        };
        let island_name = island_data[3].to_string();
        
        let atoll_index:usize = island_data[2].parse().unwrap();
        // Atoll
        let atoll_data = {
            let columns:Vec<&str> = self.atoll.split('\n').collect();
            let table:Vec<Vec<&str>> = columns.into_iter().map(|column| column.split(';').collect()).collect();
            table
        };
        let atoll_name = atoll_data[atoll_index][1].to_string();
        
        
        let timeset = island_data[0].clone();
        
        // PT
        let pt_data:Vec<Vec<u32>> = { 
            let columns: Vec<&str> = self.pt.split('\n').collect();
            let mut table: Vec<Vec<&str>> = columns.into_iter().map(|column| column.split(';').collect()).collect();
            table.pop();
            table.into_iter().filter(|column| column[0] == timeset).map(|column| column.into_iter().map(|e| e.parse::<u32>().unwrap_or(444000444)).collect()).collect()
        };
        
        let name = format!("{atoll_name}. {island_name}");
        let details = String::from("");
        let coordinates = (island_data[7].to_string(), island_data[8].to_string());
        let data = pt_data;
        
        Some(TimeSet {name, details, coordinates, data})
        
        
    }
}

#[derive(Debug, Default, Eq)]
pub struct TimeSet {
    pub name: String,
    pub details: String,
    pub coordinates: (String, String),
    pub data: Vec<Vec<u32>>,
}
impl PartialEq for TimeSet {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
        && self.details == other.details
        && self.coordinates == other.coordinates
        && self.data[0..2] == other.data[0..2]
    }
}




