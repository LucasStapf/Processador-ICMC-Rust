mod imp;
use crate::mem_obj::MemObject;
use adw::glib;
use adw::prelude::*;
use glib::Object;
use gtk::glib::subclass::types::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct MemoryCellRow(ObjectSubclass<imp::MemoryCellRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MemoryCellRow {
    fn build() -> Self {
        Object::builder().build()
    }
    pub fn new(addr: usize, inst: &str, raw: usize, float: Option<&str>) -> Self {
        let mem = MemoryCellRow::build();
        mem.imp()
            .label_mem_addr
            .set_markup(&format!("{:#06X}", addr));
        mem.imp().label_mem_inst.set_markup(inst);
        mem.imp().label_mem_raw.set_markup(&format!("{:016b}", raw));
        if let Some(s) = float {
            mem.imp().label_mem_float.set_markup(s);
            mem.imp().label_mem_float.set_visible(true);
        }
        mem
    }

    pub fn update(&self, addr: usize, inst: &str, raw: usize, float: Option<&str>) {
        self.imp()
            .label_mem_addr
            .set_markup(&format!("{:#06X}", addr));
        self.imp().label_mem_inst.set_markup(inst);
        self.imp()
            .label_mem_raw
            .set_markup(&format!("{:016b}", raw));

        match float {
            Some(s) => {
                self.imp().label_mem_float.set_markup(s);
                self.imp().label_mem_float.set_visible(true);
            }
            None => {
                self.imp().label_mem_float.set_text("");
                self.imp().label_mem_float.set_visible(false);
            }
        }
    }
}
