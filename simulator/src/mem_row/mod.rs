mod imp;
use crate::mem_obj::MemObject;
use adw::glib;
use adw::prelude::*;
use glib::Object;
use gtk::glib::subclass::types::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct MemRow(ObjectSubclass<imp::MemRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MemRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, mem_object: &MemObject) {
        // Get state
        let label_addr = self.imp().label_mem_addr.get();
        let label_inst = self.imp().label_mem_inst.get();
        let label_raw = self.imp().label_mem_raw.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        let content_label_binding = mem_object
            .bind_property("addr", &label_addr, "label")
            .sync_create()
            .build();
        // Save binding
        bindings.push(content_label_binding);

        let content_label_binding = mem_object
            .bind_property("inst", &label_inst, "label")
            .sync_create()
            .build();
        // Save binding
        bindings.push(content_label_binding);

        let content_label_binding = mem_object
            .bind_property("raw", &label_raw, "label")
            .sync_create()
            .build();
        // Save binding
        bindings.push(content_label_binding);
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
