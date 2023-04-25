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
            quit:   false,
            shrink: false,
            next:   false,
            prev:   false,
            enter:  false,
            back:   false,
            up:     false,
            down:   false,
        }
    }
    pub fn reset(&mut self) {
        self.rename = false;
        self.quit   = false;
        self.shrink = false;
        self.next   = false;
        self.prev   = false;
        self.enter  = false;
        self.back   = false;
        self.up     = false;
        self.down   = false;
    }
    pub fn get_current(&mut self) -> &str{
        if self.rename{return "rename"}
        if self.quit  {return "quit  "}
        if self.shrink{return "shrink"}
        if self.next  {return "next  "}
        if self.prev  {return "prev  "}
        if self.enter {return "enter "}
        if self.back  {return "back  "}
        if self.up    {return "up    "}
        if self.down  {return "down  "}
        ""
    }
}

impl Default for InputMap{
    fn default() -> Self {
        Self::new()
    }
}



