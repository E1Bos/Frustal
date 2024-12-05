mod args;
mod fractals;
mod renderer;

use args::Args;
use renderer::RendererRunner;

fn main() -> Result<(), pixels::Error> {
    let args = Args::default()
        .with_size(800, 600)
        .with_max_iterations(200);
    let runner = RendererRunner::new()?;
    runner.with_args(args).run()?;
    Ok(())
}
