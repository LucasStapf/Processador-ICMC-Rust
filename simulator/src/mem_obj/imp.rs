use super::MemData;
use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::Properties;
use gtk::glib::subclass::object::ObjectImpl;
use gtk::glib::subclass::types::ObjectSubclass;
use std::cell::RefCell;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::MemObject)]
pub struct MemObject {
    #[property(name = "addr", get, set, type = String, member = addr)]
    #[property(name = "faddr", get, set, type = String, member = faddr)]
    #[property(name = "inst", get, set, type = String, member = inst)]
    #[property(name = "raw", get, set, type = String, member = raw)]
    #[property(name = "float", get, set, type = Option<String>, member = float)]
    #[property(name = "visible", get, set, type = bool, member = visible)]
    pub data: RefCell<MemData>,
}

#[glib::object_subclass]
impl ObjectSubclass for MemObject {
    const NAME: &'static str = "MemObj";
    type Type = super::MemObject;
}

#[glib::derived_properties]
impl ObjectImpl for MemObject {}
