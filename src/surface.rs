use std::sync::atomic::{AtomicBool, Ordering};

use winit::{window::{Window, WindowBuilder}, event_loop::{EventLoop, ControlFlow}, dpi::LogicalSize, event::{Event, WindowEvent}};

pub struct Surface {
    window: Window,
    event_loop: EventLoop<()>,
}

impl Surface {
    pub fn create(title: &str, width: i32, height: i32) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop).unwrap();

        Self { window, event_loop }
    }

    pub fn start(self, update_logic: fn() -> (), shutdown_logic: fn() -> ()) {
        let logic_should_run = AtomicBool::new(true);

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
    
            match event {
                Event::WindowEvent { event, window_id } => match event {
                    WindowEvent::CloseRequested if window_id == self.window.id() => {
                        logic_should_run.store(false, Ordering::Relaxed);
                        shutdown_logic();
                        *control_flow = ControlFlow::Exit;
                    },
                    _ => { }
                },
                _ => {
                    if logic_should_run.load(Ordering::Relaxed) {
                        update_logic();
                    }
                },
            }
        });
    }
}