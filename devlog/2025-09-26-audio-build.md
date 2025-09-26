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
- Verified the scene visually: base renders as dark green, the center cube responds to the animated lighting sweep.
- Verified the project builds with `cargo check` (runtime still needs X11 cursor libs).

## Decisions
- Rely on feature flags instead of runtime plugin disable to avoid pulling in platform audio dependencies.

## Next steps
- Install `libXcursor` (or similar) if window creation is required on this machine.
- Enable the `audio` feature explicitly if audio support is needed in the future.
