mod renderer;
use renderer::*;

use glam::*;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Software Raster")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let r_width = WIDTH;
    let r_height = HEIGHT;
    let mut renderer = Renderer::new(&window, r_width, r_height);

    let vertices = [
        Vertex {
            pos: vec2(0.5, 0.0),
            color: vec4(1.0, 0.0, 0.0, 1.0),
        },
        Vertex {
            pos: vec2(0.0, 1.0),
            color: vec4(0.0, 1.0, 0.0, 1.0),
        },
        Vertex {
            pos: vec2(1.0, 1.0),
            color: vec4(0.0, 0.0, 1.0, 1.0),
        },
    ];

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            renderer.begin_frame();

            //Drawing Code
            renderer.draw_vertices(&vertices);

            renderer.draw_frame();
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {}

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}
