# Test Plan

## Scope
Manual and automated checks for the dexter-platz prototype covering terrain generation, camera control, and vehicle behavior.

## Environments
- Linux desktop with GPU support (X11 or Wayland via XWayland)
- Rust stable toolchain (1.80+)
- `cargo run`, `cargo test`, and optional `cargo clippy`

## Test Matrix
| Feature | Scenario | Expected Result |
|---------|----------|-----------------|
| Build   | `cargo check` | Completes without warnings or errors |
| Build   | `cargo build --release` | Produces optimized binary |
| Runtime | `cargo run` on X11 | Window appears, terrain renders, controls respond |
| Runtime | `cargo run --features wayland` | Runs when Wayland libs installed |
| Cameras | `Tab` toggling | Cycles fly camera ↔ ground vehicle camera |
| Fly camera | `W/S/A/D/Z/X` | Moves and rotates correctly |
| Vehicle | `K/J` drive | Vehicle moves along ground, snaps to terrain |
| Vehicle | `.` level | Vehicle yaw preserved, pitch/roll reset |
| Textures | Inspect terrain | Ground, dirt, stone textures tile correctly |

## Regression Checklist
- [ ] Run `cargo fmt`, ensure no diff
- [ ] Run `cargo clippy -- -D warnings`
- [ ] Run `cargo test`
- [ ] Run `cargo run` and verify controls
- [ ] Switch cameras with `Tab`
- [ ] Drive vehicle with `K/J`, level with `.`
- [ ] Observe lighting animation and center cube

## Future Automation Ideas
- Headless integration test using `bevy_test` harness once available
- Visual regression via screenshot comparison (e.g., wgpu capture)
- CI workflow with fmt/clippy/test matrix
