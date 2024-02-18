pub mod entry_register;
pub mod memory_view;
pub mod processor_screen;

mod imp {

    use std::borrow::Borrow;
    use std::borrow::BorrowMut;
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::rc::Rc;

    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use cairo::glib::closure_local;
    use cairo::glib::PropertySet;
    use gtk::CompositeTemplate;

    use crate::processor::ProcessorManager;

    use super::entry_register;
    use super::memory_view;
    use super::processor_screen;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/br/com/processador/processor-window.ui")]
    pub struct ProcessorWindow {
        // Campos dos Registradores 0-7
        #[template_child]
        pub entry_r0: TemplateChild<entry_register::EntryRegister>,
        #[template_child]
        pub entry_r1: TemplateChild<entry_register::EntryRegister>,
        #[template_child]
        pub entry_r2: TemplateChild<entry_register::EntryRegister>,
        #[template_child]
        pub entry_r3: TemplateChild<entry_register::EntryRegister>,
        #[template_child]
        pub entry_r4: TemplateChild<entry_register::EntryRegister>,
        #[template_child]
        pub entry_r5: TemplateChild<entry_register::EntryRegister>,
        #[template_child]
        pub entry_r6: TemplateChild<entry_register::EntryRegister>,
        #[template_child]
        pub entry_r7: TemplateChild<entry_register::EntryRegister>,

        // label do flag register - uma label para cada bit.
        #[template_child]
        pub label_fr_0: TemplateChild<gtk::Label>,
        #[template_child]
        pub label_fr_1: TemplateChild<gtk::Label>,
        #[template_child]
        pub label_fr_2: TemplateChild<gtk::Label>,
        #[template_child]
        pub label_fr_3: TemplateChild<gtk::Label>,
        #[template_child]
        pub label_fr_4: TemplateChild<gtk::Label>,
        #[template_child]
        pub label_fr_5: TemplateChild<gtk::Label>,
        #[template_child]
        pub label_fr_6: TemplateChild<gtk::Label>,
        #[template_child]
        pub label_fr_7: TemplateChild<gtk::Label>,
        #[template_child]
        pub label_fr_8: TemplateChild<gtk::Label>,
        #[template_child]
        pub label_fr_9: TemplateChild<gtk::Label>,

        // Label do Program Counter
        #[template_child]
        pub label_val_pc: TemplateChild<gtk::Label>,

        // Label do Stack Pointer
        #[template_child]
        pub label_val_sp: TemplateChild<gtk::Label>,

        // Label do Instruction Register
        #[template_child]
        pub label_val_ir: TemplateChild<gtk::Label>,

        #[template_child]
        pub memory_view: TemplateChild<memory_view::MemoryView>,

