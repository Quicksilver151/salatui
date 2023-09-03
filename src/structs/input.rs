// #[derive(Debug, Default, Clone, Copy)]
// pub struct InputMap {
//     pub forward: bool,
//     pub backward: bool,
//     
//     pub enter: bool,
//     pub escape: bool,
//     
//     pub up: bool,
//     pub left: bool,
//     pub down: bool,
//     pub right: bool,
// }


#[derive(Debug, Default, Clone, Copy)]
pub enum Modifier {
    #[default]
    None,
    Shift,
    Alt,
    Ctrl,
}
#[derive(Debug, Default, Clone, Copy)]
pub enum Key {
    #[default]
    None,
    
    Up,
    Down,
    Left,
    Right,
    
    Back,
    Forward,
    
    Enter,
    Escape,
    
    Command(char),
}
#[derive(Debug, Default, Clone)]
pub struct InputMap {
    key: Key,
    modifier: Modifier
}


impl InputMap {
    pub fn reset(&mut self) {
        *self = InputMap::default();
    }
    
    pub fn set_input(&mut self, key: Key) {
        self.key = key;
    }
    
    pub fn get_command(&self) -> Option<char> {
        if let Key::Command(x) = self.key{
            Some(x)
        }
        else {
            None
        }
    }
    
    pub fn get_key(&self) -> Option<(Key, Modifier)> {
        Some((self.key, self.modifier))
    }
    pub fn set_modifier(&mut self, modifier: Modifier) {
        self.modifier = modifier;
    }
    
}



