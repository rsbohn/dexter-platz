# dexter-platz

A minimal Bevy voxel mini-game scaffold.

## Run

Ensure Rust and Cargo are installed, then:

```
cargo run
```
This launches a Bevy window rendering a 9×9×9 chunk world
with Perlin-textured heightfield terrain (grass, dirt, stone bands), a center marker cube, and basic animated lighting.

Wayland sessions can opt into the native backend (requires system Wayland development packages and matching runtime libraries) with:

```
cargo run --features wayland
```
The default build targets X11 and will fall back to XWayland when run in a Wayland compositor.

## Controls

- `Backspace`: quit
- `Q`/`E`: rotate left/right
- `W`/`S`: move forward/back based on facing
- `A`/`D`: strafe left/right
- `Z`: move down
- `X`: move up
- `Tab`: toggle between available cameras
- `K`/`J`: move the ground vehicle forward/back (works regardless of active camera)
- `P`: capture a screenshot to `screenshots/`

The directional light gradually shifts its color between blue, white, and red to give the scene some motion.

## Build

```
cargo build --release
```

## Next steps
- Add driving controls (steering, braking) and physics for the ground vehicle.
- Populate the world with interactable props to showcase multiple cameras.
- Split rendering and gameplay into modules if this grows beyond a single binary.
- Wire up CI (fmt, clippy, tests) to keep the project healthy.
