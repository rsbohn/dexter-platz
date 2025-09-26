pub mod voxel;
pub mod world;
pub mod meshing;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use crate::meshing::{mesh_chunk, SurfaceMesh};
use crate::voxel::Voxel;
use crate::world::{Chunk, CHUNK_SIZE};

const WORLD_DIM: u32 = 9; // 9x9x9 chunks

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<bevy::audio::AudioPlugin>())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Lighting
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 100.0, 50.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight { shadows_enabled: true, illuminance: 30_000.0, ..default() },
        ..default()
    });

    // Camera
    let world_size = Vec3::new(
        (WORLD_DIM * CHUNK_SIZE as u32) as f32,
        (WORLD_DIM * CHUNK_SIZE as u32) as f32,
        (WORLD_DIM * CHUNK_SIZE as u32) as f32,
    );
    let center = world_size / 2.0;
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(center + Vec3::new(80.0, 120.0, 160.0))
            .looking_at(center, Vec3::Y),
        ..default()
    });

    // Simple worldgen: a single global ground layer at global Y = 0.
    for cz in 0..WORLD_DIM {
        for cy in 0..WORLD_DIM {
            for cx in 0..WORLD_DIM {
                let mut chunk = Chunk::new();
                generate_chunk_ground(cx, cy, cz, &mut chunk);

                let smesh = mesh_chunk(&chunk);
                if smesh.indices.is_empty() { continue; }

                let bevy_mesh = surface_to_bevy_mesh(&smesh);
                let mesh_handle = meshes.add(bevy_mesh);

                // Simple material (tinted green)
                let mat_handle = materials.add(StandardMaterial {
                    base_color: Color::rgb(0.35, 0.8, 0.4),
                    perceptual_roughness: 0.9,
                    ..default()
                });

                let tx = Vec3::new(
                    (cx * CHUNK_SIZE as u32) as f32,
                    (cy * CHUNK_SIZE as u32) as f32,
                    (cz * CHUNK_SIZE as u32) as f32,
                );

                commands.spawn(PbrBundle {
                    mesh: mesh_handle,
                    material: mat_handle,
                    transform: Transform::from_translation(tx),
                    ..default()
                });
            }
        }
    }
}

fn generate_chunk_ground(cx: u32, cy: u32, _cz: u32, chunk: &mut Chunk) {
    // Place ground only on the world layer where global Y == 0
    if cy != 0 { return; }
    for z in 0..CHUNK_SIZE as u32 {
        for x in 0..CHUNK_SIZE as u32 {
            chunk.set(x, 0, z, Voxel(1));
        }
    }
}

fn surface_to_bevy_mesh(s: &SurfaceMesh) -> Mesh {
    let mut m = Mesh::new(PrimitiveTopology::TriangleList);
    m.insert_attribute(Mesh::ATTRIBUTE_POSITION, s.positions.clone());
    m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, s.normals.clone());
    m.insert_attribute(Mesh::ATTRIBUTE_UV_0, s.uvs.clone());
    m.insert_indices(Indices::U32(s.indices.clone()));
    m
}
