use etheryal_window_clipboard::Clipboard;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    let clipboard = Clipboard::connect(&window).expect("Connect to clipboard");

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            println!("{:?}", clipboard.read());
        },
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => *control_flow = ControlFlow::Exit,
        _ => *control_flow = ControlFlow::Wait,
    });
}
