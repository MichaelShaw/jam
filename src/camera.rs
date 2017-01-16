extern crate cgmath;

use {Mat4, Vec3, Vec4};
use cgmath::*;
use geometry;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Camera {
    pub at: Vec3,
    pub pitch: Rad<f64>,
    pub viewport: (u32, u32),
    pub pixels_per_unit: f64,
}

impl Camera {
    pub fn view(&self) -> Mat4 {
        view(self.pitch, self.at)
    }

    pub fn projection(&self) -> Mat4 {
        let (width, height) = self.viewport;
        projection(width, height, self.pixels_per_unit) 
    }

    pub fn units_per_pixel(&self) -> f64 {
        1.0 / self.pixels_per_unit
    }

    pub fn view_projection(&self) -> Mat4 {
        self.projection() * self.view()
    }

    pub fn inverse_view_projection(&self) -> Option<Mat4> {
        use cgmath::SquareMatrix;
        let vp = self.view_projection();
        vp.invert()
    }

    pub fn ray_for_mouse_position(&self, x:i32, y:i32) -> Option<geometry::Line> {
        let (width, height) = self.viewport;
        self.inverse_view_projection().and_then(|ivp| {
            ray_for_mouse_position(ivp, width, height, x, y)
        })
    }
}

pub fn view(pitch: Rad<f64>, at: Vec3) -> Mat4 {
    Mat4::from_angle_x(pitch) * Mat4::from_translation(at * -1.0)
}

pub fn projection(width:u32, height:u32, pixels_per_unit: f64) -> Mat4 {
    let effective_width = (width as f64) / (pixels_per_unit);
    let effective_height = (height as f64) / (pixels_per_unit) / (2.0_f64).sqrt(); // adjust for 45 degree downward viewing angle
    let half_width = effective_width / 2.0;
    let half_height = effective_height / 2.0;

    cgmath::ortho(-half_width, half_width, -half_height, half_height, -100.0, 100.0)
}

pub fn interface(width:u32, height:u32) -> Mat4 {
    cgmath::ortho(0.0, width as f64, 0.0, height as f64, -100.0, 100.0)
}

// ortho<S: BaseFloat>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Matrix4<S> {

pub fn ray_for_mouse_position(inverse_view_projection:Mat4, width:u32, height:u32, x:i32, y:i32) -> Option<geometry::Line> {
    if 0 <= x && x < (width as i32) && 0 <= y && y < (height as i32) {
        let n_x = (x as f64) / (width as f64) * 2.0 - 1.0;
        let n_y = ((y as f64) / (height as f64) * 2.0 - 1.0) * -1.0;

        let front = Vec4::new(n_x, n_y, -1.0, 1.0);
        let back = Vec4::new(n_x, n_y, 1.0, 1.0);

        let front_world = inverse_view_projection * front;
        let back_world = inverse_view_projection * back;

        Some(geometry::Line {
            from: front_world.truncate() / front_world.w,
            to: back_world.truncate() / back_world.w,
        })
    } else {
        None
    }
} 
