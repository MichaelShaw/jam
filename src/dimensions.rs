#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dimensions {
    pub pixels: (u32,u32),
    pub scale: f64,
}

impl Dimensions {
    pub fn points(&self) -> (f64, f64) {
    	let (width, height) = self.pixels;
        (width as f64 / self.scale, height as f64 / self.scale)
    }

    pub fn approx_equal_point_size(lhs: Dimensions, rhs:Dimensions) -> bool {
    	let (l_width, l_height) = lhs.points();
    	let (r_width, r_height) = rhs.points();
    	(l_width - r_width).abs() < 0.01 && (l_height - r_height).abs() < 0.01
    }
}
