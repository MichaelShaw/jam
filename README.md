# jam
Opinionated Game Jam Rendering Engine in Rust

### Main Example

cargo run --example commands

### Bitmap Font Example

cargon run --example text

# Todo
- Allow prevention of double work. Allowed to specify texture descriptions & font descriptions before the initial load.
- Convenience functions for texture regions.
- Consider making texture regions specify layer.
- Semi convenient unloading of unneeded font textures?
- Include convenience App<State> type, with string VBO keys. With sleeping game loop for 60fps? Include time measurement?
 
# Todo - Low Importance
- Put color on a diet, [u8; 4]? I like being able to brighten though .... :-/ hrm. Floats being able to 1.5 stuff is useful.
- Should translucents write to depth? Probably, it'd overwrite people just stamping over them? Hrm. Unsure.


