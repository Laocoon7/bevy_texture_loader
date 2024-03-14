use bevy::{
    ecs::schedule::{ScheduleLabel, SystemSet},
    prelude::*,
};

/// The [`SystemSet`] used by [`TextureLoaderSchedule`]
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureLoaderSet {
    /// The step where a [`Handle<Texture>`] is converted into a proper [`Handle<Image>`] and possible [`TextureAtlas`]
    Convert,
}

/// The [`Schedule`] where a [`Handle<Texture>`] is converted into a proper [`Handle<Image>`] and possible [`TextureAtlas`]
/// 
/// This schedule is inserted after [`Update`]
#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextureLoaderSchedule;

impl TextureLoaderSchedule {
    pub(crate) fn base_schedule() -> Schedule {
        let mut schedule = Schedule::new(Self);

        schedule.configure_sets((TextureLoaderSet::Convert).chain());

        schedule
    }
}
