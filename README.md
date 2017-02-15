# jam
Opinionated Game Jam Rendering Engine in Rust

### Main Example

cargo run --example commands

### Bitmap Font Example

cargon run --example text

# Todo
- Case insensitive extension matching for texture arrays.
- Fix up our coordinate systems to be consistent ... texture UV+ with geometry xyz+ if possible for most obvious case ...
- UI rendering pass ... just clear depth before?
- Better point/pixel handling for tesselators/camera
- Before generating a font, check we have space, return a nice error message. e.g. the next power of 2 texture that has sufficient size to store the bitmap font texture (Nicer error message)
- Include convenience App<State> type, with string VBO keys
- More re-ified handling of texture array layers, font/texture enum that's smarter

# Todo - Low Importance
- Cleanup vertex naming/namespacing etc.
- Put color on a diet, [u8; 4]? I like being able to brighten though .... :-/ hrm
- Shorthand for building texture atlas/regions. Keep animation etc. in mind

