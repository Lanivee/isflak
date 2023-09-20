#![allow(unused)]
use glam::*;
use line_drawing::Bresenham;
use multimap::MultiMap;
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

    pub fn set_pixel(&mut self, position: UVec2, color: Vec4) {
        let u8_color = [
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
            (color.w * 255.0) as u8,
        ];

        let x = if position.x > 0 { position.x - 1 } else { 0 };
        let y = if position.y > 0 { position.y - 1 } else { 0 };

        self.frame_buffer[(x + y * self.width) as usize] = u8_color;
    }

    pub fn draw_vertices(&mut self, vertices: &[Vertex]) {
        let mut draw_pixels = vec![];

        for vertex in 0..vertices.len() {
            let v0 = &vertices[vertex];
            let v1 = if vertex < vertices.len() - 1 {
                &vertices[vertex + 1]
            } else {
                &vertices[vertex - 2]
            };

            let v0coords = ndc_to_pixel(v0.pos, self.width, self.height).as_ivec2();
            let v1coords = ndc_to_pixel(v1.pos, self.width, self.height).as_ivec2();

            let mut temp_pixels = vec![];

            for (x, y) in Bresenham::new((v0coords.x, v0coords.y), (v1coords.x, v1coords.y)) {
                temp_pixels.push((x, y));
            }

            let mut current_pixel = 0;
            for pos in &temp_pixels {
                let color = lerp(v0.color, v1.color, current_pixel, temp_pixels.len() as u32);
                current_pixel += 1;

                let pixel = Pixel {
                    pos: uvec2(pos.0 as u32, pos.1 as u32),
                    color,
                };

                draw_pixels.push(pixel);
            }
        }

        for _ in 0..vertices.len() / 3 {
            let mut last_scanline = -1;
            let mut scanlines = MultiMap::new();

            for pixel in &draw_pixels {
                let scanline = pixel.pos.y;
                if last_scanline != scanline as i32 {
                    scanlines.insert((scanline), (pixel.pos.x, pixel.color));
                }
                last_scanline = scanline as i32;
            }

            let min = scanlines.keys().min().unwrap();
            let max = scanlines.keys().max().unwrap();

            for scanline in *min..*max {
                let mut start_end = scanlines.get_vec_mut(&scanline).unwrap();
                if start_end.len() >= 2 {
                    let (start_pos, end_pos) = if start_end[0].0 < start_end[1].0 {
                        (start_end[0].0, start_end[1].0)
                    } else {
                        (start_end[1].0, start_end[0].0)
                    };
                    let line_length = end_pos - start_pos;
                    let start_color = start_end[0].1;
                    let end_color = start_end[1].1;

                    for x in start_pos..end_pos {
                        let color = lerp(start_color, end_color, x - start_pos, line_length);
                        let pos = uvec2(x, scanline);

                        let pixel = Pixel { pos, color };

                        draw_pixels.push(pixel);
                    }
                }
            }
        }

        for pixel in draw_pixels {
            self.set_pixel(pixel.pos, pixel.color);
        }
    }
}

pub struct Vertex {
    pub pos: Vec2,
    pub color: Vec4,
}

struct Pixel {
    pos: UVec2,
    color: Vec4,
}

fn ndc_to_pixel(ndc: Vec2, screen_width: u32, screen_height: u32) -> UVec2 {
    uvec2(
        (ndc.x * screen_width as f32) as u32,
        (ndc.y * screen_height as f32) as u32,
    )
}

fn lerp(a: Vec4, b: Vec4, current_pixel: u32, line_length: u32) -> Vec4 {
    let frac1 = (line_length - current_pixel) as f32 / line_length as f32;
    let frac2 = current_pixel as f32 / line_length as f32;

    a * frac1 + b * frac2
}
