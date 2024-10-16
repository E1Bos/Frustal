use frustal::{App, Args};
use num::Complex;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::new(
        500,
        500,
        500,
        false,
        Complex::new(0.0.into(), 0.0.into()),
        Complex::new(0.0.into(), 0.0.into()),
    );

    let event_loop: EventLoop<()> = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new(args);
    event_loop.run_app(&mut app)?;

    Ok(())
}
