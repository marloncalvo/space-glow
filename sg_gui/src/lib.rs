#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use sg_graphics::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct GUI {
    context: Context,
    evt_loop: winit::event_loop::EventLoop<()>,
}

impl GUI {
    pub async fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let context = Context::new(window).await;

        Self {
            context: context,
            evt_loop: event_loop,
        }
    }

    pub fn run(self) {
        self.evt_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.context.window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                _ => {}
            });
    }
}