#[derive(Debug, Default)]
pub struct InputMap {
    pub enter: bool,
    pub back: bool,
    
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
    
    pub command: char,
}
impl InputMap {
    pub fn reset(&mut self) {
        *self = InputMap::default();
    }
    pub fn get_current(&mut self) -> String {
        if self.enter {return String::from("enter ")}
        if self.back  {return String::from("back  ")}
        if self.up    {return String::from("up    ")}
        if self.down  {return String::from("down  ")}
        self.command.to_string()
    }
}




