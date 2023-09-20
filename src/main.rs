mod renderer;
use renderer::*;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

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
            pos: (r_width as i32 / 2, 0 + r_height as i32 / 4),
            col: (255, 0, 0),
        },
        Vertex {
            pos: (
                0 + r_width as i32 / 4,
                r_height as i32 / 2 + r_height as i32 / 4,
            ),
            col: (0, 0, 255),
        },
        Vertex {
            pos: (
                r_width as i32 - r_width as i32 / 4,
                r_height as i32 / 2 + r_height as i32 / 4,
            ),
            col: (0, 255, 0),
        },
        Vertex {
            pos: (r_width as i32 / 2 + 10, 0 + r_height as i32 / 4 - 20),
            col: (255, 0, 0),
        },
        Vertex {
            pos: (
                0 + r_width as i32 / 4 + 100,
                r_height as i32 / 2 + r_height as i32 / 4 - 60,
            ),
            col: (0, 0, 255),
        },
        Vertex {
            pos: (
                r_width as i32 - r_width as i32 / 4 + 80,
                r_height as i32 / 2 + r_height as i32 / 4 + 30,
            ),
            col: (0, 255, 0),
        },
    ];

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            renderer.begin_frame();

            //Drawing Code
            renderer.draw_polygon(&vertices[0], &vertices[1], &vertices[2]);
            renderer.draw_polygon(&vertices[3], &vertices[4], &vertices[5]);

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
