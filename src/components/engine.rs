use crate::components::Entity;
use std::time::Duration;

use sdl2::{
    render::WindowCanvas, 
    pixels::Color, 
    event::Event, 
    Sdl, 
};



pub struct Engine {
    entities: Vec<Entity>,
}

impl Engine {

    pub fn init() -> Engine {
        Engine { 
            entities: Vec::new(), 
        }
    }

    pub fn push_entity(&mut self, entity: Entity) { self.entities.push(entity); }

    pub fn load_texture(&mut self) {

    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {

        for entity in self.entities.iter_mut() {
            entity.update();
            entity.render(canvas);

        }
    
    }

    pub fn start(&mut self, canvas: &mut WindowCanvas, context: Sdl) {

        let mut event_pump = context.event_pump().unwrap();

        'render_loop: loop {
        
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
    
            for event in event_pump.poll_event() {
                match event {
                    Event::Quit { .. } => break 'render_loop,
                    _ => {},
                }
            }

            self.render(canvas);
            canvas.present();
            
            ::std::thread::sleep(Duration::new(0, 1_000_000_000 / 30));
        }
    }

}

