use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub type UpdateLogic = fn() -> ();
pub type RenderLogic = fn() -> ();
pub type ShutdownLogic = fn() -> ();

pub struct Scheduler {
    update_delay: Duration,
    render_delay: Duration,

    update_logic: UpdateLogic,
    render_logic: RenderLogic,
    shutdown_logic: ShutdownLogic,

    u_current_time: Duration,
    u_last_time: Duration,
    delta_time: f32,

    r_current_time: Duration,
    r_last_time: Duration,
}

impl Scheduler {
    pub fn new(
        updates_per_sec: i32,
        update_logic: UpdateLogic,
        renders_per_sec: i32,
        render_logic: RenderLogic,
        shutdown_logic: ShutdownLogic,
    ) -> Self {
        let updates = Scheduler::calc_per_sec(updates_per_sec);
        let renders = Scheduler::calc_per_sec(renders_per_sec);

        Self {
            update_delay: updates,
            render_delay: renders,
            update_logic,
            render_logic,
            shutdown_logic,
            u_current_time: Duration::ZERO,
            u_last_time: Duration::ZERO,
            delta_time: 0.0,
            r_current_time: Duration::ZERO,
            r_last_time: Duration::ZERO,
        }
    }

    pub(crate) fn update(&mut self) {
        self.u_current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let diff = self.u_current_time - self.u_last_time;
        self.delta_time = diff.as_secs_f32();

        if self.update_delay > diff {
            return;
        }
        self.u_last_time = self.u_current_time;

        (self.update_logic)();
    }

    pub(crate) fn render(&mut self) {
        self.r_current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let diff = self.r_current_time - self.r_last_time;

        if self.render_delay > diff {
            return;
        }
        self.r_last_time = self.r_current_time;

        (self.render_logic)();
    }

    pub(crate) fn shutdown(&mut self) {
        (self.shutdown_logic)();
    }

    fn calc_per_sec(per_sec: i32) -> Duration {
        Duration::from_millis(1000u64 / per_sec as u64)
    }
}
