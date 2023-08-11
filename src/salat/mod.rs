use crate::*;

//chrono needed
pub fn salat_times(conf: &Config, timeset: &TimeSetData){
    // let timeset = match &conf.provider {
    //     Provider::Data(timeset) => timeset,
    //     Provider::Calculation(_) => todo!(),
    //     Provider::Manual => todo!(),
    // };
    
    let mut final_string = String::new();
    
    if conf.display.show_raw_output {
        match conf.raw_output.mode {
            RawOutputMode::Array => todo!(),
            RawOutputMode::Custom => todo!(),
            RawOutputMode::Json => todo!(),
            RawOutputMode::RawData => {final_string += &format!("{:?}", timeset.data[0])},
            RawOutputMode::TOML => {},
        };
    };
    
    dbg!(final_string);
    
    
    
}
