use num::Complex;

// Mandelbrot escape-time algorithm
pub fn mandelbrot(c: Complex<f64>, max_iter: u32) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    let mut iter = 0;
    while iter < max_iter && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        iter += 1;
    }
    iter
}

pub fn mandelbrot_dynamic_zoom() {
    todo!("mandelbrot_dynamic_zoom");
}

