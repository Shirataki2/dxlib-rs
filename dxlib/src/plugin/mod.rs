use std::any::Any;

use crate::application::ApplicationBuilder;

pub trait Plugin: Any + Send + Sync {
    fn build(&self, app: &mut ApplicationBuilder);
}

pub mod basic;

pub use basic::BackgroundPlugin;
