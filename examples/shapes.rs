#![deny(clippy::all)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::single_match)]

use rgx::core::*;
use rgx::kit;
use rgx::kit::shape2d::{Line, Shape, ShapeView};

use cgmath::prelude::*;
use cgmath::Matrix4;

use wgpu::winit::{
    ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowEvent,
};

fn main() {
    env_logger::init();

    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).unwrap();

    ///////////////////////////////////////////////////////////////////////////
    // Setup renderer
    ///////////////////////////////////////////////////////////////////////////

    let mut r = Renderer::new(&window);

    let mut win = window
        .get_inner_size()
        .unwrap()
        .to_physical(window.get_hidpi_factor());

    let mut pip: kit::shape2d::Pipeline = r.pipeline(win.width as u32, win.height as u32);
    let mut running = true;

    ///////////////////////////////////////////////////////////////////////////
    // Render loop
    ///////////////////////////////////////////////////////////////////////////

    let (sw, sh) = (32., 32.);
    let mut rows: u32;
    let mut cols: u32;

    while running {
        events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => match key {
                        VirtualKeyCode::Escape => {
                            running = false;
                        }
                        _ => {}
                    },
                    WindowEvent::CloseRequested => {
                        running = false;
                    }
                    WindowEvent::Resized(size) => {
                        win = size.to_physical(window.get_hidpi_factor());

                        let (w, h) = (win.width as u32, win.height as u32);

                        pip.resize(w, h);
                        r.resize(w, h);
                    }
                    _ => {}
                }
            }
        });

        rows = (win.height as f32 / sh) as u32;
        cols = (win.width as f32 / (sw / 2.0)) as u32;

        ///////////////////////////////////////////////////////////////////////////
        // Prepare shape view
        ///////////////////////////////////////////////////////////////////////////

        let mut sv = ShapeView::new();

        for i in 0..rows {
            let y = i as f32 * sh;

            for j in 0..cols {
                let x = j as f32 * sw - sw / 2.0;

                let shape = if i * j % 2 == 0 {
                    Shape::Line(
                        Line::new(x, y, x + sw, y + sh),
                        1.0,
                        Rgba::new(i as f32 / rows as f32, j as f32 / cols as f32, 0.5, 0.75),
                    )
                } else {
                    Shape::Rectangle(
                        Rect::new(x, y, x + sw, y + sh),
                        2.0,
                        Rgba::new(i as f32 / rows as f32, j as f32 / cols as f32, 0.5, 0.75),
                    )
                };
                sv.add(shape);
            }
        }

        let buffer = sv.finish(&r);

        ///////////////////////////////////////////////////////////////////////////
        // Create frame
        ///////////////////////////////////////////////////////////////////////////

        let mut frame = r.frame();

        ///////////////////////////////////////////////////////////////////////////
        // Prepare pipeline
        ///////////////////////////////////////////////////////////////////////////

        frame.prepare(&pip, Matrix4::identity());

        ///////////////////////////////////////////////////////////////////////////
        // Draw frame
        ///////////////////////////////////////////////////////////////////////////

        let pass = &mut frame.pass(Rgba::TRANSPARENT);

        pass.apply_pipeline(&pip);
        pass.set_vertex_buffer(&buffer);
        pass.draw_buffer(0..buffer.size, 0..1);
    }
}
