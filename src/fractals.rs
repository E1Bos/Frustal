use num_complex::Complex64;

pub fn mandelbrot(real: f64, imag: f64, max_iter: u32) -> u32 {
    let c = Complex64::new(real, imag);
    let mut z = Complex64::new(0.0, 0.0);

    for iteration in 0..max_iter {
        if z.norm() > 2.0 {
            return iteration;
        }
        z = z * z + c;
    }

    max_iter
}

#[derive(Clone, Copy)]
pub enum ColorMode {
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

pub fn color_map(iterations: u32, max_iterations: u32, mode: ColorMode) -> [u8; 3] {
    if iterations == max_iterations {
        // Black for points inside the set
        return [0, 0, 0]; 
    }

    let normalized_iter = iterations as f64 / max_iterations as f64;

    match mode {
        ColorMode::Smooth => {
            // Original coloring
            let log_zn = (iterations as f64 * 1.0).log2();
            let nu = log_zn / (max_iterations as f64).log2();

            let t = nu.fract();
            let r = ((1.0 - t) * 9.0 + t * 15.0) as u8;
            let g = ((1.0 - t) * 0.0 + t * 7.0) as u8;
            let b = ((1.0 - t) * 255.0 + t * 100.0) as u8;

            [r, g, b]
        }
        ColorMode::Zebra => {
            // Zebra stripes
            let stripe_width = max_iterations as f64 / 10.0;
            let stripe_index = (iterations as f64 / stripe_width).floor() as u32;

            if stripe_index % 2 == 0 {
                [255, 255, 255]
            } else {
                [0, 0, 0]
            }
        }
        ColorMode::Red => {
            // Red gradient
            let red = (normalized_iter * 255.0) as u8;
            [red, 0, 0]
        }
        ColorMode::Blue => {
            // Blue gradient
            let blue = (normalized_iter * 255.0) as u8;
            [0, 0, blue]
        }
        ColorMode::BlackAndWhite => {
            // Grayscale gradient
            let intensity = (normalized_iter * 255.0) as u8;
            [intensity, intensity, intensity]
        }
        ColorMode::Rainbow => {
            // Rainbow gradient
            let hue = normalized_iter * 6.0;
            let r = if hue < 1.0 {
                (hue * 255.0).floor() as u8
            } else if hue < 2.0 {
                (255.0 - ((hue - 1.0) * 255.0)).floor() as u8
            } else if hue < 3.0 {
                0
            } else if hue < 4.0 {
                ((hue - 3.0) * 255.0).floor() as u8
            } else if hue < 5.0 {
                (255.0 - ((hue - 4.0) * 255.0)).floor() as u8
            } else {
                0
            };
            let g = if hue < 1.0 {
                (255.0 - (hue * 255.0)).floor() as u8
            } else if hue < 2.0 {
                255
            } else if hue < 3.0 {
                (255.0 - ((hue - 2.0) * 255.0)).floor() as u8
            } else if hue < 4.0 {
                0
            } else if hue < 5.0 {
                ((hue - 4.0) * 255.0).floor() as u8
            } else {
                255
            };
            let b = if hue < 1.0 {
                0
            } else if hue < 2.0 {
                ((hue - 1.0) * 255.0).floor() as u8
            } else if hue < 3.0 {
                255
            } else if hue < 4.0 {
                (255.0 - ((hue - 3.0) * 255.0)).floor() as u8
            } else if hue < 5.0 {
                0
            } else {
                ((hue - 5.0) * 255.0).floor() as u8
            };

            [r, g, b]
        }
        ColorMode::Psychedelic => {
            // Psychedelic gradient
            // TODO rename this color
            let r = ((normalized_iter * 255.0 * 3.0) as f64 % 256.0).floor() as u8;
            let g = ((normalized_iter * 255.0 * 5.0) as f64 % 256.0).floor() as u8;
            let b = ((normalized_iter * 255.0 * 7.0) as f64 % 256.0).floor() as u8;

            [r, g, b]
        }
        ColorMode::GreenGradient => {
            // Green gradient
            let green = (normalized_iter * 255.0) as u8;
            [0, green, 0]
        }
        ColorMode::Electric => {
            // Electric gradient
            // TODO also rename this color
            let r = ((normalized_iter * 255.0 * 2.0) as f64 % 256.0).floor() as u8;
            let g = ((normalized_iter * 255.0 * 3.0) as f64 % 256.0).floor() as u8;
            let b = ((normalized_iter * 255.0 * 5.0) as f64 % 256.0).floor() as u8;

            [r, g, b]
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let real = 0.0;
        let imag = 0.0;
        let result = mandelbrot(real, imag, 100);
        assert_eq!(result, 100);

        let real = 1.0;
        let imag = 1.0;
        let result = mandelbrot(real, imag, 100);
        assert!(result < 100);
    }
}
