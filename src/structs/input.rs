#[derive(Debug, Default, Clone, Copy)]
pub struct InputMap {
    pub forward: bool,
    pub backward: bool,
    
    pub enter: bool,
    pub escape: bool,
    
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
        if self.forward   {return Some(String::from("forward  "))}
        if self.backward  {return Some(String::from("backward "))}
        if self.enter     {return Some(String::from("enter    "))}
        if self.escape    {return Some(String::from("escape   "))}
        if self.up        {return Some(String::from("up       "))}
        if self.down      {return Some(String::from("down     "))}
        None   
    }
}




