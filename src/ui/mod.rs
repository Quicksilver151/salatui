use crate::*;

mod menu;
mod settings;

pub use menu::*;
pub use settings::*;

pub struct MainLayout {
    pub menu : Vec<Rect>,
    pub settings: Vec<Rect>,
}

impl MainLayout {
    pub fn from<B: Backend>(f: &mut Frame<B>) -> MainLayout {
        
        let menu: Vec<Rect> = get_menu_layout(f);
        let settings: Vec<Rect> = get_settings_layout(f);
        
        MainLayout { menu , settings }
    }
}






