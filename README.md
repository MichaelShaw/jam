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
- Sound? I hate ears :-( Alto (openal) + Lewton (ogg)?
- Better hotloading textures, destruction and recreation is fine. No need to worry about caching.
- Remove extra image allocation when buffering textures?
- Put color on a diet
- Add text rendering from ggez.rs
- We should probably have a locals struct, simple so we can re-use the struct between pipelines (blend/non-blending versions)

- Investigate gfx-rs Bundle struct: slice, pso, data