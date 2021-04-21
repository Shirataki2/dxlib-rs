use std::any::Any;

use crate::application::ApplicationBuilder;

pub trait Plugin: Any + Send + Sync {
    type Error: Sized + Send + Sync + 'static;
    fn build(&self, app: &mut ApplicationBuilder) -> Result<(), Self::Error>;
}

pub mod antialias;
pub mod background;

pub use antialias::FullSceneAntiAliasPlugin;
pub use background::BackgroundPlugin;
