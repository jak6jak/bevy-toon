//pub mod render_graph;

mod entity;
mod material;

use bevy::ecs::IntoSystem;
pub use entity::*;

pub mod prelude {
    pub use crate::{entity::*, material::StandardMaterial};
}



#[derive(Default)]
pub struct ToonPlugin;

impl Plugin for ToonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        
    }
}