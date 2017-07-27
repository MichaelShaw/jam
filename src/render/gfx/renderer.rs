use gfx;
use glutin;
use gfx_device_gl;

use gfx::format::{Srgba8, Rgba8};
use gfx::Device;
use gfx::traits::FactoryExt;
use gfx_window_glutin;

use gfx::texture::ImageInfoCommon;
use gfx::format::R8_G8_B8_A8;

use super::{Vertex, ColorFormat, DepthFormat, GeometryBuffer, Locals};
use super::{pipe_blend, pipe_opaque, get_dimensions};

use {input, JamError, JamResult, color, Color};
use render::{FileResources, FileWatcher, TextureArrayDimensions, Uniforms, Blend, TextureRegion, GeometryTesselator};
use FontDirectory;
use {Dimensions, InputState};
use glutin::GlContext;
use camera::ui_projection;
use render::down_size_m4;

use image::DynamicImage;
use notify::{RawEvent};
use std::sync::mpsc::{Receiver};

use aphid::HashMap;

use ui::*;

use cgmath::{Vector2, vec3};

use OurFont;

pub type OpenGLResources = gfx_device_gl::Resources;
pub type OpenGLRenderer = Renderer<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer, gfx_device_gl::Factory, gfx_device_gl::Device>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TextureArraySource {
    Primary,
    UI,
}

pub struct Renderer<R, C, F, D> where R : gfx::Resources,
                                 C : gfx::CommandBuffer<R>,
                                 F : gfx::Factory<R>,
                                 D : gfx::Device {
    pub file_resources: FileResources,
    pub file_watcher: FileWatcher,

    // next 2 are opengl specific
    pub window: glutin::GlWindow, // opengl
    pub events_loop: glutin::EventsLoop, // opengl

    pub device: D,
    pub factory: F,

    pub screen_colour_target: gfx::handle::RenderTargetView<R, ColorFormat>,
    pub screen_depth_target: gfx::handle::DepthStencilView<R, DepthFormat>,
    pub encoder: gfx::Encoder<R, C>,

    // what about raw texture representation? for blitting to ui
    pub texture: Option<(gfx::handle::Texture<R, gfx::format::R8_G8_B8_A8>, gfx::handle::ShaderResourceView<R, [f32; 4]>)>,

    pub sampler: gfx::handle::Sampler<R>,

    pub pipelines: Option<Pipelines<R>>,

    pub dimensions: Dimensions,
    pub input_state: InputState,

    pub ui: UI<R>,
}

pub struct UI<R> where R : gfx::Resources {
    pub dimensions: TextureArrayDimensions,
    pub texture_resource: gfx::handle::Texture<R, gfx::format::R8_G8_B8_A8>,
    pub texture_view: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pub elements: HashMap<ElementWithSize<i32>, RasterElement>,
    pub tick: usize,
    pub free_layers: Vec<u32>,
    pub fonts: Vec<OurFont>,
}

pub struct RasterElement {
    pub translation: Vector2<i32>, // translation from requested origin to output area
    pub texture_region: TextureRegion,
    pub last_used: usize,
    // include a cost metric for rasterization time?
}

pub struct Pipelines<R> where R : gfx::Resources {
    pub opaque: OpaquePipeline<R>,
    pub blend: BlendPipeline<R>,
}

pub struct OpaquePipeline<R> where R : gfx::Resources {
    pub pipeline: gfx::PipelineState<R, pipe_opaque::Meta>,
    pub data : Option<pipe_opaque::Data<R>>,
}

pub struct BlendPipeline<R> where R : gfx::Resources {
    pub pipeline: gfx::PipelineState<R, pipe_blend::Meta>,
    pub data : Option<pipe_blend::Data<R>>,
}

