use components::AnimationType;
use sdl2::{self, rect::Point};

mod components;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("SDL2 Engine", 800, 600).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut engine = components::Engine::init();
    let mut entity = components::Entity::new();
    
    let mut animation = components::Animation::new("assets/knight/idle.png", AnimationType::Idle, 10, 3, Point::new(44,42));

    animation.push_hitbox(components::Hitbox::new(0, 0, 80, 32));
    
    entity.push_animation(animation, AnimationType::Idle);
    engine.push_entity(entity);

    engine.start(&mut canvas, context);
}
