# jam
Opinionated Game Jam Rendering Engine in Rust

Only example is at:

examples/commands/main.rs

cargo run --example commands

# Todo
- Alpha blending (look at gfx-rs particle example, requires new pipeline)
- Locals struct, simple so we can re-use the struct between pipelines (blend/non-blending versions)?
- Put color on a diet
- Make launch application return a result and try! (we'd have to translate errors)
- UI rendering (unsure if we need this .... It still needs depth, just needs to be close to camera, hrm)
- Add text rendering from ggez.rs

