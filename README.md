# jam
Opinionated Game Jam Rendering Engine in Rust

### Main Example

cargo run --example commands

### Bitmap Font Example

cargon run --example text

# Todo
- Better point/pixel handling for tesselators/camera? Hrm
- Re-ified handling of texture array layers, font/texture enum that's smarter
- Include convenience App<State> type, with string VBO keys. With sleeping game loop for 60fps?
- Shorthand for building texture atlas/regions. Keep animation etc. in mind


# Todo - Low Importance
- Cleanup vertex naming/namespacing etc.
- Put color on a diet, [u8; 4]? I like being able to brighten though .... :-/ hrm
- Should translucents write to depth? Probably, it'd overwrite people just stamping over them? Hrm. Unsure.