impl<F> Renderer<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer, F, gfx_device_gl::Device> where F : gfx::Factory<gfx_device_gl::Resources> {
    pub fn begin_frame(&mut self, clear_color: Color) -> (Dimensions, InputState) {
        self.load_resources();

        let mut events : Vec<glutin::Event> = Vec::new();

        self.events_loop.poll_events(|ev| events.push(ev));

        let mut close_requested = false;
        let mut resize = false;

        for ev in &events {
            if let &glutin::Event::WindowEvent { ref event, .. } = ev {
                match event {
                    &glutin::WindowEvent::KeyboardInput {
                        input: glutin::KeyboardInput {
                            virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => {
                        close_requested = true
                    },
                    &glutin::WindowEvent::Closed => {
                        close_requested = true
                    },
                    &glutin::WindowEvent::Resized(width, height) => {
                        self.window.resize(width, height);
                        resize = true;
                    },
                    _ => (),
                }
            }
        }

        if resize {
            println!("resize, PRE -> {:?}", get_dimensions(&self.window));
            gfx_window_glutin::update_views(&self.window, &mut self.screen_colour_target, &mut self.screen_depth_target);
            println!("POST -> {:?}", get_dimensions(&self.window));
        }

        self.input_state = input::produce(&self.input_state, &events);
        self.input_state.close = close_requested;

        let dimensions = get_dimensions(&self.window);
        self.dimensions = dimensions;

        // clear
        if !close_requested {
//            self.screen_colour_target = 4;
            self.encoder.clear(&self.screen_colour_target, clear_color.float_raw());
            self.encoder.clear_depth(&self.screen_depth_target, 1.0);
//            let ad = self.screen_colour_target.get_dimensions();
//            println!("internal dimensions -> {:?}", ad);
        }

        (dimensions, self.input_state.clone())
    }

    pub fn load_resources(&mut self) {
        let (reload_program, reload_texture) = check_reload(&self.file_watcher.change_events, &self.file_resources);

        if reload_program || self.pipelines.is_none() {
            println!("LOAD PIPELINES");
            let pipeline_load_result = self.file_resources.shader_pair.load().and_then( |shader_data| {
                let opaque_pso = self.factory.create_pipeline_simple(
                    &shader_data.vertex_data,
                    &shader_data.fragment_data,
                    pipe_opaque::new()
                ).map_err(JamError::PipelineError)?;

                let blend_pso = self.factory.create_pipeline_simple(
                    &shader_data.vertex_data,
                    &shader_data.fragment_data,
                    pipe_blend::new()
                ).map_err(JamError::PipelineError)?;

                Ok(Pipelines {
                    opaque: OpaquePipeline {
                        pipeline: opaque_pso,
                        data: None,
                    },
                    blend: BlendPipeline {
                        pipeline: blend_pso,
                        data: None,
                    },
                })
            });

            match pipeline_load_result {
                Ok(p) => self.pipelines = Some(p),
                Err(e) => println!("pipeline load error -> {:?}", e),
            }
        }

        if reload_texture || self.texture.is_none() {
            println!("LOAD TEXTURES");
            let texture_load_result = self.file_resources.texture_directory.load().and_then(|texture_array_data| {
                let images_raw : Vec<_> = texture_array_data.images.iter().map(|img| {
                    let dyn_image = DynamicImage::ImageRgba8(img.clone()).flipv();
                    dyn_image.to_rgba().into_raw()
//                    img.clone().into_raw()
                } ).collect();
                let data : Vec<_> = images_raw.iter().map(|v| v.as_slice()).collect();

                let kind = texture_kind_for(&texture_array_data.dimensions);

                println!("kind -> {:?}", kind);
//                let (texture, texture_view) = self.factory.create_texture_immutable_u8::<Rgba8>(kind, data.as_slice()).map_err(JamError::CombinedGFXError)?;
                let (texture, texture_view) = self.factory.create_texture_immutable_u8::<Srgba8>(kind, data.as_slice()).map_err(JamError::CombinedGFXError)?;

                Ok((texture, texture_view))
            });

            match texture_load_result {
                Ok((t, tv)) => {
                    let pair = (t, tv);
                    self.texture = Some(pair);
                },
                Err(e) => println!("texture load error -> {:?}", e),
            }
        }
    }

    pub fn clear_depth(&mut self) {
        self.encoder.clear_depth(&self.screen_depth_target, 1.0);
    }

    pub fn upload(&mut self, vertices: &[Vertex]) -> GeometryBuffer<gfx_device_gl::Resources> {
        let (buffer, slice) = self.factory.create_vertex_buffer_with_slice(vertices, ());
        GeometryBuffer {
            buffer,
            slice,
        }
    }

//    pub fn screenshot(&mut self) -> JamResult<()> { // -> image::DynamicImage
//        let (width, height, depth, _) = self.screen_colour_target.get_dimensions();
//        let pixels = (width as usize * height as usize * depth as usize);
//        println!("screen dimensions {:?} x {:?} x {:?} pixels -> {:?}", width, height, depth, pixels);

//        let (tex, srv, rtv) = self.factory.create_render_target(width, height)?;

//        let download_buffer = self.factory.create_download_buffer::<u8>(pixels * 4).map_err(JamError::CreationError)?;

        // this would appear to require a frame buffer now ... which is ... quite a cost just to take screenshots.


//        self.encoder.copy_texture_to_buffer_raw(src: &RawTexture<R>, None, info: texture::RawImageInfo,
//                                                dst: &handle::RawBuffer<R>, 0);
//        ,
//        face: Option<CubeFace>,
//        info: RawImageInfo,
//        dst: &RawBuffer<R>,
//        dst_offset_bytes: usize

//        let image: glium::texture::RawImage2d<u8> = self.display.read_front_buffer();
//        let image = image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned()).unwrap();
//        let image = image::DynamicImage::ImageRgba8(image).flipv();
//        image
//        Ok(())
//    }

    fn draw_raw(&mut self, geometry: &GeometryBuffer<gfx_device_gl::Resources>, uniforms: Uniforms, blend:Blend, texture_array:TextureArraySource)  -> JamResult<()> {
        let tv = match texture_array {
            TextureArraySource::UI => &self.ui.texture_view,
            TextureArraySource::Primary => self.texture.as_ref().map(|&(_, ref v)| v).ok_or(JamError::NoTexture())?,
        };

//        let tv = self.texture.as_ref().map(|&(_, ref v)| v).ok_or(JamError::NoTexture())?;

        match blend {
            Blend::None => {
                let opaque_pipe = self.pipelines.as_mut().ok_or(JamError::NoPipeline()).map(|p| &mut p.opaque )?;
                let opaque_data = pipe_opaque::Data {
                    vbuf: geometry.buffer.clone(),
                    texture: (tv.clone(), self.sampler.clone()),
                    locals: self.factory.create_constant_buffer(1),
                    out_color: self.screen_colour_target.clone(),
                    out_depth: self.screen_depth_target.clone(),
                };
                let locals = Locals {
                    u_transform: uniforms.transform,
                    u_color: uniforms.color.float_raw(),
                    u_alpha_minimum: 0.01,
                };
                self.encoder.update_constant_buffer(&opaque_data.locals, &locals);
                self.encoder.draw(&geometry.slice, &opaque_pipe.pipeline, &opaque_data);
            },
            Blend::Add => {
                //                println!("no add pipeline atm")
            },
            Blend::Alpha => {
                let blend_pipe = self.pipelines.as_mut().ok_or(JamError::NoPipeline()).map(|p| &mut p.blend )?;
                let blend_data = pipe_blend::Data {
                    vbuf: geometry.buffer.clone(),
                    texture: (tv.clone(), self.sampler.clone()),
                    locals: self.factory.create_constant_buffer(1),
                    out_color: self.screen_colour_target.clone(),
                    out_depth: self.screen_depth_target.clone(),
                };
                let locals = Locals {
                    u_transform: uniforms.transform,
                    u_color: uniforms.color.float_raw(),
                    u_alpha_minimum: 0.01,
                };
                self.encoder.update_constant_buffer(&blend_data.locals, &locals);
                self.encoder.draw(&geometry.slice, &blend_pipe.pipeline, &blend_data);
            },
        }

        Ok(())
    }

    pub fn draw(&mut self, geometry: &GeometryBuffer<gfx_device_gl::Resources>, uniforms: Uniforms, blend:Blend) -> JamResult<()> {
        self.draw_raw(geometry, uniforms, blend, TextureArraySource::Primary)
    }

    pub fn draw_vertices(&mut self, vertices: &[Vertex], uniforms: Uniforms, blend:Blend) -> JamResult<GeometryBuffer<gfx_device_gl::Resources>> {
        let geometry = self.upload(vertices);
        let res = self.draw(&geometry, uniforms, blend);
        res.map(|()| geometry)
    }

    pub fn finish_frame(&mut self) -> JamResult<()> {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().map_err(JamError::ContextError)?;
        self.device.cleanup();
        Ok(())
    }

    pub fn draw_view<Ev>(&mut self, view:&View<Ev>) -> JamResult<()> {
        let tick = self.ui.tick;
        let store_texture_size = self.ui.dimensions;
        let mut vertices = Vec::new();

        let tesselator = GeometryTesselator {
            scale: vec3(1.0, 1.0, 1.0),
            color: color::WHITE.float_raw(),
        };

        for (layer, rect_abs, (v_z, l_z)) in view.layer_iter() {
            let size = rect_abs.size();
            let sized_element = ElementWithSize {
                element: layer.content.clone(),
                size: size,
            };

            use std::collections::hash_map::Entry;

            let (raster_translation, region) = match self.ui.elements.entry(sized_element) {
                Entry::Occupied(mut oe) => {
                    let re = oe.get_mut();
                    re.last_used = tick;
                    (re.translation, re.texture_region)
                },
                Entry::Vacant(mut ve) => {
                    println!("RASTER -> {:?} @ {:?}", layer, tick);
                    let (img, translation) = raster(&layer.content, size, self.ui.fonts.as_slice());
                    let use_layer = self.ui.free_layers.pop().expect("a free layer");

                    let region = TextureRegion {
                        u_min: 0,
                        u_max: img.width() as u32,
                        v_min: 0,
                        v_max: img.height() as u32,
                        layer: use_layer as u32,
                        texture_size: store_texture_size.width,
                    };
                    let re = RasterElement {
                        translation: translation, // translation from requested origin to output area
                        texture_region: region,
                        last_used: tick,
                    };
                    let mut image_info = ImageInfoCommon {
                        xoffset: 0,
                        yoffset: 0,
                        zoffset: use_layer as u16,
                        width: img.width() as u16,
                        height: img.height() as u16,
                        depth: 1,
                        format: (),
                        mipmap: 0,
                    };

                    let mut data : Vec<[u8; 4]> = img.into_raw().chunks(4).map(|sl| [sl[0], sl[1], sl[2], sl[3]]).collect();
                    self.encoder.update_texture::<R8_G8_B8_A8, Srgba8>(
                        &self.ui.texture_resource,
                        None,
                        image_info,
                        &data,
                    ).expect("updating the texture");
                    ve.insert(re);
                    (translation, region)
                },
            };

            let position = raster_translation + rect_abs.min;
            let z = (v_z as f64) * 1.0 + (l_z as f64) * 0.1;

            tesselator.draw_ui(&mut vertices, &region, position.x as f64, position.y as f64, z, 1.0);
        }

        // transform
        let (pixel_width, pixel_height) = self.dimensions.pixels;

//        println!("screen dimensions {:?} {:?}", pixel_width, pixel_height);
        let transform = ui_projection(pixel_width as f64, pixel_height as f64);
        let uniforms = Uniforms {
            transform : down_size_m4(transform.into()),
            color: color::WHITE,
        };
        let geo = self.upload(&vertices);

        let mut reclaim_elements : Vec<ElementWithSize<i32>> = Vec::new();

        for (element, entry) in self.ui.elements.iter() {
            if entry.last_used < self.ui.tick {
                let freed_layer = entry.texture_region.layer;
//                println!("reclaiming entry -> {:?}, freed layer {:?}", element, freed_layer);
                self.ui.free_layers.push(freed_layer);
                reclaim_elements.push(element.clone());
            }
        }

        self.ui.tick += 1;

        if !reclaim_elements.is_empty() {
            for e in &reclaim_elements {
                self.ui.elements.remove(e);
            }
        }


        self.draw_raw(&geo, uniforms, Blend::Alpha, TextureArraySource::UI)
    }

    //        for l in 0..8 {
    //            let region = TextureRegion {
    //                u_min: 0,
    //                u_max: 512,
    //                v_min: 0,
    //                v_max: 512,
    //                layer: l,
    //                texture_size: 512,
    //            };
    //            let scale = 0.10;
    //
    //            let x = 512.0 * scale * 1.1 * l as f64;
    //            let y = 0.0;
    //
    //            tesselator.draw_ui(&mut vertices, &region, x, y, 0.0, scale);
    //        }
}

pub fn texture_kind_for(dimensions: &TextureArrayDimensions) -> gfx::texture::Kind {
    gfx::texture::Kind::D2Array(dimensions.width as u16, dimensions.height as u16, dimensions.layers as u16, gfx::texture::AaMode::Single)
}

pub fn check_reload(rx: &Receiver<RawEvent>, files:&FileResources) -> (bool, bool) {
    let mut reload_program = false;
    let mut reload_texture = false;

    'fs: loop {
        match rx.try_recv() {
            Ok(RawEvent { path, op:_, cookie:_ }) => {
                if let Some(p) = path {
                    if files.shader_pair.contains(&p) {
                        reload_program = true;
                    } else if files.texture_directory.contains(&p) {
                        reload_texture = true;
                    } else {
                        use std::path;
                        let components: Vec<path::Component> = p.components().collect();
                        println!("fs event {:?} -> {:?}", p, components);
                    }
                }
            },
            Err(_) => break 'fs,
        }
    }

    (reload_program, reload_texture)
}