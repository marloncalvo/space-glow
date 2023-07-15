#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod context;
pub use context::Context;

pub trait Shape {
    fn render(&self, context: Context);
}

pub struct Position {
    x: u32,
    y: u32
}

pub struct Rectangle {
    /// Height of `Rectangle` in pixels.
    height: u32,
    /// Width of `Rectangle` in pixels.
    width: u32,
    /// `Position` representing the upper left corner of `Rectangle`
    pos: Position
}

pub struct Primitive {

}

impl Rectangle {
    pub fn new(height: u32, width: u32, pos: Position) -> Self {
        Self {
            height,
            width,
            pos,
        }
    }
}

/// Renderer exposes methods to render several primitive shapes, such as:
/// - Rectangles
/// - Triangles
/// - Circles
///
/// Internally, it will keep a list of all the shapes and pass these shapes to the `Context`
/// rendering pipeline.
pub struct Renderer {
    context: Context,
    shapes: Vec<Box<dyn Shape>>,
}

impl Renderer {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            shapes: vec![],
        }
    }

    pub fn render_loop() {

    }

    pub fn render(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    fn primitives(&self) -> Vec<Primitive> {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
