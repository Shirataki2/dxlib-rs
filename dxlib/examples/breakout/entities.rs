use dxlib::prelude::*;
use specs::*;
use crate::components::*;

#[derive(Debug)]
pub struct Paddle;

impl Paddle {
    pub fn register(
        world: &mut World,
        position: Vector2<f32>,
        size: Vector2<f32>,
        color: dxlib::color::Color<u8>,
    ) {
        world.create_entity()
            .with(ObjectType::Paddle)
            .with(Position(position))
            .with(Rect(size))
            .with(Color(color))
            .build();
    }
}

#[derive(Debug)]
pub struct Ball;
impl Ball {
    pub fn register(
        world: &mut World,
        position: Vector2<f32>,
        size: Vector2<f32>,
        color: dxlib::color::Color<u8>,
    ) {
        world.create_entity()
            .with(ObjectType::Ball)
            .with(Position(position))
            .with(Rect(size))
            .with(Color(color))
            .build();
    }
}
