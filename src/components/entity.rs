use std::collections::HashMap;

use crate::components::{AnimationType, Animation};
use sdl2::{rect::Point, render::WindowCanvas};

use super::animation;

pub enum EntityType {
    Enviroment,
    Player,
    Enemy,
}

pub enum Direction {
    Right,
    Down,
    Left,
    Up, 
}

pub struct Entity { 
    animations: HashMap<AnimationType, Animation>,
    animation: AnimationType,
    position: Point,
}


impl Entity { 

    pub fn new() -> Entity { 
        Entity { 
            animation: AnimationType::Idle,
            animations: HashMap::new(),
            position: Point::new(0,0),
        }
    }

    pub fn set_position(&mut self, position: Point) {
        self.position = position;
    }

    pub fn get_position(&self) -> & Point {
        &self.position
    }

    pub fn push_animation(&mut self, animation: Animation, animation_type: AnimationType) {
        self.animations.insert(animation_type, animation);
    }

    pub fn get_animation(&mut self) -> &mut Animation {
        let animation = self.animations.get_mut(&self.animation).unwrap();
        animation
    }

    pub fn update(&mut self) {
        let position = self.position.clone();
        let animation = self.get_animation();
    
        animation.set_position(position);
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        let animation = self.get_animation();

        animation.render(canvas);
    }

}