pub mod game;
pub mod scheduler;
pub mod window;

pub mod prelude {
    pub use crate::game::Game;
    pub use crate::scheduler::Scheduler;
}
