#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate bitflags;

use bevy::prelude::*;

use bevy::render::render_resource::PrimitiveTopology;

mod font_loader;
mod mesh_cache;
mod mesh_data_generator;
mod mesh_system;
mod text_mesh;

pub mod prelude {
    pub use crate::font_loader::TextMeshFont;
    pub use crate::text_mesh::*;
    pub use crate::TextMeshPlugin;
    pub use glyph_brush_layout::{HorizontalAlign, VerticalAlign};
    pub use crate::generate_text_mesh;
    pub use ttf2mesh::{TTFFile};
    pub use crate::mesh_cache::TTF2MeshCache;
}

use mesh_cache::TTF2MeshCache;
pub use prelude::*;

use ttf2mesh::{TTFFile};

use mesh_data_generator::generate_text_mesh as generate_text_mesh_inner;
use mesh_system::apply_mesh;

pub fn generate_text_mesh(
    text_mesh: &TextMesh,
    font: &mut TTFFile,
    meshes: &mut Assets<Mesh>,
    cache: Option<&mut TTF2MeshCache>,
) -> Handle<Mesh> {
    let ttf2_mesh = generate_text_mesh_inner(&text_mesh, font, cache);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    apply_mesh(ttf2_mesh, &mut mesh);

    meshes.add(mesh)
}

pub struct TextMeshPlugin;

impl Plugin for TextMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<font_loader::TextMeshFont>()
            .add_system(mesh_system::text_mesh)
            .add_system(mesh_system::font_loaded)
            .insert_resource(TTF2MeshCache::default())
            .init_asset_loader::<font_loader::FontLoader>();
    }
}
