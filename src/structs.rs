use crate::*;

#[derive(Debug, Default)]
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
        InputMap::default()
    }
    pub fn reset(&mut self) {
        *self = InputMap::default();
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




