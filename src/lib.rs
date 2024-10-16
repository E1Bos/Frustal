#![allow(unused_imports)]

use num::Complex;
use pixels::{Pixels, SurfaceTexture};
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};
mod fractals;

pub struct Args {
    width: u32,
    height: u32,
    max_iterations: u32,
    fullscreen: bool,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
}

impl Args {
    pub fn new(
        width: u32,
        height: u32,
        max_iterations: u32,
        fullscreen: bool,
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
    ) -> Self {
        Self {
            width,
            height,
            max_iterations,
            fullscreen,
            upper_left,
            lower_right,
        }
    }

    pub fn default() -> Self {
        Self::new(
            500,
            500,
            255,
            false,
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        )
    }
}

struct Range<T> {
    lower: T,
    upper: T,
}

pub struct App {
    args: Args,
    window: Option<Window>,
    pixels: Option<Pixels>,
    x: Range<f64>,
    y: Range<f64>,
    zoom_factor: f64,
    pan_factor: f64,
    needs_redraw: bool,
}

impl App {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            window: Default::default(),
            pixels: Default::default(),
            x: Range {
                lower: -1.20,
                upper: -1.0,
            },
            y: Range {
                lower: 0.20,
                upper: 0.35,
            },
            zoom_factor: 0.9,
            pan_factor: 0.1,
            needs_redraw: true,
        }
    }

    pub fn default() -> Self {
        Self::new(Args::default())
    }
}

impl ApplicationHandler for App {
    /// Called when the window is created.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let fullscreen = if self.args.fullscreen {
            Some(winit::window::Fullscreen::Borderless(None))
        } else {
            None
        };

        let window_attributes = Window::default_attributes()
            .with_title("Fractals")
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.args.width as f64,
                self.args.height as f64,
            ))
            .with_fullscreen(fullscreen);

        self.window = Some(event_loop.create_window(window_attributes).unwrap());

        let surface_texture = SurfaceTexture::new(
            self.args.width,
            self.args.height,
            self.window.as_ref().unwrap(),
        );
        self.pixels =
            Some(Pixels::new(self.args.width, self.args.height, surface_texture).unwrap());
    }

    /// Called when the window receives an event.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            // Closes the window when the close button is pressed
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            // Redraw the fractal if needed
            WindowEvent::RedrawRequested => {
                // Redraw the fractal if needed
                if self.needs_redraw {
                    self.render_fractal();
                    self.needs_redraw = false;
                }

                self.window.as_ref().unwrap().request_redraw();
            }
            // Handle User Input
            WindowEvent::KeyboardInput { event, .. } => {
                // Handle pan with keyboard inputs
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::ArrowUp) => {
                        self.pan(0.0, -1.0);
                    }
                    PhysicalKey::Code(KeyCode::ArrowDown) => {
                        self.pan(0.0, 1.0);
                    }
                    PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        self.pan(-1.0, 0.0);
                    }
                    PhysicalKey::Code(KeyCode::ArrowRight) => {
                        self.pan(1.0, 0.0);
                    }
                    PhysicalKey::Code(KeyCode::Equal) => {
                        self.zoom(true);
                    }
                    PhysicalKey::Code(KeyCode::Minus) => {
                        self.zoom(false);
                    }
                    _ => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => {
                    self.zoom(y > 0.0);
                    self.needs_redraw = true;
                }
                _ => (),
            },
            _ => (),
        }
    }
}

impl App {
    /// Zoom into the fractal
    ///
    /// # Arguments
    ///
    /// * `factor` - The amount to zoom into the fractal
    fn zoom(&mut self, zoom_in: bool) {

        let zoom_factor = if zoom_in {
            self.zoom_factor
        } else {
            1.0 / self.zoom_factor
        };

        let x_center = (self.x.lower + self.x.upper) / 2.0;
        let y_center = (self.y.lower + self.y.upper) / 2.0;
        let x_range = (self.x.upper - self.x.lower) / 2.0;
        let y_range = (self.y.upper - self.y.lower) / 2.0;

        self.x.lower = x_center - x_range * zoom_factor;
        self.x.upper = x_center + x_range * zoom_factor;
        self.y.lower = y_center - y_range * zoom_factor;
        self.y.upper = y_center + y_range * zoom_factor;

        self.needs_redraw = true;
    }

    /// Pan around the fractal
    ///
    /// # Arguments
    ///
    /// * `x` - The pixels to pan in the X axis
    /// * `y` - The pixels to pan in the Y axis
    fn pan(&mut self, x: f64, y: f64) {
        let x_range = self.x.upper - self.x.lower;
        let y_range = self.y.upper - self.y.lower;

        self.x.lower += x * x_range * self.pan_factor;
        self.x.upper += x * x_range * self.pan_factor;
        self.y.lower += y * y_range * self.pan_factor;
        self.y.upper += y * y_range * self.pan_factor;

        self.needs_redraw = true;
    }
}

impl App {
    fn render_fractal(&mut self) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();

            // Render only the next batch of rows
            frame.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
                let x = i as u32 % self.args.width;
                let y = i as u32 / self.args.width;

                // Map pixel to complex plane
                let c_re = self.x.lower
                    + (x as f64 / self.args.width as f64) * (self.x.upper - self.x.lower);
                let c_im = self.y.lower
                    + (y as f64 / self.args.height as f64) * (self.y.upper - self.y.lower);
                let c = Complex::new(c_re, c_im);

                // Calculate Mandelbrot iterations using the modified mandelbrot function
                let iter = fractals::mandelbrot(c, self.args.max_iterations);
                // Determine color based on iterations
                let intensity = if iter == self.args.max_iterations {
                    0
                } else {
                    (iter as f64 / self.args.max_iterations as f64 * 255.0) as u8
                };
                let color = [intensity, intensity, intensity, 255]; // Grayscale for points in range
                pixel.copy_from_slice(&color);
            });

            pixels.render().unwrap();
        }
    }
}




// Create a context with a high precision
// let ctx = Context::new(100); // 100 decimal places

// // Define the zoom function using BigFloat
// fn zoom(x: BigFloat, y: BigFloat, zoom_factor: BigFloat) -> (BigFloat, BigFloat) {
//     let x_center = (x + 1.0) / 2.0;
//     let y_center = (y + 1.0) / 2.0;
//     let x_range = (x - 1.0) / 2.0;
//     let y_range = (y - 1.0) / 2.0;

//     let new_x = x_center - x_range * zoom_factor;
//     let new_y = y_center - y_range * zoom_factor;

//     (new_x, new_y)
// }

// // Use the zoom function in your code
// let x = BigFloat::from(0.5);
// let y = BigFloat::from(0.5);
// let zoom_factor = BigFloat::from(2.0);

// let (new_x, new_y) = zoom(x, y, zoom_factor);