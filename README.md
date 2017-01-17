# jam
Opinionated Game Jam Engine in Rust

Only example is at:

examples/commands/main.rs

cargo run --example commands

# Todo
- Make launch application return a result and try! (we'd have to translate errors)
- Add notion of time, call "render" with a time delta.
- Fix Texture/render directions
- Alpha blending (look at gfx-rs particle exapmle, requires new pipeline)
- Work out better placeholder geometry (so we can cut the Tesselator creation in fatter.rs)
- Alto (openal) + Lewton (ogg) for sound?
- Put color on a diet
- Add text rendering from ggez.rs
- We should probably have a locals struct, simple so we can re-use the struct between pipelines (blend/non-blending versions)
