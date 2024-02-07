use self::video::VideoModule;

pub mod video;

pub struct Modules {
    pub video: VideoModule,
}

impl Modules {
    pub fn new() -> Self {
        Self {
            video: VideoModule::new(40, 30),
        }
    }
}
