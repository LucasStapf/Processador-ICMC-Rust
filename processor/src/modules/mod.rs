use self::video::VideoModule;

pub mod video;

#[derive(Default, Clone)]
pub struct Modules {
    pub video: VideoModule,
}
