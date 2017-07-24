
pub mod renderer;
pub mod init;

pub use self::renderer::*;
pub use self::init::*;

use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        position: [f32; 3] = "position",
        tex_coord: [f32; 3] = "tex_coord",
        color: [f32; 4] = "color",
        normal: [f32; 3] = "normal",
    }

    constant Locals {
        u_transform: [[f32; 4]; 4] = "u_transform",
        u_color: [f32; 4] = "u_color",
        u_alpha_minimum: f32 = "u_alpha_minimum",
    }

    pipeline pipe_opaque {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "u_texture",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }

    pipeline pipe_blend {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "u_texture",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out_color: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}


#[derive(Debug)]
pub struct GeometryBuffer<R> where R : gfx::Resources {
    pub buffer: gfx::handle::Buffer<R, Vertex>,
    pub slice : gfx::Slice<R>,
}
