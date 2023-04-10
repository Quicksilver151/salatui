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
    pub fn from<B: Backend>(f: &mut Frame<B>, shrink: bool) -> MainLayout {
        
        let menu: Vec<Rect> = get_menu_layout(f);     
        
        let mut container_size : Vec<f64> = vec![];
        if shrink {
        container_size.append(&mut vec![1.0]);
        } else {
        container_size.append(&mut vec![2.0]);
        }
        container_size.append(&mut vec![2.0]);
        container_size.append(&mut vec![1.0]);
        container_size.append(&mut vec![1.0]);
        
        let settings: Vec<Rect> = get_settings_layout(f, container_size);
        
        MainLayout { menu , settings }
    }
}






