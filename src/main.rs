mod args;
use args::Args;

mod fractals;
mod renderer;

use winit::error::EventLoopError;
use num::Complex;

// Example position
// upper left: -2.5690780055377322+1.4248376056487224i, 1.3964634498916704-2.5407038497806793i

fn main() -> Result<(), EventLoopError> {
    let renderer_runner = renderer::RendererRunner::new().with_args(Args::new(
        500,
        500,
        300,
        false,
        Complex::new(-2.5, 1.5),
        Complex::new(1.5, -2.5),
    ));
    renderer_runner.run()?;

    Ok(())
}
