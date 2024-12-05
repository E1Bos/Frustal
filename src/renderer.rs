use crate::args::{Args, ColorScheme, ScanConfig};
use crate::fractals::{color_map, mandelbrot, ColorMode};
use pixels::{Error, Pixels, SurfaceTexture};
use rayon::prelude::*;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

pub struct Renderer {
    width: u32,
    height: u32,
    center_x: f64,
    center_y: f64,
    scale: f64,
    max_iterations: u32,
    color_scheme: ColorScheme,
    scan_level: u32,
    scan_config: ScanConfig,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            center_x: -0.5,
            center_y: 0.0,
            scale: 2.5,
            max_iterations: 200,
            color_scheme: ColorScheme::Smooth,
            scan_level: 0,
            scan_config: ScanConfig::default(),
        }
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.center_x += dx * self.scale * 0.3;
        self.center_y += dy * self.scale * 0.3;
        if self.scan_config.enabled {
            self.scan_level = 0;
        }
    }

    pub fn zoom(&mut self, factor: f64) {
        let new_scale = self.scale * factor;
        if new_scale <= 10.0 {
            self.scale = new_scale;

            if self.scan_config.enabled {
                self.scan_level = 0;
            }
        }
    }

    pub fn render(&mut self, frame: &mut [u8]) {
        if !self.scan_config.enabled {
            // Regular rendering without scanning
            self.render_full(frame);
            return;
        }

        // Calculate stride based on current scan level
        let stride = if self.scan_level == 0 {
            self.scan_config.initial_stride
        } else {
            self.scan_config.initial_stride >> self.scan_level
        };

        if stride < 1 {
             // All passes completed
            return;
        }

        self.render_with_stride(frame, stride);
        self.scan_level += 1;
    }

    fn render_full(&self, frame: &mut [u8]) {
        let width = self.width as usize;
        let height = self.height as usize;
        let chunk_size = (width * height / rayon::current_num_threads()).max(1);

        frame
            .par_chunks_exact_mut(4 * chunk_size)
            .enumerate()
            .for_each(|(chunk_index, chunk)| {
                let start = chunk_index * chunk_size;
                let end = (start + chunk_size).min(width * height);

                for index in start..end {
                    let x = index % width;
                    let y = index / width;

                    let real =
                        self.center_x + (x as f64 - width as f64 / 2.0) * self.scale / width as f64;
                    let imag = self.center_y
                        + (y as f64 - height as f64 / 2.0) * self.scale / height as f64;

                    let iterations = mandelbrot(real, imag, self.max_iterations);
                    let color = self.get_color(iterations);

                    let pixel_index = (index - start) * 4;
                    chunk[pixel_index..pixel_index + 4]
                        .copy_from_slice(&[color[0], color[1], color[2], 255]);
                }
            });
    }

    fn render_with_stride(&self, frame: &mut [u8], stride: u32) {
        let width = self.width as usize;
        let height = self.height as usize;
        let chunk_size = (width * height / rayon::current_num_threads()).max(1);

        frame
            .par_chunks_exact_mut(4 * chunk_size)
            .enumerate()
            .for_each(|(chunk_index, chunk)| {
                let start = chunk_index * chunk_size;
                let end = (start + chunk_size).min(width * height);

                for index in start..end {
                    let x = index % width;
                    let y = index / width;

                    if (x % stride as usize == 0) && (y % stride as usize == 0) {
                        let real = self.center_x
                            + (x as f64 - width as f64 / 2.0) * self.scale / width as f64;
                        let imag = self.center_y
                            + (y as f64 - height as f64 / 2.0) * self.scale / height as f64;

                        let iterations = mandelbrot(real, imag, self.max_iterations);
                        let color = self.get_color(iterations);

                        // Fill the block of pixels for the current stride
                        for dy in 0..stride as usize {
                            for dx in 0..stride as usize {
                                let fill_x = x + dx;
                                let fill_y = y + dy;
                                if fill_x < width && fill_y < height {
                                    let fill_index = (fill_y * width + fill_x - start) * 4;
                                    if fill_index + 3 < chunk.len() {
                                        chunk[fill_index..fill_index + 4]
                                            .copy_from_slice(&[color[0], color[1], color[2], 255]);
                                    }
                                }
                            }
                        }
                    }
                }
            });
    }

    fn get_color(&self, iterations: u32) -> [u8; 3] {
        match self.color_scheme {
            ColorScheme::Smooth => color_map(iterations, self.max_iterations, ColorMode::Smooth),
            ColorScheme::Zebra => color_map(iterations, self.max_iterations, ColorMode::Zebra),
            ColorScheme::Red => color_map(iterations, self.max_iterations, ColorMode::Red),
            ColorScheme::Blue => color_map(iterations, self.max_iterations, ColorMode::Blue),
            ColorScheme::BlackAndWhite => {
                color_map(iterations, self.max_iterations, ColorMode::BlackAndWhite)
            }
            ColorScheme::Rainbow => color_map(iterations, self.max_iterations, ColorMode::Rainbow),
            ColorScheme::Psychedelic => {
                color_map(iterations, self.max_iterations, ColorMode::Psychedelic)
            }
            ColorScheme::GreenGradient => {
                color_map(iterations, self.max_iterations, ColorMode::GreenGradient)
            }
            ColorScheme::Electric => {
                color_map(iterations, self.max_iterations, ColorMode::Electric)
            }
        }
    }

    pub fn change_color_scheme(&mut self, scheme: ColorScheme) {
        self.color_scheme = scheme;
        if self.scan_config.enabled {
            self.scan_level = 0;
        }
    }

    pub fn is_scanning(&self) -> bool {
        if !self.scan_config.enabled {
            return false;
        }
        let stride = if self.scan_level == 0 {
            self.scan_config.initial_stride
        } else {
            self.scan_config.initial_stride >> self.scan_level
        };
        stride >= 1
    }
}

