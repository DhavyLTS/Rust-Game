use crate::components::Hitbox;

use sdl2::{
    render::WindowCanvas, 
    rect::{Point, Rect}, image::LoadTexture
};


#[derive(Clone, Hash, PartialEq, Eq)]
pub enum AnimationType { 
    Attack,
    Idle,
    Fall,
    Run,
}
#[derive(Clone)]
pub struct Animation {
    animation_type: AnimationType,
    hitboxes: Vec<Hitbox>,
    filename: String,
    position: Point,
    frame_time: i32,
    offset: Point,
    frames: i32,
    frame: i32,
    scale: i32,
    count: i32,
}

impl Animation {

    pub fn new(file: &str, animation_type: AnimationType, frames: i32, frame_time: i32, offset: Point) -> Animation {
        Animation {
            filename: String::from(file),
            position: Point::new(0,0),
            hitboxes: Vec::new(),
            animation_type,
            frame_time,
            scale: 1,
            frame: 0,
            count: 0,
            offset,
            frames,
        }
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {

        self.count += 1; 
        
        if self.count > self.frame_time  {
   
            self.frame += 1;

            if self.frame >= self.frames {
                self.frame = 0;
            }

            self.count = 0;

        }
    
             
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture(&self.filename).unwrap();
        
        let _ = canvas.copy(
            &texture, 
            Rect::new(120 * self.frame, 0, 120, 80),
            Rect::new(
                self.position.x - (self.offset.x * self.scale), 
                self.position.y - (self.offset.y * self.scale), 
                (120 * self.scale).try_into().unwrap(), 
                (80 * self.scale).try_into().unwrap()
            )
        );
        
    }

    pub fn set_position(&mut self, position: Point) { self.position = position; }
    pub fn push_hitbox(&mut self, hitbox: Hitbox) { self.hitboxes.push(hitbox);}
    pub fn set_offset(&mut self, offset: Point) { self.offset = offset; }
    pub fn get_hitbox(&self) -> &Hitbox { &self.hitboxes[0] }
    
}