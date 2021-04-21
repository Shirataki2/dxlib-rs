use std::any::Any;

use crate::application::ApplicationBuilder;

pub trait Plugin: Any + Send + Sync {
    type Error: Sized + Send + Sync + 'static;
    fn build(&self, app: &mut ApplicationBuilder) -> Result<(), Self::Error>;
}

pub mod background;
pub mod antialias;

pub use background::BackgroundPlugin;
pub use antialias::FullSceneAntiAliasPlugin;