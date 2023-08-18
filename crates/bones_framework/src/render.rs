//! Rendering components.

use bones_lib::prelude::*;

/// Module prelude.
pub mod prelude {
    pub use super::{
        audio::*, camera::*, color::*, line::*, sprite::*, tilemap::*, transform::*, ui::*,
        Renderer, RendererApi,
    };
}

pub mod audio;
pub mod camera;
pub mod color;
pub mod line;
pub mod sprite;
pub mod tilemap;
pub mod transform;
pub mod ui;

/// Trait for the interface exposed by external bones renderers.
///
/// These methods allow the game to notify the renderer when certain things happen, and to allow the
/// game to instruct the renderer to do certain things.
///
///
pub trait RendererApi: Sync + Send {
    /// Have the renderer delete the session.
    ///
    /// The default implementation doesn't do anything, and that may be appropriate for some
    /// renderers. Other renderers may need to clean up synchronized entities that are present in
    /// the deleted session.
    fn delete_session(&self, session: Session) {
        let _ = session;
    }
}

/// Resource containing the [`RendererApi`] implementation provided by the bones renderer.
#[derive(HasSchema)]
#[schema(opaque, no_clone, no_default)]
pub struct Renderer(Box<dyn RendererApi>);