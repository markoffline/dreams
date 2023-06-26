use winit::dpi::{LogicalSize, LogicalPosition};
use winit::window::{WindowBuilder, Window};
use winit::event_loop::{EventLoop, ControlFlow, EventLoopWindowTarget};
use winit::event::{Event, WindowEvent};
use winit::monitor::MonitorHandle;

fn main() {
    println!("Hello, world!");

    App::run();
}


struct App {
}

impl App {
    pub fn run() {
        let event_loop = EventLoop::new();

        let primary_monitor = event_loop.primary_monitor().unwrap();
        let monitor_size = primary_monitor.size();


        let window_width = 800;
        let window_height = 600;
        let window_x = (monitor_size.width - window_width) / 2;
        let window_y = (monitor_size.height - window_height) / 2;
        
        let window_builder = WindowBuilder::new()
            .with_title("DREAMS with Mark's Offline")
            .with_inner_size(LogicalSize::new(window_width, window_height)) // Placeholder...
            .with_position(LogicalPosition::new(window_x, window_y));

        let window = window_builder
            .build(&event_loop).expect("Failed to create a window!");


        let mut destroying = false;
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                // Render a frame
                Event::MainEventsCleared if !destroying => {} // Nothing right now

                // Destroy everything
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    destroying = true;
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        });
    }
}
