use std::cell::RefCell;

use adw::glib;
use adw::subclass::prelude::*;
use gtk::{
    gdk, gio,
    glib::clone,
    prelude::{EditableExt, WidgetExt},
    CompositeTemplate, Entry, Label, ListView,
};
use log::error;

use crate::processor::{ProcessadorICMC, RunMode};

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
    pub mem: RefCell<Option<gio::ListStore>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "SimDebugWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_memory();
        obj.setup_factory();

        for _ in 0..100 {
            obj.new_memory(
                "0x0000".to_string(),
                "NOP".to_string(),
                "0000000000000000".to_string(),
            );
        }

        let (tx_1, rx_1) = async_channel::bounded(1);
        let (tx_2, rx_2) = async_channel::bounded(1);

        ProcessadorICMC::new(tx_1, rx_2).run();

        glib::spawn_future_local(clone!(
                @strong self.entry_r0 as val_r0,
                @strong self.entry_r1 as val_r1,
                @strong self.entry_r2 as val_r2,
                @strong self.entry_r3 as val_r3,
                @strong self.entry_r4 as val_r4,
                @strong self.entry_r5 as val_r5,
                @strong self.entry_r6 as val_r6,
                @strong self.entry_r7 as val_r7,
                @strong self.label_fr_0 as val_fr_0,
                @strong self.label_val_pc as val_pc,
                @strong self.label_val_sp as val_sp,
                @strong self.label_val_ir as val_ir => async move {
                    loop {
                        match rx_1.recv().await {
                            Ok((r0, r1, r2, r3, r4, r5, r6, r7, pc, sp, ir)) => {
                                val_r0.set_text(&r0.to_string());
                                val_r1.set_text(&r1.to_string());
                                val_r2.set_text(&r2.to_string());
                                val_r3.set_text(&r3.to_string());
                                val_r4.set_text(&r4.to_string());
                                val_r5.set_text(&r5.to_string());
                                val_r6.set_text(&r6.to_string());
                                val_r7.set_text(&r7.to_string());
                                val_pc.set_text(&format!("0x{:04x}", pc));
                                val_sp.set_text(&format!("0x{:04x}", sp));
                                val_ir.set_text(&format!("{:016b}", ir));
                            },
                            Err(e) => {
                                error!("{e}");
                                break;
                            },
                        }
                    }
        }));

        let event_controller = gtk::EventControllerKey::new();
        event_controller.connect_key_pressed(move |_, key, _, _| {
            match key {
                gdk::Key::Page_Up => {
                    let _ = tx_2
                        .send_blocking(RunMode::Run)
                        .map_err(|e| error!("[Sending RunMode] {e}"));
                }
                gdk::Key::Page_Down => {
                    let _ = tx_2
                        .send_blocking(RunMode::Debug(true))
                        .map_err(|e| error!("[Sending RunMode] {e}"));
                }
                _ => (),
            }
            glib::Propagation::Proceed
        });

        self.obj().add_controller(event_controller);
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
