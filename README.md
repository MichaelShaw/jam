# jam
Opinionated Game Jam Rendering Engine in Rust

Only example is at:

examples/commands/main.rs

cargo run --example commands

# Todo
- Make launch application return a result and try! (we'd have to translate errors)
- Fix Texture/render directions (decision time)
- We should probably have a locals struct, simple so we can re-use the struct between pipelines (blend/non-blending versions)
- Alpha blending (look at gfx-rs particle example, requires new pipeline)
- Work out better placeholder geometry (so we can cut the Tesselator creation in fatter.rs)
- Put color on a diet
- Add text rendering from ggez.rs

