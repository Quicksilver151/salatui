use crate::*;

#[derive(Debug)]
pub struct InputMap {
    pub rename: bool,
    pub quit: bool,
    pub shrink: bool,
}
impl InputMap {
    pub fn new() -> InputMap {
        InputMap { rename: false, quit: false, shrink: false}
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



