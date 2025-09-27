pub mod meshing;
pub mod voxel;
pub mod world;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::screenshot::ScreenshotManager;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::meshing::{mesh_chunk, SurfaceMesh};
use crate::voxel::Voxel;
use crate::world::{Chunk, CHUNK_SIZE};

const WORLD_DIM: u32 = 9; // 9x9x9 chunks
const PROJECT_NAME: &str = "dexter-platz";

#[derive(Default, Resource)]
struct HudState {
    entity: Option<Entity>,
    message: String,
    dirty: bool,
}

#[derive(Default, Resource)]
struct CameraRegistry {
    cameras: Vec<Entity>,
    active: usize,
}

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

#[derive(Component)]
struct GroundVehicle;

#[derive(Component)]
struct HudText;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.build())
        .init_resource::<CameraRegistry>()
        .init_resource::<HudState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                camera_controls,
                vehicle_controls,
                screenshot_capture,
                animate_light,
                cycle_cameras,
                update_hud,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut camera_registry: ResMut<CameraRegistry>,
    mut hud_state: ResMut<HudState>,
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
    let camera_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 0.6, 1.6)));
    let camera_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.8, 0.3),
        emissive: Color::srgb(0.05, 0.03, 0.01).into(),
        perceptual_roughness: 0.2,
        reflectance: 0.08,
        ..default()
    });
    let camera_start = center + Vec3::new(80.0, 120.0, 160.0);
    let mut camera_rig = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(camera_start).looking_at(center, Vec3::Y),
            ..default()
        },
        FlyCamera,
    ));
    let mut fly_camera_entity = None;
    camera_rig.with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: camera_mesh.clone(),
            material: camera_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -0.4, 0.0))
                .with_scale(Vec3::new(0.8, 0.6, 0.8)),
            ..default()
        });
        let camera = parent
            .spawn(Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.4, 0.0)),
                ..default()
            })
            .id();
        fly_camera_entity = Some(camera);
    });
    if let Some(entity) = fly_camera_entity {
        camera_registry.active = camera_registry.cameras.len();
        camera_registry.cameras.push(entity);
    }

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

    // Ground vehicle rig
    let vehicle_mesh = meshes.add(Mesh::from(Cuboid::new(2.4, 1.2, 4.0)));
    let vehicle_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.36, 0.4),
        perceptual_roughness: 0.6,
        metallic: 0.1,
        reflectance: 0.05,
        ..default()
    });
    let vehicle_pos = Vec3::new(center.x + 40.0, 0.0, center.z + 20.0);
    let vehicle_height = height_at(vehicle_pos.x, vehicle_pos.z);
    let vehicle_translation = Vec3::new(vehicle_pos.x, vehicle_height + 1.2, vehicle_pos.z);
    let mut vehicle_rig = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(vehicle_translation).looking_at(center, Vec3::Y),
            ..default()
        },
        GroundVehicle,
    ));
    let mut vehicle_camera_entity = None;
    vehicle_rig.with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: vehicle_mesh.clone(),
            material: vehicle_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        });
        let camera = parent
            .spawn(Camera3dBundle {
                camera: Camera {
                    is_active: false,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 1.2, 0.0)),
                ..default()
            })
            .id();
        vehicle_camera_entity = Some(camera);
    });
    if let Some(entity) = vehicle_camera_entity {
        camera_registry.cameras.push(entity);
    }

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        ..default()
    });

    let font = asset_server.load("embedded://bevy_text/FiraMono-Regular.ttf");

    // HUD overlay
    let hud_entity = commands
        .spawn((
            TextBundle {
                text: Text::from_sections([
                    TextSection::new(
                        format!("{PROJECT_NAME}\n"),
                        TextStyle {
                            font: font.clone(),
                            font_size: 26.0,
                            color: Color::WHITE,
                        },
                    ),
                    TextSection::new(
                        "Press P to capture screenshot",
                        TextStyle {
                            font,
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                    ),
                ]),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(12.0),
                    left: Val::Px(12.0),
                    ..default()
                },
                ..default()
            },
            HudText,
        ))
        .id();
    hud_state.entity = Some(hud_entity);
    hud_state.message = "Press P to capture screenshot".into();
    hud_state.dirty = true;

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

    let mut transform = match query.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let delta = time.delta_seconds();
    let rotation_speed = std::f32::consts::PI; // half turn per second
    let mut yaw_input: f32 = 0.0;
    if keys.pressed(KeyCode::KeyQ) {
        yaw_input += 1.0;
    }
    if keys.pressed(KeyCode::KeyE) {
        yaw_input -= 1.0;
    }
    if yaw_input.abs() > f32::EPSILON {
        transform.rotate_y(yaw_input * rotation_speed * delta);
    }

    let mut movement = Vec3::ZERO;
    let mut forward = transform.forward().as_vec3();
    forward.y = 0.0;
    if forward.length_squared() > f32::EPSILON {
        forward = forward.normalize();
    }
    let mut right = transform.right().as_vec3();
    right.y = 0.0;
    if right.length_squared() > f32::EPSILON {
        right = right.normalize();
    }

    // Forward/back movement with W/S
    if keys.pressed(KeyCode::KeyW) {
        movement += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        movement -= forward;
    }

    // Lateral movement with A/D (strafe left/right)
    if keys.pressed(KeyCode::KeyA) {
        movement -= right;
    }
    if keys.pressed(KeyCode::KeyD) {
        movement += right;
    }
    if keys.pressed(KeyCode::KeyX) {
        movement.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyZ) {
        movement.y -= 1.0;
    }

    if movement != Vec3::ZERO {
        let speed = 50.0;
        transform.translation += movement.normalize() * speed * delta;
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

fn cycle_cameras(
    keys: Res<ButtonInput<KeyCode>>,
    mut registry: ResMut<CameraRegistry>,
    mut cameras: Query<&mut Camera>,
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    }
    let count = registry.cameras.len();
    if count < 2 {
        return;
    }

    let current = registry.active.min(count - 1);
    if let Ok(mut camera) = cameras.get_mut(registry.cameras[current]) {
        camera.is_active = false;
    }

    registry.active = (current + 1) % count;

    if let Ok(mut camera) = cameras.get_mut(registry.cameras[registry.active]) {
        camera.is_active = true;
    }
}

