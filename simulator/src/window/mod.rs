mod imp;

use adw::prelude::*;
use adw::Application;
use adw::{gio, glib};
use glib::Object;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::{ListItem, NoSelection, SignalListItemFactory};
use log::error;
use std::ops::RangeInclusive;

use crate::mem_obj::MemObject;
use crate::mem_row::MemRow;
use crate::processor::ProcessorManager;

const SCROLL_MEMORY_ADD: usize = 100;

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

    pub fn update_ui(&self) {
        let mut pc = 0;
        match self.imp().data.borrow().processor_manager.processor.lock() {
            Ok(p) => {
                pc = p.pc();
                // Registradores 0-7
                self.imp().entry_r0.set_text(&p.reg(0).unwrap().to_string());
                self.imp().entry_r1.set_text(&p.reg(1).unwrap().to_string());
                self.imp().entry_r2.set_text(&p.reg(2).unwrap().to_string());
                self.imp().entry_r3.set_text(&p.reg(3).unwrap().to_string());
                self.imp().entry_r4.set_text(&p.reg(4).unwrap().to_string());
                self.imp().entry_r5.set_text(&p.reg(5).unwrap().to_string());
                self.imp().entry_r6.set_text(&p.reg(6).unwrap().to_string());
                self.imp().entry_r7.set_text(&p.reg(7).unwrap().to_string());

                // PC, SP e IR
                self.imp()
                    .label_val_pc
                    .set_text(&format!("{:#06x}", p.pc()));
                self.imp()
                    .label_val_sp
                    .set_text(&format!("{:#06x}", p.sp()));
                self.imp()
                    .label_val_ir
                    .set_text(&format!("{:016b}", p.ir()));

                // FR
            }
            Err(e) => error!("Falha na comunicação entre UI e processador: {e}"),
        }
        self.update_memory_view(pc);
    }

    pub fn update_memory_view(&self, index: usize) {
        let mut data = self.imp().data.borrow_mut();
        let pm = data.processor_manager.clone();
        let vmr = data.view_memory_range.clone();

        match pm.processor.lock() {
            Ok(p) => {
                if !vmr.contains(&index) {
                    for i in *vmr.end()..index {
                        self.new_memory(
                            i.to_string(),
                            "".to_string(),
                            p.mem(i).unwrap().to_string(),
                            None,
                        )
                    }
                    data.view_memory_range = 0..=index;
                }
            }
            Err(e) => error!(
                "Falha na comunicação entre UI e processador para atualizar o MemoryView: {e}"
            ),
        };
    }

    fn memory(&self) -> gio::ListStore {
        self.imp()
            .mem_list
            .borrow()
            .clone()
            .expect("Não foi possível obter acessar a memória.")
    }

    fn setup_memory(&self) {
        let model = gio::ListStore::new::<MemObject>();
        self.imp().mem_list.replace(Some(model));

        let selection_model = NoSelection::new(Some(self.memory()));
        self.imp().list_view_mem.set_model(Some(&selection_model));
    }

    fn new_memory(&self, s1: String, s2: String, s3: String, s4: Option<String>) {
        // Add new task to model
        let mem = MemObject::new(s1, s2, s3, s4);
        self.memory().append(&mem);
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `TaskRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `TaskRow`
            let mem_row = MemRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&mem_row));
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

pub struct WindowData {
    pub processor_manager: ProcessorManager,
    pub view_memory_range: RangeInclusive<usize>,
}

impl Default for WindowData {
    fn default() -> Self {
        Self {
            processor_manager: Default::default(),
            view_memory_range: 0..=0,
        }
    }
}
