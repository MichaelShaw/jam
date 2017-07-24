#![allow(dead_code)]

pub mod gfx;

pub mod command;
pub mod quads;
pub mod shader;
pub mod text;
pub mod texture_array;
pub mod texture_region;

pub use self::command::*;
pub use self::quads::*;
pub use self::shader::*;
pub use self::text::*;
pub use self::texture_array::*;
pub use self::texture_region::*;

pub use self::gfx::Vertex;

use font::FontDirectory;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, RawEvent, FsEventWatcher};
use std::sync::mpsc::{channel, Receiver};


pub fn down_size_m4(arr: [[f64; 4];4]) -> [[f32; 4]; 4] {
    let mut out : [[f32; 4]; 4] = [[0.0; 4]; 4];
    for a in 0..4 {
        for b in 0..4 {
            out[a][b] = arr[a][b] as f32
        }
    }

    out
}


pub struct FileResources {
    pub shader_pair : ShaderPair,
    pub texture_directory: TextureDirectory,
    pub font_directory: FontDirectory,

}

pub struct FileWatcher {
    pub watcher : RecommendedWatcher,
    pub change_events: Receiver<RawEvent>,
}

impl FileResources {
    pub fn watch(&self) -> FileWatcher {
        let (tx, notifier_rx) = channel::<RawEvent>();

        let mut resource_file_watcher : RecommendedWatcher = Watcher::new_raw(tx).expect("a watcher");
        resource_file_watcher.watch(&self.shader_pair.vertex_path, RecursiveMode::Recursive).expect("watching shader vertex path");
        resource_file_watcher.watch(&self.shader_pair.fragment_path, RecursiveMode::Recursive).expect("watching shader fragment path");
        resource_file_watcher.watch(&self.texture_directory.path, RecursiveMode::Recursive).expect("watching texture directory path");

        FileWatcher {
            watcher: resource_file_watcher,
            change_events: notifier_rx,
        }
    }
}