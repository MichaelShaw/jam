
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dimensions {
    pub pixels: (u32,u32),
    pub scale: f32,
}

impl Dimensions {
    pub fn points(&self) -> (f32, f32) {
    	let (width, height) = self.pixels;
        (width as f32 / self.scale, height as f32 / self.scale)
    }

    pub fn approx_equal_point_size(lhs: Dimensions, rhs:Dimensions) -> bool {
    	let (l_width, l_height) = lhs.points();
    	let (r_width, r_height) = rhs.points();
    	(l_width - r_width).abs() < 0.01 && (l_height - r_height).abs() < 0.01
    }
}
