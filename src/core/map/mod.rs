use bevy::prelude::*;

use crate::core::Material;
pub use boundaries::*;
pub use combobox::*;
pub use elevator::*;
pub use player::*;
pub use spawn_point::*;
pub use wall::*;

mod boundaries;
mod combobox;
mod elevator;
mod player;
mod spawn_point;
mod wall;

pub struct MapBuilder<'w, 's, 'a, 'b> {
    builder: &'b mut ChildBuilder<'w, 's, 'a>,
    meshes: &'b mut Assets<Mesh>,
    materials: &'b mut Assets<Material>,
    assets: &'b mut AssetServer,
    clear_color: &'b mut ClearColor,
    boundaries: &'b mut MapBoundaries,

    wall_material: Handle<Material>,
}

impl<'w, 's, 'a, 'b> MapBuilder<'w, 's, 'a, 'b> {
    pub fn new(
        builder: &'b mut ChildBuilder<'w, 's, 'a>,
        meshes: &'b mut Assets<Mesh>,
        materials: &'b mut Assets<Material>,
        clear_color: &'b mut ClearColor,
        boundaries: &'b mut MapBoundaries,
        assets: &'b mut AssetServer,
    ) -> MapBuilder<'w, 's, 'a, 'b> {
        let wall_material = materials.add(Color::rgb(0.1, 0.1, 0.1).into());

        MapBuilder {
            builder,
            meshes,
            materials,
            wall_material,
            clear_color,
            boundaries,
            assets,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.clear_color.0 = color;
    }
}