        pub processor_manager: RefCell<Rc<ProcessorManager>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ProcessorWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "ProcessorWindow";
        type Type = super::ProcessorWindow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            // memory_view::MemoryView::ensure_type();
            processor_screen::ProcessorScreen::ensure_type();
            entry_register::EntryRegister::ensure_type();
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl ProcessorWindow {
        #[template_callback]
        fn button_up_clicked(&self, _button: gtk::Button) {
            let index = self.memory_view.borrow().imp().addr.get().saturating_sub(1);
            if let Ok(p) = self.processor_manager.borrow().processor.lock() {
                self.memory_view.borrow().update(index, &p);
            }
        }

        #[template_callback]
        fn button_down_clicked(&self, _button: gtk::Button) {
            let index = self
                .memory_view
                .borrow()
                .imp()
                .addr
                .get()
                .saturating_add(1)
                .clamp(0, processor::MEMORY_SIZE - 1);
            if let Ok(p) = self.processor_manager.borrow().processor.lock() {
                self.memory_view.borrow().update(index, &p);
            }
        }

        #[template_callback]
        fn search_changed(&self, entry: gtk::SearchEntry) {
            let text = entry.text().to_string();
            if !text.is_empty() {
                let re =
                    regex::RegexSet::new(&[r"^[[:digit:]]{1,6}$", r"^0[xX][[:xdigit:]]{1,4}$"])
                        .unwrap();
                match re.is_match(&text) {
                    true => {
                        entry.remove_css_class("error");
                        if let Ok(p) = self.processor_manager.borrow().processor.lock() {
                            let number = if let Ok(number) = usize::from_str_radix(&text, 10) {
                                number
                            } else if let Ok(number) = usize::from_str_radix(&text[2..], 16) {
                                number
                            } else {
                                0
                            };
                            self.memory_view.borrow().update(number, &p);
                        }
                    }
                    false => entry.add_css_class("error"),
                }
            } else {
                entry.remove_css_class("error");
            }
            // let text = entry.text().to_string();
            // match text.len() {
            //     0 => entry.remove_css_class("error"),
            //     1..=2 => {
            //         // Decimal
            //         let number = usize::from_str_radix(&text, 10);
            //         match number {
            //             Ok(n) => {
            //                 entry.remove_css_class("error");
            //                 self.obj().update_memory_view(n);
            //             }
            //             Err(_) => entry.add_css_class("error"),
            //         }
            //     }
            //     3.. => {
            //         // Decimal ou Hexadecimal
            //         match &text[0..2] {
            //             "0x" | "0X" => {
            //                 let number = usize::from_str_radix(&text[2..], 16);
            //                 match number {
            //                     Ok(n) => {
            //                         entry.remove_css_class("error");
            //                         self.obj().update_memory_view(n);
            //                     }
            //                     Err(_) => entry.add_css_class("error"),
            //                 }
            //             }
            //             _ => {
            //                 let number = usize::from_str_radix(&text, 10);
            //                 match number {
            //                     Ok(n) => {
            //                         entry.remove_css_class("error");
            //                         self.obj().update_memory_view(n);
            //                     }
            //                     Err(_) => entry.add_css_class("error"),
            //                 }
            //             }
            //         }
            //     }
            // }
        }
    }
    // Trait shared by all GObjects
    impl ObjectImpl for ProcessorWindow {
        fn constructed(&self) {
            self.parent_constructed();
            // self.memory_view
            //     .imp()
            //     .scrolled_memory_cells
            //     .connect_closure(
            //         "edge-overshot",
            //         false,
            //         closure_local!(|| log::debug!("Scroll")),
            //     );
            //
            let processor = self.processor_manager.borrow().processor.clone();

            // Scroll do Memory-View
            self.memory_view
                .imp()
                .scrolled_memory_cells
                .connect_closure(
                    "edge-overshot",
                    false,
                    closure_local!(@strong self.memory_view as memory_view =>
                    move |_window: gtk::ScrolledWindow, pos: gtk::PositionType| {
                        match pos {
                            gtk::PositionType::Top => {
                                let index = memory_view.imp().addr.get().saturating_sub(1);
                                if let Ok(p) = processor.lock() {
                                    memory_view.update(index, &p);
                                }
                            }
                            gtk::PositionType::Bottom => {
                                let index =
                                    memory_view
                                    .imp()
                                    .addr
                                    .get()
                                    .saturating_add(1)
                                    .clamp(0, processor::MEMORY_SIZE - 1);
                                if let Ok(p) = processor.lock() {
                                    memory_view.update(index, &p);
                                }
                            },
                            _ => (),
                        }
                    }),
                );

            if let Ok(mut p) = self.processor_manager.borrow().processor.lock() {
                p.set_mem(4, 0b1110010000000000);
                p.set_mem(1, 0b0111111111111111);

                self.memory_view.setup(&p);
            }

            self.obj().update_ui();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for ProcessorWindow {}

    impl BoxImpl for ProcessorWindow {}
}

use std::borrow::Borrow;

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};

glib::wrapper! {
    pub struct ProcessorWindow(ObjectSubclass<imp::ProcessorWindow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ProcessorWindow {
    pub fn build() -> Self {
        glib::Object::builder().build()
    }

    fn update_registers(&self, p: &processor::Processor) {
        let imp = self.imp();
        // Registradores 0-7
        imp.entry_r0.set_text(&p.reg(0).unwrap().to_string());
        imp.entry_r1.set_text(&p.reg(1).unwrap().to_string());
        imp.entry_r2.set_text(&p.reg(2).unwrap().to_string());
        imp.entry_r3.set_text(&p.reg(3).unwrap().to_string());
        imp.entry_r4.set_text(&p.reg(4).unwrap().to_string());
        imp.entry_r5.set_text(&p.reg(5).unwrap().to_string());
        imp.entry_r6.set_text(&p.reg(6).unwrap().to_string());
        imp.entry_r7.set_text(&p.reg(7).unwrap().to_string());

        // PC, SP e IR
        imp.label_val_pc.set_text(&format!("{:#06X}", p.pc()));
        imp.label_val_sp.set_text(&format!("{:#06X}", p.sp()));
        imp.label_val_ir.set_text(&format!("{:016b}", p.ir()));

        // FR
        imp.label_fr_0.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::GREATER)
                    .expect("Esperado o índice da flag GREATER"),
            )
            .to_string(),
        );
        imp.label_fr_1.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::LESSER)
                    .expect("Esperado o índice da flag LESSER"),
            )
            .to_string(),
        );
        imp.label_fr_2.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::EQUAL)
                    .expect("Esperado o índice da flag EQUAL"),
            )
            .to_string(),
        );
        imp.label_fr_3.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::ZERO)
                    .expect("Esperado o índice da flag ZERO"),
            )
            .to_string(),
        );
        imp.label_fr_4.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::CARRY)
                    .expect("Esperado o índice da flag CARRY"),
            )
            .to_string(),
        );
        imp.label_fr_5.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::ARITHMETIC_OVERFLOW)
                    .expect("Esperado o índice da flag ARITHMETIC_OVERFLOW"),
            )
            .to_string(),
        );
        imp.label_fr_6.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::DIV_BY_ZERO)
                    .expect("Esperado o índice da flag DIV_BY_ZERO"),
            )
            .to_string(),
        );
        imp.label_fr_7.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::STACK_OVERFLOW)
                    .expect("Esperado o índice da flag STACK_OVERFLOW"),
            )
            .to_string(),
        );
        imp.label_fr_8.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::STACK_UNDERFLOW)
                    .expect("Esperado o índice da flag STACK_UNDERFLOW"),
            )
            .to_string(),
        );
        imp.label_fr_9.set_text(
            &usize::from(
                p.fr(isa::FlagIndex::NEGATIVE)
                    .expect("Esperado o índice da flag NEGATIVE"),
            )
            .to_string(),
        );
    }

    pub fn update_ui(&self) {
        if let Ok(p) = self.imp().processor_manager.borrow().processor.lock() {
            self.update_registers(&p);
            self.imp().memory_view.update(p.pc(), &p);
        }
    }
}