pub struct RendererRunner {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    pixels: Pixels,
    renderer: Renderer,
    input: WinitInputHelper,
    args: Args,
}

impl RendererRunner {
    pub fn new() -> Result<Self, Error> {
        let event_loop = EventLoop::new();
        let input = WinitInputHelper::new();
        let window = Self::create_window(&event_loop);
        let args = Args::default();
        let pixels = Self::create_pixels(&window, &args)?;
        let renderer = Renderer::new();

        Ok(Self {
            event_loop,
            window,
            pixels,
            renderer,
            input,
            args,
        })
    }

    fn create_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        let size = LogicalSize::new(800.0, 600.0);
        WindowBuilder::new()
            .with_title("Fractal Renderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(event_loop)
            .unwrap()
    }

    fn create_pixels(window: &winit::window::Window, args: &Args) -> Result<Pixels, Error> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        Pixels::new(args.get_width(), args.get_height(), surface_texture)
    }

    pub fn with_args(mut self, args: Args) -> Self {
        // Update renderer configuration
        self.renderer.max_iterations = args.get_max_iterations();
        self.renderer.scan_config = args.get_scan_config();

        // Check if window size needs to be updated
        let current_size = self.window.inner_size();
        let new_width = args.get_width();
        let new_height = args.get_height();

        if current_size.width != new_width || current_size.height != new_height {
            // Resize the window
            self.window
                .set_inner_size(LogicalSize::new(new_width as f64, new_height as f64));

            // Recreate pixels with new dimensions
            self.pixels = Self::create_pixels(&self.window, &args)
                .expect("Failed to create pixels with new dimensions");

            self.renderer.width = new_width;
            self.renderer.height = new_height;
        }

        // Update stored args
        self.args = args;

        self
    }

    pub fn run(self) -> Result<(), Error> {
        let RendererRunner {
            event_loop,
            window,
            mut pixels,
            mut renderer,
            mut input,
            args: _
        } = self;

        // Initial render
        renderer.render(pixels.frame_mut());
        pixels.render()?;

        event_loop.run(move |event, _, control_flow| {
            input.update(&event);

            if input.key_pressed(VirtualKeyCode::Escape) {
                *control_flow = ControlFlow::Exit;
                return;
            }

            Self::handle_input(&mut renderer, &input, &mut pixels, &window);

            // Handle window events
            match event {
                Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    winit::event::WindowEvent::Resized(_) => {
                        window.request_redraw();
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    if renderer.is_scanning() {
                        renderer.render(pixels.frame_mut());
                        pixels.render().expect("pixels.render() failed");
                        // Request another redraw if still scanning
                        window.request_redraw();
                    }
                }
                Event::MainEventsCleared => {
                    // Request redraw during scanning
                    if renderer.is_scanning() {
                        window.request_redraw();
                    }
                }
                Event::LoopDestroyed => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        })
    }

    fn handle_input(
        renderer: &mut Renderer,
        input: &WinitInputHelper,
        pixels: &mut Pixels,
        window: &winit::window::Window,
    ) {
        let mut needs_update = false;

        // Handle panning
        let mut pan_x = 0.0;
        let mut pan_y = 0.0;
        if input.key_held(VirtualKeyCode::Left) {
            pan_x -= 0.05;
        }
        if input.key_held(VirtualKeyCode::Right) {
            pan_x += 0.05;
        }
        if input.key_held(VirtualKeyCode::Up) {
            pan_y -= 0.05;
        }
        if input.key_held(VirtualKeyCode::Down) {
            pan_y += 0.05;
        }

        if pan_x != 0.0 || pan_y != 0.0 {
            renderer.pan(pan_x, pan_y);
            needs_update = true;
        }

        // Handle zooming
        if input.key_held(VirtualKeyCode::PageUp) {
            renderer.zoom(0.9);
            needs_update = true;
        }
        if input.key_held(VirtualKeyCode::PageDown) {
            renderer.zoom(1.1);
            needs_update = true;
        }

        // Handle color scheme changes
        if input.key_pressed(VirtualKeyCode::Key1) {
            renderer.change_color_scheme(ColorScheme::Smooth);
            needs_update = true;
        }
        if input.key_pressed(VirtualKeyCode::Key2) {
            renderer.change_color_scheme(ColorScheme::Zebra);
            needs_update = true;
        }
        if input.key_pressed(VirtualKeyCode::Key3) {
            renderer.change_color_scheme(ColorScheme::Red);
            needs_update = true;
        }
        if input.key_pressed(VirtualKeyCode::Key4) {
            renderer.change_color_scheme(ColorScheme::Blue);
            needs_update = true;
        }
        if input.key_pressed(VirtualKeyCode::Key5) {
            renderer.change_color_scheme(ColorScheme::BlackAndWhite);
            needs_update = true;
        }
        if input.key_pressed(VirtualKeyCode::Key6) {
            renderer.change_color_scheme(ColorScheme::Rainbow);
            needs_update = true;
        }
        if input.key_pressed(VirtualKeyCode::Key7) {
            renderer.change_color_scheme(ColorScheme::Psychedelic);
            needs_update = true;
        }
        if input.key_pressed(VirtualKeyCode::Key8) {
            renderer.change_color_scheme(ColorScheme::GreenGradient);
            needs_update = true;
        }
        if input.key_pressed(VirtualKeyCode::Key9) {
            renderer.change_color_scheme(ColorScheme::Electric);
            needs_update = true;
        }

        if needs_update {
            renderer.render(pixels.frame_mut());
            pixels.render().expect("pixels.render() failed");
            window.request_redraw();
        }
    }
}
