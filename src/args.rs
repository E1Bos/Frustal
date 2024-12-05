use num::Complex;

#[derive(Clone, Copy)]
pub enum ColorScheme {
    Smooth,
    Zebra,
    Red,
    Blue,
    BlackAndWhite,
    Rainbow,
    Psychedelic,
    GreenGradient,
    Electric,
}

#[derive(Clone, Copy)]
pub struct ScanConfig {
    pub enabled: bool,
    pub initial_stride: u32,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            initial_stride: 8,
        }
    }
}

pub struct Args {
    width: u32,
    height: u32,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    max_iterations: u32,
    color_scheme: ColorScheme,
    _fullscreen: bool,
    scan_config: ScanConfig,
}

#[allow(dead_code)]
impl Args {
    pub fn new(
        width: u32,
        height: u32,
        max_iterations: u32,
        fullscreen: bool,
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
        color_scheme: ColorScheme,
    ) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Width and height must be greater than 0");
        }

        if max_iterations <= 0 {
            panic!("Max iterations must be greater than 0");
        }

        Self {
            width,
            height,
            upper_left,
            lower_right,
            max_iterations,
            color_scheme,
            _fullscreen: fullscreen,
            scan_config: ScanConfig::default(),
        }
    }

    pub fn with_scan_config(mut self, enabled: bool, initial_stride: u32) -> Self {
        self.scan_config = ScanConfig {
            enabled,
            initial_stride,
        };
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Width and height must be greater than 0");
        }
        
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_max_iterations(mut self, max_iterations: u32) -> Self {
        if max_iterations <= 0 {
            panic!("Max iterations must be greater than 0");
        }

        self.max_iterations = max_iterations;
        self
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_upper_left(&self) -> Complex<f64> {
        self.upper_left
    }

    pub fn get_lower_right(&self) -> Complex<f64> {
        self.lower_right
    }

    pub fn get_max_iterations(&self) -> u32 {
        self.max_iterations
    }

    pub fn get_color_scheme(&self) -> ColorScheme {
        self.color_scheme
    }

    pub fn get_scan_config(&self) -> ScanConfig {
        self.scan_config
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            width: 800,
            height: 800,
            upper_left: Complex::new(-2.5, 1.5),
            lower_right: Complex::new(1.5, -2.5),
            max_iterations: 200,
            color_scheme: ColorScheme::Red,
            _fullscreen: false,
            scan_config: ScanConfig::default(),
        }
    }
}
