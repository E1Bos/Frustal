use crate::args::Args;
use crate::fractals;

use num::Complex;
use pixels::{Pixels, SurfaceTexture};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

pub struct RendererRunner {
    args: Option<Args>,
}

impl RendererRunner {
    pub fn new() -> Self {
        RendererRunner { args: None }
    }

    pub fn with_args(mut self, args: Args) -> Self {
        self.args = Some(args);
        self
    }

    pub fn run(self) -> Result<(), EventLoopError> {
        let args = self
            .args
            .expect("Args must be set before running the renderer");

        let event_loop: EventLoop<()> = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);

        let mut renderer = Renderer::new(args);
        event_loop.run_app(&mut renderer)?;

        Ok(())
    }
}

struct Renderer {
    // Window specific properties
    window: Option<Window>,
    pixels: Option<Pixels>,
    needs_redraw: bool,

    args: Args,

    // Fractal specific properties
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    zoom_factor: f64,
    pan_factor: f64,
}

impl Renderer {
    fn new(args: Args) -> Self {
        Self {
            window: Default::default(),
            pixels: Default::default(),
            needs_redraw: true,
            upper_left: args.get_upper_left(),
            lower_right: args.get_lower_right(),
            zoom_factor: 0.05,
            pan_factor: 0.05,
            args,
        }
    }
}

impl ApplicationHandler for Renderer {
    /// Called when the window is created.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // let fullscreen = if self.args.fullscreen {
        //     Some(winit::window::Fullscreen::Borderless(None))
        // } else {
        //     None
        // };
        let fullscreen = None;

        let window_attributes = Window::default_attributes()
            .with_title("Fractals")
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.args.get_width() as f64,
                self.args.get_height() as f64,
            ))
            .with_fullscreen(fullscreen);

        self.window = Some(event_loop.create_window(window_attributes).unwrap());

        let surface_texture = SurfaceTexture::new(
            self.args.get_width(),
            self.args.get_height(),
            self.window.as_ref().unwrap(),
        );
        self.pixels = Some(
            Pixels::new(
                self.args.get_width(),
                self.args.get_height(),
                surface_texture,
            )
            .unwrap(),
        );
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
                        self.pan(0.0, 1.0);
                    }
                    PhysicalKey::Code(KeyCode::ArrowDown) => {
                        self.pan(0.0, -1.0);
                    }
                    PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        self.pan(1.0, 0.0);
                    }
                    PhysicalKey::Code(KeyCode::ArrowRight) => {
                        self.pan(-1.0, 0.0);
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

impl Renderer {
    /// Zoom into the fractal
    ///
    /// # Arguments
    ///
    /// * `factor` - The amount to zoom into the fractal
    fn zoom(&mut self, zoom_in: bool) {
        let factor = if zoom_in { 1.0 - self.zoom_factor } else { 1.0 / 1.0 + self.zoom_factor };
        
        let center_x = (self.upper_left.re + self.lower_right.re) / 2.0;
        let center_y = (self.upper_left.im + self.lower_right.im) / 2.0;
    
        let x_range = (self.lower_right.re - self.upper_left.re) / 2.0;
        let y_range = (self.lower_right.im - self.upper_left.im) / 2.0;
    
        self.upper_left.re = center_x - x_range * factor;
        self.upper_left.im = center_y - y_range * factor;
        self.lower_right.re = center_x + x_range * factor;
        self.lower_right.im = center_y + y_range * factor;
    
        self.needs_redraw = true;

        // let x_center = (self.x.lower + self.x.upper) / 2.0;
        // let y_center = (self.y.lower + self.y.upper) / 2.0;
        // let x_range = (self.x.upper - self.x.lower) / 2.0;
        // let y_range = (self.y.upper - self.y.lower) / 2.0;

        // self.x.lower = x_center - x_range * zoom_factor;
        // self.x.upper = x_center + x_range * zoom_factor;
        // self.y.lower = y_center - y_range * zoom_factor;
        // self.y.upper = y_center + y_range * zoom_factor;

        // self.needs_redraw = true;
    
    }

    /// Pan around the fractal
    ///
    /// # Arguments
    ///
    /// * `x` - The pixels to pan in the X axis
    /// * `y` - The pixels to pan in the Y axis
    fn pan(&mut self, x: f64, y: f64) {
        let x_range = self.upper_left.re - self.lower_right.re;
        let y_range = self.upper_left.im - self.lower_right.im;

        self.upper_left.re += x * x_range * self.pan_factor;
        self.upper_left.im += y * y_range * self.pan_factor;
        self.lower_right.re += x * x_range * self.pan_factor;
        self.lower_right.im += y * y_range * self.pan_factor;

        self.needs_redraw = true;
    }
}

impl Renderer {
    fn render_fractal(&mut self) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();

            let width = self.args.get_width();
            let height = self.args.get_height();
            let max_iterations = self.args.get_max_iterations();

            let upper_left_re = &self.upper_left.re;
            let upper_left_im = &self.upper_left.im;
            let lower_right_re = &self.lower_right.re;
            let lower_right_im = &self.lower_right.im;


            frame.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
                let x = i as u32 % width;
                let y = i as u32 / width;

                let c_re = upper_left_re + (x as f64 / width as f64)
                    * (lower_right_re - upper_left_re);
                let c_im = upper_left_im + (y as f64 / height as f64)
                    * (lower_right_im - upper_left_im);
                let c = Complex::new(c_re, c_im);

                let iter = fractals::mandelbrot(c, max_iterations);
                let intensity = if iter == max_iterations {
                    0
                } else {
                    (iter as f64 / max_iterations as f64 * 255.0) as u8
                };
                let color = [intensity, intensity, intensity, 255];
                pixel.copy_from_slice(&color);
            });

            pixels.render().unwrap();
        }
    }
}
