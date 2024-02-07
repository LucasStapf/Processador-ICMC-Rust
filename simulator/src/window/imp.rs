use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::ActionRow;
use gtk::glib::clone;
use gtk::DrawingArea;
use gtk::InfoBar;
use gtk::Revealer;
use gtk::{gdk, CompositeTemplate, Entry, Label, ToggleButton};
use log::{debug, error};
use std::{borrow::BorrowMut, cell::RefCell};

use crate::mem_row::MemoryCellRow;
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
    pub box_memory_cells: TemplateChild<gtk::Box>,
    #[template_child]
    pub frame_screen: TemplateChild<gtk::Frame>,

    #[template_child]
    pub toggle_mode_debug: TemplateChild<ToggleButton>,

    #[template_child]
    pub revealer_info_top: TemplateChild<Revealer>,
    #[template_child]
    pub info_bar_top: TemplateChild<InfoBar>,
    #[template_child]
    pub action_row_info: TemplateChild<ActionRow>,

    pub data: RefCell<WindowData>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "SimDebugWindow";
    type Type = super::Window;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        MemoryCellRow::ensure_type();
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
    fn mem_button_up_clicked(&self, _button: gtk::Button) {
        let index = self.data.borrow().top_index.saturating_sub(1);
        self.obj().update_memory_view(index);
    }

    #[template_callback]
    fn mem_button_down_clicked(&self, _button: gtk::Button) {
        let index = self
            .data
            .borrow()
            .top_index
            .saturating_add(1)
            .clamp(0, processor::MEMORY_SIZE - 1);
        self.obj().update_memory_view(index);
    }

    #[template_callback]
    fn restart_button_clicked(&self, _button: gtk::Button) {
        todo!("Implementar o botão de restart!");
    }

    #[template_callback]
    fn button_info_close_clicked(&self) {
        self.obj().close_info();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        // Atualiza o memory-view
        obj.update_memory_view(0);

        // Cria o processador
        let (tx, rx) = async_channel::bounded(1);
        self.data.borrow_mut().processor_manager.borrow_mut().tx = Some(tx);
        self.data.borrow_mut().processor_manager.borrow_mut().run();

        glib::spawn_future_local(clone!(@strong obj as window => async move {
            while let Ok(error) = rx.recv().await {
                match error {
                    Some(e) => window.show_error_dialog_processor(e),
                    None => (),
                }
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

        // Screen
        let screen = crate::ui::screen::Screen::new();
        screen.set_content_height(480);
        screen.set_content_width(640);
        screen.add_css_class("frame");
        screen.set_halign(gtk::Align::Center);
        screen.set_valign(gtk::Align::Center);
        screen.set_margin_bottom(8);
        screen.set_margin_start(8);
        screen.set_margin_end(8);
        self.frame_screen.set_child(Some(&screen));

        self.obj().add_controller(event_controller);
        // self.proc_screen.set_draw_func(draw_pixelmap);
    }
}

fn draw_pixelmap(draw: &DrawingArea, cairo: &cairo::Context, width: i32, height: i32) {
    cairo.rectangle(0.0, 0.0, width.into(), height.into());
    cairo.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cairo.fill().expect("Falha ao tentar escurecer a tela.");

    // draw.data("buffer").expect("Esperado buffer")
    let charmap = [
        0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    // let charmap = [true; 64].to_vec();
    let mut buf = Vec::new();
    for _ in 0..20 {
        buf.push((0, processor::modules::video::Color::White));
        buf.push((0, processor::modules::video::Color::Red));
    }
    crate::processor::video::draw_buffer(draw, cairo, &buf, &charmap);
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
