use derive_more::{Deref, DerefMut};
use dxlib::prelude::*;
use specs::*;

#[allow(dead_code)]
#[derive(Debug, Component, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ObjectType {
    Paddle,
    Ball,
    Brick,
    Empty,
    UI,
}

#[derive(Debug, Component, Deref, DerefMut)]
#[storage(VecStorage)]
pub struct Position(pub Vector2<f32>);

#[derive(Debug, Component, Deref, DerefMut)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector2<f32>);

#[derive(Debug, Component, Deref, DerefMut)]
#[storage(VecStorage)]
pub struct Rect(pub Vector2<f32>);

#[derive(Debug, Component, Deref, DerefMut)]
#[storage(VecStorage)]
pub struct Color(pub dxlib::color::Color<u8>);
