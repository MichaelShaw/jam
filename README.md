# jam
Opinionated Game Jam Rendering Engine in Rust

Only example is at:

examples/commands/main.rs

cargo run --example commands

# Todo
- Cleanup vertex naming

- Alpha blending (look at gfx-rs particle example, requires new pipeline) ... we 

- UI rendering ... just clear depth before?

- Add text rendering from ggez.rs

- Make launch application return a result and try! (we'd have to translate errors)
- Put color on a diet, [u8; 4]? I like being able to brighten though .... :-/ hrm

- Eventually, invert control of our renderer. Make it a thing you can prod ... rather than prods you?

