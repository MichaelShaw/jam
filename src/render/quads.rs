extern crate cgmath;

use {Vec3};
use color::*;
use cgmath::{Rad, Matrix3, Point3, Transform};
use super::vertex::Vertex;
use render::texture_region::TextureRegion;

pub fn add_quad<T>(vertices: &mut Vec<T>, ts:[T; 4]) where T : Copy {
    for ele in ts.iter() {
        vertices.push(*ele);
    }
    vertices.push(ts[0]);
    vertices.push(ts[2]);
}

pub struct GeometryTesselator {
    pub scale: Vec3, // scale ... translating pixel coord to real world coords
    pub color: ColorFloatRaw,
}

const X_POS : [f32; 3] = [1.0, 0.0, 0.0];
const Y_POS : [f32; 3] = [0.0, 1.0, 0.0];
const Z_POS : [f32; 3] = [0.0, 0.0, 1.0];

const Z_NEG : [f32; 3] = [0.0, 0.0, -1.0];
 
impl GeometryTesselator {
    pub fn new(scale:Vec3) -> GeometryTesselator {
        GeometryTesselator {
            scale : scale,
            color : WHITE.float_raw(),
        }
    }

    pub fn draw_floor_tile_at(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, v:Vec3, depth_adjust:f64, flip:bool) {
        self.draw_floor_tile(vertices, tr, layer, v.x, v.y, v.z, depth_adjust, flip)
    }

