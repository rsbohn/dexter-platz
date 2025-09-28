## Context
Investigated why `cargo run` kept rebuilding `alsa-sys` and failing on missing system ALSA libs.

## Actions taken
- Disabled Bevy's default audio feature in `Cargo.toml` while keeping the rendering stack and adding opt-in `audio`/`wayland`/`x11` toggles.
- Updated `src/lib.rs` for Bevy 0.14 API changes (new mesh constructor, `Color::srgb`) without referencing audio-specific types.
- Removed an unused import in `src/meshing.rs` to clear warnings.
- Documented how to launch with native Wayland support in `README.md`.
- Added WASD/ZX camera controls and Backspace-to-quit handling.
- Animated the directional light to drift between blue, white, and red and documented the behavior.
- Spawned a white cube at the world's center to serve as a visual reference point.
- Enabled Bevy's `tonemapping_luts` feature so TonyMcMapFace tonemapping works without runtime warnings.
- Swapped the flat ground plane for a trigonometric heightfield when populating chunks.
- Raised the center cube so it floats 10 voxels above the sampled terrain height.
- Generated a Perlin-based ground texture and applied it via a shared material.
- Added dirt and stone texture variants and pick chunk materials based on sampled elevation.
- Reworked the fly camera: A/D now yaw, W/S moves relative to facing, rig now has a visible mesh child plus the camera.
- Spawned a ground vehicle rig with its own camera and wired Tab to cycle active viewpoints.
- Hooked up K/J keys to drive the ground vehicle forward/back independent of the active camera.
- Renamed the README heading to match the `dexter-platz` repo and refreshed the next-steps list.
- Added screenshot capture on `P` using Bevy's `ScreenshotPlugin`, dumping files into `screenshots/` and ignoring them in git.
- Removed the redundant `ScreenshotPlugin` registration after runtime panic; rely on the default Bevy stack instead.
- Enabled Bevy UI/text features and added a HUD that shows the project name plus the most recent screenshot message.
- Loaded Bevy's embedded font explicitly, spawned a dedicated UI camera, and log HUD updates so the overlay renders reliably and screenshots confirm success.
- Built a simple U-shaped ruin with pillars and debris near the world center to give the scene some points of interest.
- Added an emissive stone core inside the ruin so the new spotlight has something to highlight.
- Built a glowing fountain near the ruin with animated spray and a cyan point light.
- Added a rotating spotlight rig atop the ground vehicle to sweep the terrain while it moves.
- Verified the scene visually: base renders as dark green, the center cube responds to the animated lighting sweep.
- Verified the project builds with `cargo check` (runtime still needs X11 cursor libs).

## Decisions
- Rely on feature flags instead of runtime plugin disable to avoid pulling in platform audio dependencies.

## Next steps
- Install `libXcursor` (or similar) if window creation is required on this machine.
- Enable the `audio` feature explicitly if audio support is needed in the future.
