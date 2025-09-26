# platz

A minimal Bevy voxel mini-game scaffold.

## Run

Ensure Rust and Cargo are installed, then:

```
cargo run
```
This launches a Bevy window rendering a 9×9×9 chunk world
with a Perlin-textured heightfield terrain, a center marker cube, and basic animated lighting.

Wayland sessions can opt into the native backend (requires system Wayland development packages and matching runtime libraries) with:

```
cargo run --features wayland
```
The default build targets X11 and will fall back to XWayland when run in a Wayland compositor.

## Controls

- `Backspace`: quit
- `W`/`A`/`S`/`D`: move ground-plane directions
- `Z`: move down
- `X`: move up

The directional light gradually shifts its color between blue, white, and red to give the scene some motion.

## Build

```
cargo build --release
```

## Next steps
- Decide if this should be a binary, a library, or a workspace.
- Add dependencies in `Cargo.toml`.
- Add CI (fmt, clippy, tests) if needed.
- Choose a license.
