pub type UpdateLogic = fn() -> ();
pub type RenderLogic = fn() -> ();
pub type ShutdownLogic = fn() -> ();

pub struct Scheduler {
    updates_per_sec: i32,
    renders_per_sec: i32,
    update_logic: UpdateLogic,
    render_logic: RenderLogic,
    shutdown_logic: ShutdownLogic,
}

impl Scheduler {
    pub fn new(
        updates_per_sec: i32,
        update_logic: UpdateLogic,
        renders_per_sec: i32,
        render_logic: RenderLogic,
        shutdown_logic: ShutdownLogic,
    ) -> Self {
        Self {
            updates_per_sec,
            renders_per_sec,
            update_logic,
            render_logic,
            shutdown_logic,
        }
    }

    pub(crate) fn update(&mut self) {
        (self.update_logic)();
    }

    pub(crate) fn render(&mut self) {
        (self.render_logic)();
    }

    pub(crate) fn shutdown(&mut self) {
        (self.shutdown_logic)();
    }
}