fn screenshot_capture(
    keys: Res<ButtonInput<KeyCode>>,
    registry: Res<CameraRegistry>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut hud_state: ResMut<HudState>,
) {
    if !keys.just_pressed(KeyCode::KeyP) {
        return;
    }
    if registry.cameras.is_empty() {
        hud_state.message = "Screenshot failed: no registered cameras".into();
        hud_state.dirty = true;
        return;
    }
    if let Err(err) = std::fs::create_dir_all("screenshots") {
        hud_state.message = format!("Screenshot failed: {err}");
        hud_state.dirty = true;
        return;
    }

    let active_index = registry
        .active
        .min(registry.cameras.len().saturating_sub(1));
    let camera_entity = registry.cameras[active_index];

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let filename = format!(
        "screenshots/shot-{}-{:03}.png",
        now.as_secs(),
        now.subsec_millis()
    );

    match screenshot_manager.save_screenshot_to_disk(camera_entity, filename.clone()) {
        Ok(()) => {
            hud_state.message = format!("Saved screenshot: {filename}");
            hud_state.dirty = true;
            info!("Saved screenshot to {filename}");
        }
        Err(err) => {
            hud_state.message = format!("Screenshot failed: {err}");
            hud_state.dirty = true;
            warn!("Failed to capture screenshot: {err}");
        }
    }
}

fn vehicle_controls(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut vehicles: Query<&mut Transform, With<GroundVehicle>>,
) {
    let mut movement_input = 0.0f32;
    if keys.pressed(KeyCode::KeyK) {
        movement_input += 1.0;
    }
    if keys.pressed(KeyCode::KeyJ) {
        movement_input -= 1.0;
    }
    let level_request = keys.just_pressed(KeyCode::Period);
    if movement_input.abs() < f32::EPSILON && !level_request {
        return;
    }

    let delta = time.delta_seconds();
    let speed = 25.0;

    for mut transform in &mut vehicles {
        let mut forward = transform.forward().as_vec3();
        forward.y = 0.0;
        if forward.length_squared() > f32::EPSILON {
            forward = forward.normalize();
        } else {
            forward = Vec3::Z;
        }

        if movement_input.abs() > f32::EPSILON {
            transform.translation += forward * (movement_input * speed * delta);
        }

        let ground_height = height_at(transform.translation.x, transform.translation.z);
        transform.translation.y = ground_height + 1.2;

        if level_request {
            let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
            transform.rotation = Quat::from_rotation_y(yaw);
        }
    }
}

fn update_hud(mut hud_state: ResMut<HudState>, mut texts: Query<&mut Text, With<HudText>>) {
    if !hud_state.dirty {
        return;
    }
    let Some(entity) = hud_state.entity else {
        return;
    };
    if let Ok(mut text) = texts.get_mut(entity) {
        if text.sections.len() >= 2 {
            text.sections[0].value = format!("{PROJECT_NAME}\n");
            text.sections[1].value = hud_state.message.clone();
        }
    }
    hud_state.dirty = false;
}
