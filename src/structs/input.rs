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
    
    // commands
    Quit,
    Fullscreen,
    Config,
    Calender,
    
    Command(char),
}
#[derive(Debug, Default, Clone)]
pub struct InputMap {
    key: Key,
    modifier: Modifier
}


impl InputMap {
    
    pub fn map_inputs(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::{KeyCode, KeyModifiers};
        match key.code {
            KeyCode::Right => self.set_input(Key::Right), //app_state.day_offset += 1},
            KeyCode::Left  => self.set_input(Key::Left), //app_state.day_offset -= 1},
            KeyCode::Enter => self.set_input(Key::Enter),
            KeyCode::Backspace | KeyCode::Esc => self.set_input(Key::Escape),//app_state.day_offset = 0,
            
            KeyCode::Up   | KeyCode::BackTab => self.set_input(Key::Up),
            KeyCode::Down | KeyCode::Tab     => self.set_input(Key::Down),
            
            KeyCode::Char(x) => self.set_input(Key::Command(x)),
            _ => {}
        }
        match key.modifiers {
            KeyModifiers::SHIFT   => self.set_modifier(Modifier::Shift),
            KeyModifiers::CONTROL => self.set_modifier(Modifier::Ctrl ),
            KeyModifiers::ALT     => self.set_modifier(Modifier::Alt  ),
            _ => {},
        }
    }
    
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



