use std::borrow::Borrow;
use std::{borrow::BorrowMut, cell::RefCell};

use adw::glib;
use adw::subclass::prelude::*;
use gtk::gdk::Cursor;
use gtk::{gdk, gio, prelude::WidgetExt, CompositeTemplate, Entry, Label, ListView, ToggleButton};
use gtk::{glib::clone, prelude::*};
use log::{debug, error};

use crate::mem_row::MemRow;
use crate::processor::RunMode;

use super::WindowData;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/br/com/processador/sim.ui")]
pub struct Window {
    #[template_child]
    pub entry_r0: TemplateChild<Entry>,
    #[template_child]
    pub entry_r1: TemplateChild<Entry>,
    #[template_child]
    pub entry_r2: TemplateChild<Entry>,
    #[template_child]
    pub entry_r3: TemplateChild<Entry>,
    #[template_child]
    pub entry_r4: TemplateChild<Entry>,
    #[template_child]
    pub entry_r5: TemplateChild<Entry>,
    #[template_child]
    pub entry_r6: TemplateChild<Entry>,
    #[template_child]
    pub entry_r7: TemplateChild<Entry>,
    #[template_child]
    pub label_fr_0: TemplateChild<Label>,
    #[template_child]
    pub label_fr_1: TemplateChild<Label>,
    #[template_child]
    pub label_fr_2: TemplateChild<Label>,
    #[template_child]
    pub label_fr_3: TemplateChild<Label>,
    #[template_child]
    pub label_fr_4: TemplateChild<Label>,
    #[template_child]
    pub label_fr_5: TemplateChild<Label>,
    #[template_child]
    pub label_fr_6: TemplateChild<Label>,
    #[template_child]
    pub label_fr_7: TemplateChild<Label>,
    #[template_child]
    pub label_fr_8: TemplateChild<Label>,
    #[template_child]
    pub label_fr_9: TemplateChild<Label>,
    #[template_child]
    pub label_val_pc: TemplateChild<Label>,
    #[template_child]
    pub label_val_sp: TemplateChild<Label>,
    #[template_child]
    pub label_val_ir: TemplateChild<Label>,

    #[template_child]
    pub list_view_mem: TemplateChild<ListView>,
    pub mem_list: RefCell<Option<gio::ListStore>>,

    pub data: RefCell<WindowData>,

    #[template_child]
    pub toggle_mode_debug: TemplateChild<ToggleButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "SimDebugWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handler_edge_reached(&self, pos: gtk::PositionType, _scroll: &gtk::ScrolledWindow) {
        if pos == gtk::PositionType::Bottom {
            let end = *self.data.borrow().view_memory_range.end();
            let index = (end + super::SCROLL_MEMORY_ADD).clamp(0, processor::MEMORY_SIZE - 1);
            self.obj().update_memory_view(index);
        }
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_memory();
        obj.setup_factory();
        obj.update_memory_view(super::SCROLL_MEMORY_ADD);

        let (tx, rx) = async_channel::bounded(1);
        self.data.borrow_mut().processor_manager.borrow_mut().tx = Some(tx);
        self.data.borrow_mut().processor_manager.borrow_mut().run();

        glib::spawn_future_local(clone!(@strong obj as window => async move {
            while let Ok(()) = rx.recv().await {
                window.update_ui();
            }
        }));

        let pm = self.data.borrow_mut().processor_manager.clone();
        let event_controller = gtk::EventControllerKey::new();
        event_controller.connect_key_pressed(
            clone!(@strong self.toggle_mode_debug as toggle_debug,
                @strong pm => move |_, key, _, _| {
                match key {
                    gdk::Key::Page_Up => {
                        match pm.mode.lock() {
                            Ok(mut m) => {
                                debug!("Modo selecionado: Run");
                                *m = RunMode::Run;
                                toggle_debug.set_active(false);
                            }
                            Err(e) => error!("Falha ao mudar o modo para Run: {e}"),
                        }
                    }
                    gdk::Key::Page_Down => {
                        match pm.mode.lock() {
                            Ok(mut m) => {
                                debug!("Modo selecionado: Debug");
                                *m = RunMode::Debug(true);
                                toggle_debug.set_active(true);
                            }
                            Err(e) => error!("Falha ao mudar o modo para Run: {e}"),
                        }
                    }
                    _ => (),
                }
                glib::Propagation::Proceed
            }),
        );

        self.obj().add_controller(event_controller);
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
