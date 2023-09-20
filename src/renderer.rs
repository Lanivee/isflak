#![allow(unused)]
use line_drawing::Bresenham;
use pixels::raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use pixels::{Error, Pixels, SurfaceTexture};
use std::collections::HashMap;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub struct Renderer {
    pixels: Pixels,
    frame_buffer: Vec<[u8; 4]>,
    width: u32,
    height: u32,
    frame_done: bool,
}

impl Renderer {
    pub fn new(window: &winit::window::Window, width: u32, height: u32) -> Self {
        let scale = if window.inner_size().width > width {
            window.inner_size().width / width
        } else if width > window.inner_size().width {
            width / window.inner_size().width
        } else {
            1
        };

        let surface_texture = SurfaceTexture::new(width * scale, height * scale, window);

        Self {
            pixels: Pixels::new(width, height, surface_texture).unwrap(),
            frame_buffer: Vec::new(),
            width,
            height,
            frame_done: true,
        }
    }

    pub fn begin_frame(&mut self) {
        if self.frame_done {
            self.frame_done = false;
            self.frame_buffer = vec![[0x00, 0x00, 0x00, 0xff]; (self.width * self.height) as usize];
        }
    }

    pub fn draw_frame(&mut self) {
        if !self.frame_done {
            for (i, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                pixel.copy_from_slice(&self.frame_buffer[i]);
            }

            self.pixels.render();
            self.frame_done = true;
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        self.frame_buffer[(x + y * self.width) as usize] = color;
    }

    pub fn draw_polygon(&mut self, v0: &Vertex, v1: &Vertex, v2: &Vertex) {
        let mut v0v1 = vec![];
        let mut v1v2 = vec![];
        let mut v2v0 = vec![];

        for (x, y) in Bresenham::new(v0.pos, v1.pos) {
            v0v1.push((x, y))
        }

        for (x, y) in Bresenham::new(v1.pos, v2.pos) {
            v1v2.push((x, y))
        }

        for (x, y) in Bresenham::new(v2.pos, v0.pos) {
            v2v0.push((x, y))
        }

        let mut pixels = vec![];

        for pixel in 0..v0v1.len() {
            let pos = v0v1[pixel];
            let col = lerp(v0.col, v1.col, pixel as u32, v0v1.len() as u32);

            pixels.push((pos, col))
        }

        for pixel in 0..v1v2.len() {
            let pos = v1v2[pixel];
            let col = lerp(v1.col, v2.col, pixel as u32, v1v2.len() as u32);

            pixels.push((pos, col))
        }

        for pixel in 0..v2v0.len() {
            let pos = v2v0[pixel];
            let col = lerp(v2.col, v0.col, pixel as u32, v2v0.len() as u32);

            pixels.push((pos, col))
        }

        for pixel in pixels {
            self.set_pixel(
                pixel.0 .0 as u32,
                pixel.0 .1 as u32,
                [pixel.1 .0, pixel.1 .1, pixel.1 .2, 255],
            );
        }

        let mut pixels2 = HashMap::new();
    }
}

pub struct Vertex {
    pub pos: (i32, i32),
    pub col: (u8, u8, u8),
}

fn lerp(a: (u8, u8, u8), b: (u8, u8, u8), current_pixel: u32, line_length: u32) -> (u8, u8, u8) {
    let frac1 = (line_length - current_pixel) as f32 / line_length as f32;
    let frac2 = current_pixel as f32 / line_length as f32;

    (
        (a.0 as f32 * frac1 + b.0 as f32 * frac2) as u8,
        (a.1 as f32 * frac1 + b.1 as f32 * frac2) as u8,
        (a.2 as f32 * frac1 + b.2 as f32 * frac2) as u8,
    )
}
