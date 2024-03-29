mod imp;
use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct MemObject(ObjectSubclass<imp::MemObject>);
}

impl MemObject {
    pub fn new(addr: String, inst: String, raw: String, float: Option<String>) -> Self {
        Object::builder()
            .property("addr", addr)
            // .property("faddr", faddr)
            .property("inst", inst)
            .property("raw", raw)
            .property("float", float)
            .build()
    }
}

#[derive(Default)]
pub struct MemData {
    pub addr: String,
    pub faddr: String,
    pub inst: String,
    pub raw: String,
    pub float: Option<String>,
    pub visible: bool,
}
