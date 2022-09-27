use crate::scheduler::Scheduler;

pub struct Game<'a> {
    pub(crate) scheduler: Scheduler,
    pub(crate) title: &'a str,
    pub(crate) pref_width: i32,
    pub(crate) pref_height: i32,
}

impl<'a> Game<'a> {
    pub fn create(scheduler: Scheduler, title: &'a str, pref_width: i32, pref_height: i32) -> Self {
        Self {
            scheduler,
            title,
            pref_width,
            pref_height,
        }
    }
}
