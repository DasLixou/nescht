use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::game::Game;

pub fn start(mut game: Game) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(game.title)
        .with_inner_size(LogicalSize::new(game.pref_width, game.pref_height))
        .build(&event_loop)
        .unwrap();

    let mut logic_should_run = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested if window_id == window.id() => {
                    logic_should_run = false;
                    game.scheduler.shutdown();
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                if logic_should_run {
                    game.scheduler.update()
                }
            }
            Event::RedrawRequested(_) => {
                if logic_should_run {
                    game.scheduler.render()
                }
            }
            _ => {}
        }
    });
}
