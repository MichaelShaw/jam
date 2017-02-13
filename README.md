# jam
Opinionated Game Jam Rendering Engine in Rust

Only example is at:

examples/commands/main.rs

cargo run --example commands

# Todo
- Is Initial dimensions pixels or points .... we probably need seperate structs to avoid mistakes .... and a scale factor ... to allow translation.
- Cleanup vertex naming/namespacing etc.
- Work out how points vs pixels work? Should it be a zoom option, or a different fundamental pixels/points per unit thing. So units remain the same (sounds sane)

- Alpha blending (look at gfx-rs particle example, requires new pipeline) ... we 

- UI rendering ... just clear depth before?

- Add text rendering  + font loading.

- Make launch application return a result and try! (we'd have to translate errors)
- Put color on a diet, [u8; 4]? I like being able to brighten though .... :-/ hrm

- Shorthand for building texture atlas/regions.

