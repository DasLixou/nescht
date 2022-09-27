use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::prelude::Scheduler;

pub struct Game {
    scheduler: Scheduler,
    window: Window,
    event_loop: EventLoop<()>,
}

impl Game {
    pub fn create(scheduler: Scheduler, title: &str, width: i32, height: i32) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();

        Self {
            scheduler,
            window,
            event_loop,
        }
    }

    pub fn start(mut self) {
        let mut logic_should_run = true;

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, window_id } => match event {
                    WindowEvent::CloseRequested if window_id == self.window.id() => {
                        logic_should_run = false;
                        self.scheduler.shutdown();
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    if logic_should_run {
                        self.scheduler.update()
                    }
                }
                Event::RedrawRequested(_) => {
                    if logic_should_run {
                        self.scheduler.render()
                    }
                }
                _ => {}
            }
        });
    }
}
