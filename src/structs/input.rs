use crate::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct InputMap {
    pub enter: bool,
    pub back: bool,
    
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
}

impl InputMap {
    pub fn reset(&mut self) {
        *self = InputMap::default();
    }
    pub fn get_current(&self) -> Option<String> {
        if self.enter {return Some(String::from("enter "))}
        if self.back  {return Some(String::from("back  "))}
        if self.up    {return Some(String::from("up    "))}
        if self.down  {return Some(String::from("down  "))}
        None   
    }
}




