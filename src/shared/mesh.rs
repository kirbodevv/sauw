use bevy::{
    asset::RenderAssetUsages,
    ecs::system::SystemParam,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::game::assets::atlas::{AtlasAsset, TextureId};

#[derive(SystemParam)]
pub struct RenderParam<'w> {
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
}

impl RenderParam<'_> {
    pub fn add_mesh(&mut self, mesh: Mesh) -> Handle<Mesh> {
        self.meshes.add(mesh)
    }

    pub fn add_material(
        &mut self,
        texture: Handle<Image>,
        color: Option<Color>,
    ) -> Handle<ColorMaterial> {
        self.materials.add(ColorMaterial {
            texture: Some(texture.clone()),
            color: color.unwrap_or_default(),
            ..default()
        })
    }
}

#[derive(Default)]
pub struct MeshBuilder {
    positions: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl MeshBuilder {
    pub fn append_quad(
        &mut self,
        texture: TextureId,
        atlas: &AtlasAsset,
        position: Vec3,
        size: Vec2,
        offset: Vec2,
    ) {
        let atlas_entry = &atlas.get(texture);

        let tex_x = atlas_entry.x() as f32;
        let tex_y = atlas_entry.y() as f32;
        let tex_w = atlas_entry.width() as f32;
        let tex_h = atlas_entry.height() as f32;
        let atlas_w = atlas.width as f32;
        let atlas_h = atlas.height as f32;

        let pad = 0.5;
        let u0 = (tex_x + pad) / atlas_w;
        let v0 = (tex_y + pad) / atlas_h;
        let u1 = (tex_x + tex_w - pad) / atlas_w;
        let v1 = (tex_y + tex_h - pad) / atlas_h;

        let base = self.positions.len() as u32;

        let x = position.x + size.x / 2.0 + offset.x;
        let y = position.y + size.y / 2.0 + offset.y;
        let z = position.z;

        let hw = size.x / 2.0;
        let hh = size.y / 2.0;

        self.positions.extend_from_slice(&[
            [x - hw, y - hh, z],
            [x + hw, y - hh, z],
            [x - hw, y + hh, z],
            [x + hw, y + hh, z],
        ]);
        self.uvs
            .extend_from_slice(&[[u0, v1], [u1, v1], [u0, v0], [u1, v0]]);
        self.indices
            .extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 1, base + 3]);
    }

    pub fn build(&self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone())
        .with_inserted_indices(Indices::U32(self.indices.clone()))
    }
}
