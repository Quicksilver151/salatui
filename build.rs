use std::fs::{write, read_to_string};
use std::env;
use std::path::Path;

trait PTDataParse {
    fn parse_for_island(self) -> Vec<Vec<u32>>;
}

impl PTDataParse for String{
    fn parse_for_island(self) -> Vec<Vec<u32>>{

        // split by line for each valid data
        let mut grouped :Vec<&str> = self.split('\n').collect();
        grouped.pop(); // remove last line
        grouped.reverse();
        grouped.pop(); // remove first line
        grouped.reverse();
        
        
        // let mut full_list: [[u32; 8]; 15372];
        let mut full_list: Vec<Vec<u32>> = vec![];
        
        // split by column for each valid data
        for group in grouped.iter(){
            let columns: Vec<u32> = group.split(';').map(|x| x.parse::<u32>().unwrap_or(0)).collect();
            
            // skip irrelevant data
            // if island_index != columns[0]{
                // continue;
            // }
            
            let result = columns;
            // let mut result : PrayerData = PrayerData { island_index: (0), day: (0), fajr: (0), sun: (0), dhuhur: (0), asr: (0), magrib: (0), isha: (0) };
            
            // result.island_set_from_vec(columns.iter().map(|x| x.parse::<u32>().unwrap()).collect());
            full_list.append(&mut vec![result]);
        }
        
        full_list   
    }
}

fn get_vec_from_db(db: &str) -> Vec<String> {
    let mut grouped: Vec<&str> = db.split('\n').collect();
    grouped.pop();
    grouped
        .iter()
        .map(|x| x.parse::<String>().unwrap())
        .collect()
}

fn format_as_rust_vec(atoll_data: Vec<String>, island_data: Vec<String>) -> String{
    let mut string : String = String::new();
    string.push_str("pub static ATOLL_DATA : & [[&str;4]; 20] = &[");
    for atoll in atoll_data{
        
        let item = format!("\n[\"{}\"],",atoll.replace(';', "\",\""));
        string.push_str(&item);
    }
    string.push_str("];\n\n");
    
    
    string.push_str("pub static ISLAND_DATA : & [[&str;10]; 205] = &[");
    for island in island_data{
        let item = format!("\n[\"{}\"],",island.replace(';', "\",\""));
        string.push_str(&item);
    }
    string.push_str("];");

    println!("{}",string);
    string
    
}

fn format_as_rust_array(pt_data:Vec<Vec<u32>>) -> String{
    let mut string : String = "".to_string();
    let default_str: String =
"pub static PT_DATA: & [[u32; 8]; 15372] = &[".to_string();
    string.push_str(&default_str);
    for i in pt_data{
        string.push_str(&format!("{:?},\n",i));
        
        
    }
    string.push_str("];");
    
    string
    
}

#[allow(unused)]
fn main(){
    let data : &str = include_str!("./data/ptdata.csv");
    let pt_data = data.to_string().parse_for_island();
    let pt_data = format_as_rust_array(pt_data);
    
    let atoll_data = get_vec_from_db(&read_to_string("./data/atolls.csv" ).unwrap());
    let island_dat = get_vec_from_db(&read_to_string("./data/islands.csv").unwrap());
    
    let atoll_and_island_data = format_as_rust_vec(atoll_data, island_dat);
    let final_string = format!("{}\n\n{}", atoll_and_island_data, pt_data);
    
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("parsed_data.rs");
    
    write(dest_path, final_string).unwrap_or(());
    
    println!("cargo:rerun-if-changed=build.rs")   
}