    // anchor is near x/z coord
    pub fn draw_floor_tile(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, ax:f64, y:f64, az:f64, depth_adjust:f64, flip:bool) {
        let layer_f = layer as f32;
        let ww = (tr.width() as f64) * self.scale.x;
        let dw = (tr.height() as f64) * self.scale.y;

        let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
        let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

        add_quad(vertices, [
            Vertex { position: [ax as f32,        (y + depth_adjust) as f32, (az + dw + depth_adjust) as f32], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(ax + ww) as f32, (y + depth_adjust) as f32, (az + dw + depth_adjust) as f32], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(ax + ww) as f32, (y + depth_adjust) as f32, (az + depth_adjust     ) as f32], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [ax as f32,        (y + depth_adjust) as f32, (az + depth_adjust     ) as f32], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color, normal: Y_POS }
        ]);
    }


    // anchor is near x/z coord
    pub fn draw_wall_tile(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, ax:f64, ay:f64, z:f64, depth_adjust:f64, flip:bool) {
        let layer_f = layer as f32;
        let ww = (tr.width() as f64) * self.scale.x;
        let hw = (tr.height() as f64) * self.scale.z;

        let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
        let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

        add_quad(vertices, [
            Vertex { position: [ax as f32,        (ay + depth_adjust) as f32,      (z + depth_adjust) as f32], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(ax + ww) as f32, (ay + depth_adjust) as f32,      (z + depth_adjust) as f32], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(ax + ww) as f32, (ay + depth_adjust + hw) as f32, (z + depth_adjust) as f32], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [ax as f32,        (ay + depth_adjust + hw) as f32, (z + depth_adjust) as f32], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color, normal: Y_POS }
        ]);
    }

    // anchor is centre of tile
    pub fn draw_floor_centre_anchored_at(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, v:Vec3, depth_adjust:f64, flip:bool)  {
        self.draw_floor_centre_anchored(vertices, tr, layer, v.x, v.y, v.z, depth_adjust, flip)
    }

    pub fn draw_floor_centre_anchored(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, ax:f64, y:f64, az:f64, depth_adjust:f64, flip:bool) {
        let layer_f = layer as f32;

        let hww = (tr.width() as f64) * self.scale.x / 2.0;
        let hdw = (tr.height() as f64) * self.scale.z / 2.0;

        let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
        let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };


        add_quad(vertices, [
            Vertex { position: [(ax - hww) as f32, (y + depth_adjust) as f32, (az + hdw + depth_adjust) as f32], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(ax + hww) as f32, (y + depth_adjust) as f32, (az + hdw + depth_adjust) as f32], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(ax + hww) as f32, (y + depth_adjust) as f32, (az - hdw + depth_adjust) as f32], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(ax - hww) as f32, (y + depth_adjust) as f32, (az - hdw + depth_adjust) as f32], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color, normal: Y_POS }
        ]);
    }

    pub fn draw_floor_centre_anchored_rotated_at(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, v:Vec3, theta:f64, depth_adjust:f64)  {
        self.draw_floor_centre_anchored_rotated(vertices, tr, layer, v.x, v.y, v.z, theta, depth_adjust)
    }

    pub fn draw_floor_centre_anchored_rotated(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, ax:f64, y:f64, az:f64, theta:f64, depth_adjust:f64) {
        let layer_f = layer as f32;

        let hww = (tr.width() as f64) * self.scale.x / 2.0;
        let hdw = (tr.height() as f64) * self.scale.z / 2.0;

        let rot : Matrix3<f64> = Matrix3::from_angle_y(Rad(-theta));

        let p0 = rot.transform_point(Point3::new(- hww, 0.0, hdw));
        let p1 = rot.transform_point(Point3::new(hww,   0.0, hdw));
        let p2 = rot.transform_point(Point3::new(hww,   0.0, - hdw));
        let p3 = rot.transform_point(Point3::new(- hww, 0.0, - hdw));

        let xx = ax;
        let yy = y + depth_adjust;
        let zz = az + depth_adjust;

        add_quad(vertices, [
            Vertex { position: [(p0.x + xx) as f32, (p0.y + yy + depth_adjust) as f32, (p0.z + zz + depth_adjust) as f32], tex_coord: [tr.nu_min(), tr.nv_min(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(p1.x + xx) as f32, (p1.y + yy + depth_adjust) as f32, (p1.z + zz + depth_adjust) as f32], tex_coord: [tr.nu_max(), tr.nv_min(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(p2.x + xx) as f32, (p2.y + yy + depth_adjust) as f32, (p2.z + zz + depth_adjust) as f32], tex_coord: [tr.nu_max(), tr.nv_max(), layer_f], color: self.color, normal: Y_POS },
            Vertex { position: [(p3.x + xx) as f32, (p3.y + yy + depth_adjust) as f32, (p3.z + zz + depth_adjust) as f32], tex_coord: [tr.nu_min(), tr.nv_max(), layer_f], color: self.color, normal: Y_POS }
        ]);
    }

    pub fn draw_wall_base_anchored_at(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, v:Vec3, depth_adjust:f64, flip:bool) {
        self.draw_wall_base_anchored(vertices, tr, layer, v.x, v.y, v.z, depth_adjust, flip)
    }

    pub fn draw_wall_base_anchored(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, ax:f64, ay:f64, z:f64, depth_adjust:f64, flip:bool) {
        let layer_f = layer as f32;

        let hww = (tr.width() as f64) * self.scale.x / 2.0;
        let hhw = (tr.height() as f64) * self.scale.y;

        let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
        let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

        add_quad(vertices, [
            Vertex { position: [(ax - hww) as f32, (ay + depth_adjust) as f32,       (z + depth_adjust) as f32], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(ax + hww) as f32, (ay + depth_adjust) as f32,       (z + depth_adjust) as f32], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(ax + hww) as f32, (ay + hhw + depth_adjust) as f32, (z + depth_adjust) as f32], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(ax - hww) as f32, (ay + hhw + depth_adjust) as f32, (z + depth_adjust) as f32], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color, normal: Z_POS }
        ]);
    }

    pub fn draw_wall_centre_anchored_at(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, v:Vec3, depth_adjust:f64, flip:bool) {
        self.draw_wall_centre_anchored(vertices, tr, layer, v.x, v.y, v.z, depth_adjust, flip)
    }

    pub fn draw_wall_centre_anchored(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer:u32, ax:f64, ay:f64, z:f64, depth_adjust:f64, flip:bool) {
        let layer_f = layer as f32;

        let hww = (tr.width() as f64) * self.scale.x / 2.0;
        let hhw = (tr.height() as f64) * self.scale.y / 2.0;

        let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
        let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

        add_quad(vertices, [
            Vertex { position: [(ax - hww) as f32, (ay - hhw + depth_adjust) as f32, (z + depth_adjust) as f32], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(ax + hww) as f32, (ay - hhw + depth_adjust) as f32, (z + depth_adjust) as f32], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(ax + hww) as f32, (ay + hhw + depth_adjust) as f32, (z + depth_adjust) as f32], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(ax - hww) as f32, (ay + hhw + depth_adjust) as f32, (z + depth_adjust) as f32], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color, normal: Z_POS }
        ]);
    }

    pub fn draw_ui(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer: u32, x:f64, y:f64, z:f64, flip:bool, scale: f64) {
        let layer_f = layer as f32;
        let ww = (tr.width() as f64) * scale;
        let hw = (tr.height() as f64) * scale;

        let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
        let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

        add_quad(vertices, [
            Vertex { position: [x as f32,        (y) as f32,      z as f32], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(x + ww) as f32, (y) as f32,      z as f32], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(x + ww) as f32, (y + hw) as f32, z as f32], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [x as f32,        (y + hw) as f32, z as f32], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color, normal: Z_POS }
        ]);
    }

    pub fn draw_ui_centered(&self, vertices: &mut Vec<Vertex>, tr:&TextureRegion, layer: u32, x:f64, y:f64, z:f64, flip:bool, scale: f64) {
        let layer_f = layer as f32;
        let hww = (tr.width() as f64) * scale;
        let hhw = (tr.height() as f64) * scale;

        let nu_left = if flip { tr.nu_max() } else { tr.nu_min() };
        let nu_right = if flip { tr.nu_min() } else { tr.nu_max() };

        add_quad(vertices, [
            Vertex { position: [(x - hww) as f32, (y - hhw) as f32, z as f32], tex_coord: [nu_left , tr.nv_min(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(x + hww) as f32, (y - hhw) as f32, z as f32], tex_coord: [nu_right, tr.nv_min(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(x + hww) as f32, (y + hhw) as f32, z as f32], tex_coord: [nu_right, tr.nv_max(), layer_f], color: self.color, normal: Z_POS },
            Vertex { position: [(x - hww) as f32, (y + hhw) as f32, z as f32], tex_coord: [nu_left , tr.nv_max(), layer_f], color: self.color, normal: Z_POS }
        ]);
    }
}
