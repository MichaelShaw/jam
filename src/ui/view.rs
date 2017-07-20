use ui::{Rect, Layer, MouseEvent, ZLayer, Point2I, RectI, Text, Element};
use cgmath::vec2;

use std::fmt;
use std::fmt::Debug;

// man, that function ... can't even clone that thing
pub struct View<Ev> {
    pub frame: Rect<i32>,
    pub on_event: Option<Box<Fn(MouseEvent) -> Option<Ev>>>,
    pub layers: Vec<Layer>,
    pub sub_views : Vec<View<Ev>>,
}

impl<Ev> fmt::Debug for View<Ev> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "View {{ frame: {:?}, layers: {:?} sub_view: {:?} }}", self.frame, self.layers, self.sub_views)
    }
}


impl<Ev> View<Ev> {
    pub fn iter<'a>(&'a self) -> ViewIterator<'a, Ev> {
        let queue = vec![(self, vec2(0, 0), 0)];
        ViewIterator {
            queue,
        }
    }

    pub fn layer_iter<'a>(&'a self) -> LayerIterator<'a, Ev> {
        let queue = vec![(self, vec2(0, 0), 0)];
        LayerIterator {
            views: queue,
            layer_idx: 0,
        }
    }
}

pub struct ViewIterator<'a, Ev> where Ev: 'a {
    queue : Vec<(&'a View<Ev>, Point2I, ZLayer)>,
}

impl<'a, Ev> Iterator for ViewIterator<'a, Ev> {
    type Item = (&'a View<Ev>, RectI, ZLayer);

    fn next(&mut self) -> Option<(&'a View<Ev>, RectI, ZLayer)> {
        if let Some((view, parent_origin, z)) = self.queue.pop() {
            for sv in &view.sub_views {
                self.queue.push((sv, parent_origin + view.frame.min, z+1));
            }
            Some((view, view.frame.offset(parent_origin), z))
        } else {
            None
        }
    }
}

pub struct LayerIterator<'a, Ev> where Ev: 'a {
    views : Vec<(&'a View<Ev>, Point2I, ZLayer)>,
    layer_idx: usize,
}

impl<'a, Ev> Iterator for LayerIterator<'a, Ev> {
    type Item = (&'a Layer, RectI, (ZLayer, ZLayer));

    fn next(&mut self) -> Option<(&'a Layer, RectI, (ZLayer, ZLayer))> {
        if let Some(&(view, parent_origin, z)) = self.views.last() {
            if self.layer_idx < view.layers.len() {
                let l = &view.layers[self.layer_idx];
                let layer_z = self.layer_idx as ZLayer;
                self.layer_idx += 1;
                let layer_frame = l.frame.offset(parent_origin + view.frame.min);
                return Some((l, layer_frame, (z, layer_z)));
            } else {
                // reset layer for next view
                self.layer_idx = 0;
            }
        }

        // no more layers
        if let Some((view, parent_origin, z)) = self.views.pop() {
            for sv in &view.sub_views {
                self.views.push((sv, parent_origin + view.frame.min, z+1));
            }
            self.next()
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    pub fn empty_view(at:Point2I) -> View<()> {
        View {
            frame : Rect {
                min: at,
                max: at + vec2(10, 10),
            },
            on_event: None,
            layers : Vec::new(),
            sub_views: Vec::new(),
        }
    }

    pub fn view_with_text(at:Point2I) -> View<()> {
        let mut layers = Vec::new();

        for i in 0..3 {

            layers.push(Layer {
                frame: Rect {
                    min: vec2(0, i * 30),
                    max: vec2(100, (i + 1) * 30),
                },
                content: Element::Text(Text::new("awesome".into())),
            });
        }

        View {
            frame : Rect {
                min: at,
                max: at + vec2(50, 100),
            },
            on_event: None,
            layers : layers,
            sub_views: Vec::new(),
        }
    }


    #[test]
    fn view_iterator() {
        let mut v = empty_view(vec2(100, 100));
        for i in 0..3 {
            v.sub_views.push(empty_view(vec2(10 * i, 10)));
        }

        println!("====== VIEWS =======");

        for (v, r, z) in v.iter() {
            println!("view -> {:?} rect -> {:?} z -> {:?}", v, r, z);
        }
    }

    #[test]
    fn layer_iterator() {
        let mut v = empty_view(vec2(100, 100));
        for i in 0..3 {
            v.sub_views.push(view_with_text(vec2(100 * i, 0)));
        }

        println!("====== LAYERS =======");

        for (l, r, z) in v.layer_iter() {
            println!("layer -> {:?} rect -> {:?} z -> {:?}", l, r, z);
        }
    }
}