use ui::{Rect, Layer, MouseEvent, ZLayer, Point2I, RectI};
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
            Some((view, view.frame.offset(&parent_origin), z))
        } else {
            None
        }
    }
}

//pub struct ElementIterator<'a, Ev> where Ev: 'a {
//    queue : Vec<(&'a, View<Ev>>
//}



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
                content: Element::T,
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

        for (v, r, z) in v.iter() {
            println!("view -> {:?} rect -> {:?} z -> {:?}", v, r, z);
        }


    }
}