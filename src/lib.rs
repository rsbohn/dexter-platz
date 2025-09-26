pub mod meshing;
pub mod voxel;
pub mod world;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

use crate::meshing::{mesh_chunk, SurfaceMesh};
use crate::voxel::Voxel;
use crate::world::{Chunk, CHUNK_SIZE};

const WORLD_DIM: u32 = 9; // 9x9x9 chunks

fn height_at(world_x: f32, world_z: f32) -> f32 {
    let coarse = (world_x * 0.05).sin() + (world_z * 0.05).cos();
    let medium = ((world_x + world_z) * 0.02).sin();
    let detail = (world_x * 0.14).cos() * (world_z * 0.14).sin();
    let height = coarse * 4.5 + medium * 7.5 + detail * 2.0 + 14.0;
    height.max(0.0)
}

#[derive(Component)]
struct FlyCamera;

#[derive(Component)]
struct AnimatedLight {
    speed: f32,
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.build())
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_controls, animate_light))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Lighting
    commands
        .spawn(DirectionalLightBundle {
            transform: Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 30_000.0,
                ..default()
            },
            ..default()
        })
        .insert(AnimatedLight { speed: 0.25 });

    // Camera
    let world_size = Vec3::new(
        (WORLD_DIM * CHUNK_SIZE as u32) as f32,
        (WORLD_DIM * CHUNK_SIZE as u32) as f32,
        (WORLD_DIM * CHUNK_SIZE as u32) as f32,
    );
    let center = world_size / 2.0;
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(center + Vec3::new(80.0, 120.0, 160.0))
                .looking_at(center, Vec3::Y),
            ..default()
        })
        .insert(FlyCamera);

    let ground_texture = asset_server.load("textures/ground.png");
    let dirt_texture = asset_server.load("textures/dirt.png");
    let stone_texture = asset_server.load("textures/stone.png");

    let ground_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.75, 0.45),
        base_color_texture: Some(ground_texture.clone()),
        perceptual_roughness: 0.85,
        reflectance: 0.02,
        ..default()
    });
    let dirt_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.32, 0.22),
        base_color_texture: Some(dirt_texture.clone()),
        perceptual_roughness: 0.95,
        reflectance: 0.01,
        ..default()
    });
    let stone_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.52),
        base_color_texture: Some(stone_texture.clone()),
        perceptual_roughness: 0.7,
        reflectance: 0.05,
        ..default()
    });

    for cz in 0..WORLD_DIM {
        for cy in 0..WORLD_DIM {
            for cx in 0..WORLD_DIM {
                let mut chunk = Chunk::new();
                populate_chunk_heightfield(cx, cy, cz, &mut chunk);

                let smesh = mesh_chunk(&chunk);
                if smesh.indices.is_empty() {
                    continue;
                }

                let bevy_mesh = surface_to_bevy_mesh(&smesh);
                let mesh_handle = meshes.add(bevy_mesh);

                let tx = Vec3::new(
                    (cx * CHUNK_SIZE as u32) as f32,
                    (cy * CHUNK_SIZE as u32) as f32,
                    (cz * CHUNK_SIZE as u32) as f32,
                );

                let sample_x = tx.x + CHUNK_SIZE as f32 * 0.5;
                let sample_z = tx.z + CHUNK_SIZE as f32 * 0.5;
                let sample_height = height_at(sample_x, sample_z);
                let material_handle = if sample_height > 28.0 {
                    stone_material.clone()
                } else if sample_height > 18.0 {
                    dirt_material.clone()
                } else {
                    ground_material.clone()
                };

                commands.spawn(PbrBundle {
                    mesh: mesh_handle,
                    material: material_handle,
                    transform: Transform::from_translation(tx),
                    ..default()
                });
            }
        }
    }

    // Highlight the world center with a white voxel-sized cube.
    let cube_mesh = meshes.add(Mesh::from(Cuboid::default()));
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.95, 0.95),
        ..default()
    });
    let ground_height = height_at(center.x, center.z);
    let cube_translation = Vec3::new(center.x - 0.5, ground_height + 10.5, center.z - 0.5);
    commands.spawn(PbrBundle {
        mesh: cube_mesh,
        material: cube_material,
        transform: Transform::from_translation(cube_translation),
        ..default()
    });
}

fn populate_chunk_heightfield(cx: u32, cy: u32, cz: u32, chunk: &mut Chunk) {
    let chunk_size = CHUNK_SIZE as u32;
    let world_y_base = (cy * chunk_size) as i32;
    let max_world_height = (WORLD_DIM * chunk_size - 1) as i32;

    for z in 0..chunk_size {
        let world_z = (cz * chunk_size + z) as f32;
        for x in 0..chunk_size {
            let world_x = (cx * chunk_size + x) as f32;
            let target_height = height_at(world_x, world_z).floor() as i32;
            let clamped_height = target_height.clamp(0, max_world_height);

            for y in 0..chunk_size {
                let world_y = world_y_base + y as i32;
                if world_y <= clamped_height {
                    chunk.set(x, y, z, Voxel(1));
                }
            }
        }
    }
}

fn surface_to_bevy_mesh(s: &SurfaceMesh) -> Mesh {
    let mut m = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    m.insert_attribute(Mesh::ATTRIBUTE_POSITION, s.positions.clone());
    m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, s.normals.clone());
    m.insert_attribute(Mesh::ATTRIBUTE_UV_0, s.uvs.clone());
    m.insert_indices(Indices::U32(s.indices.clone()));
    m
}

fn camera_controls(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
    mut query: Query<&mut Transform, With<FlyCamera>>,
) {
    if keys.just_pressed(KeyCode::Backspace) {
        exit.send(AppExit::Success);
    }

    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyX) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyZ) {
        direction.y -= 1.0;
    }

    if direction == Vec3::ZERO {
        return;
    }

    let delta = direction.normalize() * 50.0 * time.delta_seconds();
    if let Ok(mut transform) = query.get_single_mut() {
        transform.translation += delta;
    }
}

fn animate_light(time: Res<Time>, mut lights: Query<(&AnimatedLight, &mut DirectionalLight)>) {
    for (animated, mut light) in &mut lights {
        let cycle = time.elapsed_seconds() * animated.speed;
        let normalized = cycle.rem_euclid(3.0);

        let (from, to, t) = if normalized < 1.0 {
            (Vec3::new(0.2, 0.3, 1.0), Vec3::splat(1.0), normalized)
        } else if normalized < 2.0 {
            (
                Vec3::splat(1.0),
                Vec3::new(1.0, 0.25, 0.25),
                normalized - 1.0,
            )
        } else {
            (
                Vec3::new(1.0, 0.25, 0.25),
                Vec3::new(0.2, 0.3, 1.0),
                normalized - 2.0,
            )
        };

        let rgb = from.lerp(to, t);
        light.color = Color::srgb(rgb.x, rgb.y, rgb.z);
    }
}
