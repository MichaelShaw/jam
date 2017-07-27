use image::Rgba;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Source {
    ConstantColour(ConstantColour),
}

pub trait ColourSource {
    fn get(&self, x:i32, y:i32) -> Rgba<u8>;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ConstantColour {
    pub color: Rgba<u8>,
}

impl ColourSource for ConstantColour {
    fn get(&self, x:i32, y:i32) -> Rgba<u8> {
        self.color
    }
}

