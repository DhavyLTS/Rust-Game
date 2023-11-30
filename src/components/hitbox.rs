use sdl2::{rect::Rect, pixels::Color};

#[derive(Debug, Clone)]
pub struct Hitbox { 
    geometry: Rect,
    color: Color,
}

impl Hitbox {

    pub fn new(x:i32, y:i32, width:u32, height:u32) -> Hitbox {
        Hitbox {
            color: Color::RGB(255,255,255),
            geometry: Rect::new(x, y, width, height),
        }
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = Color::RGB(r, g, b);
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_geometry(&self) -> Rect {
        self.geometry
    }

}