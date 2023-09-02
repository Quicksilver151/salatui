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


#[derive(Debug, Clone, Copy)]
pub enum Modifier {
    Shift,
    Alt,
    Ctrl,
}
#[derive(Debug, Clone, Copy)]
pub enum Key {
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
    key: Option<Key>,
    modifiers: Option<Vec<Modifier>>
}


impl InputMap {
    pub fn reset(&mut self) {
        *self = InputMap::default();
    }
    
    pub fn set_input(&mut self, key: Key) {
        self.key = Some(key);
    }
    
    pub fn get_command(&self) -> Option<char> {
        if let Some(Key::Command(x)) = self.key{
            Some(x)
        }
        else {
            None
        }
    }
    
    pub fn get_key(&self) -> Option<Key> {
        self.key
    }
    
}



