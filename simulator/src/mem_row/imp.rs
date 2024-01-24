use std::cell::RefCell;

use adw::glib;
use adw::subclass::prelude::*;
use glib::Binding;
use gtk::{CompositeTemplate, Label};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/br/com/processador/mem_row.ui")]
pub struct MemRow {
    #[template_child]
    pub label_mem_addr: TemplateChild<Label>,
    #[template_child]
    pub label_mem_inst: TemplateChild<Label>,
    #[template_child]
    pub label_mem_raw: TemplateChild<Label>,
    #[template_child]
    pub label_mem_float: TemplateChild<Label>,

    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for MemRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MemRowObj";
    type Type = super::MemRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for MemRow {}

// Trait shared by all widgets
impl WidgetImpl for MemRow {}

// Trait shared by all boxes
impl BoxImpl for MemRow {}
