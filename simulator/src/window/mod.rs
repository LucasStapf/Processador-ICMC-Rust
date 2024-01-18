mod imp;

use adw::prelude::*;
use adw::Application;
use adw::{gio, glib};
use glib::Object;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::{ListItem, NoSelection, SignalListItemFactory};

use crate::mem_obj::MemObject;
use crate::mem_row::MemRow;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn memory(&self) -> gio::ListStore {
        self.imp()
            .mem
            .borrow()
            .clone()
            .expect("Não foi possível obter acessar a memória.")
    }

    fn setup_memory(&self) {
        let model = gio::ListStore::new::<MemObject>();
        self.imp().mem.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.memory()));
        self.imp().list_view_mem.set_model(Some(&selection_model));
    }

    fn new_memory(&self, s1: String, s2: String, s3: String) {
        // Add new task to model
        let mem = MemObject::new(s1, s2, s3);
        self.memory().append(&mem);
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `TaskRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `TaskRow`
            let task_row = MemRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&task_row));
        });

        // Tell factory how to bind `TaskRow` to a `TaskObject`
        factory.connect_bind(move |_, list_item| {
            // Get `TaskObject` from `ListItem`
            let task_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<MemObject>()
                .expect("The item has to be an `TaskObject`.");

            // Get `TaskRow` from `ListItem`
            let task_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<MemRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.bind(&task_object);
        });

        // Tell factory how to unbind `TaskRow` from `TaskObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `TaskRow` from `ListItem`
            let task_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<MemRow>()
                .expect("The child has to be a `TaskRow`.");

            task_row.unbind();
        });

        // Set the factory of the list view
        self.imp().list_view_mem.set_factory(Some(&factory));
    }
}
