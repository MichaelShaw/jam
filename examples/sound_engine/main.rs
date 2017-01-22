#![allow(dead_code)]


extern crate jam;

use jam::audio::*;

fn main() {
    let something = jam::audio::engine::do_sample(32, 4);
}