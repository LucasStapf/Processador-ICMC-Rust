use self::{control_unit::ControlUnit, video::VideoModule};

pub mod control_unit;
pub mod video;

#[derive(Default, Clone)]
pub struct Modules {
    pub video: VideoModule,
    pub control_unit: ControlUnit,
}
