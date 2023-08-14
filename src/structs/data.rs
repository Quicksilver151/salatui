
use crate::*;


/// General Data structs

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeSetData {
    pub name: String,
    pub details: Option<String>,
    pub coordinates: (String, String),
    pub data: Vec<Vec<u32>>,
}
impl TimeSetData {
    
    pub fn load(name: &str) -> Result<TimeSetData, confy::ConfyError> {
        let dirs = directories::ProjectDirs::from("", "", "salatui").unwrap();
        let mut data_path = dirs.data_dir().to_path_buf();
        data_path.push(name);
        
        if std::path::Path::exists(&data_path) {
            let data = confy::load_path(data_path)?;
            Ok(data)
        }else{
            let err_path = data_path.to_str().unwrap().to_string();
            Err(confy::ConfyError::BadConfigDirectory(err_path))
        }
    }
    
    pub fn save(&self, name: &str) -> Result<(), confy::ConfyError>{
        let dirs = directories::ProjectDirs::from("", "", "salatui").unwrap();
        let mut data_path = dirs.data_dir().to_path_buf();
        
        if std::path::Path::exists(&data_path) {
            data_path.push(name);
            confy::store_path(&data_path, self)?;
            Ok(())
            
        } else {
            match std::fs::create_dir(&data_path) {
                Ok(_) => println!("no data directory exists. creating"),
                Err(err) => println!("failed to create directory\n{err}"),
            };
            data_path.push(name);
            confy::store_path(&data_path, self)?;
            Ok(())
        }    
    }
}




// ===================
/// Built-in support:
// ===================

// data parsed during build time
// include!(concat!(env!("OUT_DIR"), "/parsed_data.rs")); // build script output
// might try this if optimisation is required, for now gonna focus on finishing the main program
// #[derive(Debug, PartialEq, Eq)]
// pub struct MVData<'a> {
//     pub pt:     [[u32    ; 8]; 15372],
//     pub island: [[&'a str;10];   205],
//     pub atoll:  [[&'a str; 4];    20],
// }
// impl<'a> MVData<'a> {
//     pub fn from(pt: [[u32; 8]; 15372], island: [[&'a str;10]; 205], atoll:[[&'a str; 4];20]) -> Self{
//         MVData {pt,  island, atoll}
//     }
//     pub fn parse_timeset(&self, island_index: usize) -> Option<TimeSet> {
//         todo!()
//     }
// }

#[derive(Debug, PartialEq, Eq)]
/// Salatmv string dataset struct
pub struct MVRawData {
    /// atoll[x]  => 0:index, 1:eng_name, 2:dhi_name, 3:ar_name
    pub atoll: String,
    /// island[x]  => 0:timeset_index, 1:island_index, 2:atoll_index, 3:eng_name, 4:dhi_name,
    /// 5:ar_name, 6:unknown, 7:latitude, 8:longitude, 9:unknown
    pub island: String,
    /// pt[x] => 0:timeset_index, 1:day, 2:fajr, 3:sun, 4:duhur, 5:asr, 6:magrib, 7:isha
    pub pt: String,
}

impl MVRawData {
    pub fn from(pt: String, atoll: String, island: String) -> MVRawData {
        MVRawData {pt, atoll, island}
    }
    
    pub fn parse_to_timeset(&self, island_index: usize) -> Option<TimeSetData> {
        if self.pt.is_empty(){
            return None
        };
        
        // Island
        let island_data:Vec<String> = {
            let mut rows:Vec<String> = self.island.split('\n').map(str::to_string).collect();
            rows.pop();
            let table:Vec<Vec<String>> = rows.into_iter().map(|column| column.split(';').map(str::to_string).collect()).collect();
            
            table.into_iter()
                .filter(|column| column[1] == island_index.to_string())
                .flatten()
                .collect()
        };
        let island_name = island_data[3].to_string();
        
        // Atoll
        let atoll_index:usize = island_data[2].parse().unwrap();
        let atoll_data: Vec<Vec<&str>> = {
            let mut rows:Vec<&str> = self.atoll.split('\n').collect();
            rows.pop();
            
            rows.into_iter()
                .map(|column| column.split(';').collect())
                .collect()
        };
        let atoll_name = atoll_data[atoll_index][1].to_string();
        
        // PT
        let timeset_index = island_data[0].clone();
        let pt_data:Vec<Vec<u32>> = { 
            let mut rows: Vec<&str> = self.pt.split('\n').collect();
            rows.pop();
            let table: Vec<Vec<&str>> = rows.into_iter().map(|column| column.split(';').collect()).collect();
            
            table.into_iter()
                .filter(|column| column[0] == timeset_index)
                .map(|column| column.into_iter().map(|e| e.parse::<u32>().unwrap_or(444000444)).collect())
                .collect()
        };
        
        let name = format!("{atoll_name}. {island_name}");
        let details = None;
        let coordinates = (island_data[7].to_string(), island_data[8].to_string());
        
        let data = pt_data;
        
        Some(TimeSetData{name, details, coordinates, data})
    }
}
impl Default for MVRawData {
    fn default() -> Self {
        use std::fs;
        
        let atoll = fs::read_to_string("./data/atolls.csv")
            .expect("Should have been able to read the file");
        
        let pt = fs::read_to_string("./data/ptdata.csv")
            .expect("Should have been able to read the file");
        
        let island = fs::read_to_string("./data/islands.csv")
            .expect("Should have been able to read the file");
        
        MVRawData::from(pt, atoll, island)
    }
}



#[test]
fn mv_data_parse(){
    use std::fs;
    
    let atoll = fs::read_to_string("./data/atolls.csv")
        .expect("Should have been able to read the file");
    
    let pt = fs::read_to_string("./data/ptdata.csv")
        .expect("Should have been able to read the file");
    
    let island = fs::read_to_string("./data/islands.csv")
        .expect("Should have been able to read the file");
    
    let parsed = match (MVRawData{atoll, pt, island}).parse_to_timeset(177){
        Some(parsed) => parsed,
        None => panic!("parsing data")
    };
    dbg!(&parsed);
    let parsed = parsed.name;
    let expected = String::from("GDh. Vilingili");
        // details: String::new(),
        // coordinates: (
        //     String::from("0.755293"),
        //     String::from("73.434885")
        // ),
        // data: Box::new(TimeSetData {
        //     content: vec![
        //         vec![77, 0, 289, 366, 734, 937, 1095, 1172],
        //         vec![77, 1, 290, 366, 735, 937, 1095, 1173],
        //     ], ..Default::default()
        // })
    // };
    assert_eq!(parsed, expected);
}



