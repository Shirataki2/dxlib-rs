#![allow(clippy::needless_range_loop)]
#![cfg_attr(feature = "unstable", allow(incomplete_features))]
#![cfg_attr(feature = "unstable", feature(const_generics))]
#![cfg_attr(feature = "unstable", feature(const_evaluatable_checked))]
pub mod application;
pub mod color;
pub mod dx3d;
pub mod error;
pub mod ext;
pub mod graphics;
pub mod input;
pub mod math;
pub mod plugin;
pub mod prelude;
pub mod screen;
pub mod sound;
pub mod utils;
pub mod writer;

#[macro_use]
extern crate anyhow;
