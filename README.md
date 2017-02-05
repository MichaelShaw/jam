# jam
Opinionated Game Jam Rendering Engine in Rust

Only example is at:

examples/commands/main.rs

cargo run --example commands

# Todo
- Fix up texture rendering to be nearest neighbour
- Cleanup vertex naming/namespacing etc.
- Is Initial dimensions pixels or points .... we probably need seperate structs to avoid mistakes .... and a scale factor ... to allow translation.

- Alpha blending (look at gfx-rs particle example, requires new pipeline) ... we 

- UI rendering ... just clear depth before?

- Add text rendering  + font loading.

- Make launch application return a result and try! (we'd have to translate errors)
- Put color on a diet, [u8; 4]? I like being able to brighten though .... :-/ hrm

- Invert control of our renderer. Make it a thing you can call, rather than something that calls you.
- Shorthand for building texture atlas/regions.

