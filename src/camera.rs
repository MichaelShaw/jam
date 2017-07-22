extern crate cgmath;

use {Mat4, Vec3, Vec4};
use cgmath::*;
use geometry;
use dimensions::Dimensions;

use cgmath::SquareMatrix;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Camera {
    pub at: Vec3,
    pub pitch: Rad<f64>,
    pub viewport: Dimensions,
    pub points_per_unit: f64,
}

impl Camera {
    // UI
    pub fn ui_projection(&self) -> Mat4 {
        let (points_wide, points_high) = self.viewport.points;
        ui_projection(points_wide as f64, points_high as f64)
    }

    pub fn ui_inverse_projection(&self) -> Option<Mat4> {
        self.ui_projection().invert()
    }

    // THE WORLD
    pub fn view(&self) -> Mat4 {
        view(self.pitch, self.at)
    }

    pub fn projection(&self) -> Mat4 {
        let (points_wide, points_high) = self.viewport.points;
        projection(points_wide as f64, points_high as f64, self.points_per_unit)
    }

    pub fn units_per_point(&self) -> f64 {
        1.0 / self.points_per_unit
    }

    pub fn view_projection(&self) -> Mat4 {
        self.projection() * self.view()
    }

    pub fn inverse_view_projection(&self) -> Option<Mat4> {
        self.view_projection().invert()
    }

    pub fn world_line_segment_for_mouse_position(&self, x:i32, y:i32) -> Option<geometry::LineSegment> {
        let (width, height) = self.viewport.pixels;
        self.inverse_view_projection().and_then(|ivp| {
            ray_for_mouse_position(ivp, width, height, x, y)
        })
    }

    pub fn ui_line_segment_for_mouse_position(&self, x:i32, y:i32) -> Option<(f64, f64)> {
        let (pixels_wide, pixels_high) = self.viewport.pixels;
        if x > 0 && y > 0 && x < pixels_wide as i32 && y < pixels_high as i32 {
            let point_x = x as f64 / self.viewport.scale();
            let point_y = (pixels_high as i32 - y) as f64 / self.viewport.scale();
            Some((point_x, point_y))
        } else {
            None
        }
    }
}

pub fn view(pitch: Rad<f64>, at: Vec3) -> Mat4 {
    Mat4::from_angle_x(pitch) * Mat4::from_translation(at * -1.0)
}

pub fn ui_projection(width: f64, height: f64) -> Mat4 {
    // left, right, bottom, top, near, far
    cgmath::ortho(0.0, width, 0.0, height, -100.0, 100.0) // having trouble with this z stuff
}

pub fn projection(width:f64, height:f64, pixels_per_unit: f64) -> Mat4 {
    let effective_width = width  / pixels_per_unit;
    let effective_height = height / pixels_per_unit / (2.0_f64).sqrt(); // adjust for 45 degree downward viewing angle
    let half_width = effective_width / 2.0;
    let half_height = effective_height / 2.0;

    cgmath::ortho(-half_width, half_width, -half_height, half_height, -100.0, 100.0)
}

pub fn ray_for_mouse_position(inverse_view_projection:Mat4, width:u32, height:u32, x:i32, y:i32) -> Option<geometry::LineSegment> {
    if 0 <= x && x < (width as i32) && 0 <= y && y < (height as i32) {
        let n_x = (x as f64) / (width as f64) * 2.0 - 1.0;
        let n_y = ((y as f64) / (height as f64) * 2.0 - 1.0) * -1.0;

        let front = Vec4::new(n_x, n_y, -1.0, 1.0);
        let back = Vec4::new(n_x, n_y, 1.0, 1.0);

        let front_world = inverse_view_projection * front;
        let back_world = inverse_view_projection * back;

        Some(geometry::LineSegment {
            from: front_world.truncate() / front_world.w,
            to: back_world.truncate() / back_world.w,
        })
    } else {
        None
    }
} 
