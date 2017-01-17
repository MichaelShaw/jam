# jam
Opinionated Game Jam Engine in Rust

Only example is at:

examples/commands/main.rs

cargo run --example commands

# Todo
- Test our sound decoding through playing a hundred various oggs in sequence.
- Make launch application return a result and try! (we'd have to translate errors)
- Fix Texture/render directions
- We should probably have a locals struct, simple so we can re-use the struct between pipelines (blend/non-blending versions)
- Alpha blending (look at gfx-rs particle exapmle, requires new pipeline)
- Work out better placeholder geometry (so we can cut the Tesselator creation in fatter.rs)
- Alto (openal) + Lewton (ogg) for sound?
- Put color on a diet
- Add text rendering from ggez.rs

