use bevy::prelude::*;

mod camera;
pub mod grid;
pub mod input;
pub mod turn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, input::plugin, camera::plugin));
}
