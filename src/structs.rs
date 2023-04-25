use crate::*;

#[derive(Debug)]
pub struct InputMap {
    pub rename: bool,
    pub quit: bool,
    pub shrink: bool,
    
    pub next: bool,
    pub prev: bool,
    
    pub enter: bool,
    pub back: bool,
    
    pub up: bool,
    pub down: bool,
}
impl InputMap {
    pub fn new() -> InputMap {
        InputMap {
            rename: false,
            quit: false,
            shrink: false,
            next: false,
            prev: false,
            enter: false,
            back: false,
            up: false,
            down: false,
        }
    }
    pub fn reset(&mut self) {
        self.rename = false;
        self.quit   = false;
        self.shrink = false;
    }
}

impl Default for InputMap{
    fn default() -> Self {
        Self::new()
    }
}



