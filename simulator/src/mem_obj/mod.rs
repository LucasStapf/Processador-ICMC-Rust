mod imp;
use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct MemObject(ObjectSubclass<imp::MemObject>);
}

impl MemObject {
    pub fn new(addr: String, inst: String, raw: String) -> Self {
        Object::builder()
            .property("addr", addr)
            .property("inst", inst)
            .property("raw", raw)
            .build()
    }
}

#[derive(Default)]
pub struct MemData {
    pub addr: String,
    pub inst: String,
    pub raw: String,
}
