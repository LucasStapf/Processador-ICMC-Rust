mod imp {

    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use adw::subclass::window;
    use adw::ActionRow;
    use async_channel::Receiver;
    use async_channel::Sender;
    use gtk::glib::clone;
    use gtk::InfoBar;
    use gtk::Revealer;
    use gtk::{gdk, CompositeTemplate, Entry, Label, ToggleButton};
    use log::debug;
    use log::error;
    use log::info;
    use processor::errors::ProcessorError;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;
    use std::{borrow::BorrowMut, cell::RefCell};

    use crate::mem_row::MemoryCellRow;
    use crate::processor::RunMode;
    use crate::ui::screen::Screen;
    use crate::RUNTIME;

    use super::InfoType;
    use super::WindowData;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/br/com/processador/window.ui")]
    pub struct Window {
        // Campos dos Registradores 0-7
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

        // Label do Flag Register - Uma label para cada bit.
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

        // Label do Program Counter
        #[template_child]
        pub label_val_pc: TemplateChild<Label>,

        // Label do Stack Pointer
        #[template_child]
        pub label_val_sp: TemplateChild<Label>,

        // Label do Instruction Register
        #[template_child]
        pub label_val_ir: TemplateChild<Label>,

        // Box onde é mostrado os dados de alguns endereços da memória
        #[template_child]
        pub box_memory_cells: TemplateChild<gtk::Box>,

        // Frame onde a tela do processador é colocada.
        #[template_child]
        pub frame_screen: TemplateChild<gtk::Frame>,

        // Botão do Modo Debug
        #[template_child]
        pub toggle_mode_debug: TemplateChild<ToggleButton>,

        #[template_child]
        pub revealer_info_top: TemplateChild<Revealer>,
        #[template_child]
        pub info_bar_top: TemplateChild<InfoBar>,
        #[template_child]
        pub action_row_info: TemplateChild<ActionRow>,

        #[template_child]
        pub search_addr: TemplateChild<gtk::SearchEntry>,

        pub data: RefCell<WindowData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
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
        fn toggled_debug(&self, button: gtk::ToggleButton) {}

        #[template_callback]
        fn button_info_close_clicked(&self) {
            self.obj().close_info();
        }

        #[template_callback]
        fn search_changed(&self, entry: gtk::SearchEntry) {
            let text = entry.text().to_string();
            match text.len() {
                0 => entry.remove_css_class("error"),
                1..=2 => {
                    // Decimal
                    let number = usize::from_str_radix(&text, 10);
                    match number {
                        Ok(n) => {
                            entry.remove_css_class("error");
                            self.obj().update_memory_view(n);
                        }
                        Err(_) => entry.add_css_class("error"),
                    }
                }
                3.. => {
                    // Decimal ou Hexadecimal
                    match &text[0..2] {
                        "0x" | "0X" => {
                            let number = usize::from_str_radix(&text[2..], 16);
                            match number {
                                Ok(n) => {
                                    entry.remove_css_class("error");
                                    self.obj().update_memory_view(n);
                                }
                                Err(_) => entry.add_css_class("error"),
                            }
                        }
                        _ => {
                            let number = usize::from_str_radix(&text, 10);
                            match number {
                                Ok(n) => {
                                    entry.remove_css_class("error");
                                    self.obj().update_memory_view(n);
                                }
                                Err(_) => entry.add_css_class("error"),
                            }
                        }
                    }
                }
            }
        }
    }

    impl Window {
        fn processor_start(
            &self,
            tx: Sender<InfoType<ProcessorError>>,
            rx: Receiver<InfoType<ProcessorError>>,
        ) {
            self.data.borrow_mut().processor_manager.run(tx);

            let obj = self.obj();
            let pm = self
                .data
                .borrow_mut()
                .processor_manager
                .borrow_mut()
                .clone();
            glib::spawn_future_local(clone!(@strong obj as win => async move {
                while let Ok(info) = rx.recv().await {
                    match info {
                        InfoType::UpdateUI => win.update_ui(),
                        InfoType::UpdateScreen(pixel, index) => {
                            if let Some(screen) = win.imp().frame_screen.child().and_downcast::<Screen>() {
                                screen.set_pixelmap(pixel, index);
                                screen.draw();
                            }
                        }
                        InfoType::UpdateMode(mode) => {
                            if let Ok(mut m) = pm.mode.lock() {
                                *m = mode;
                            }
                        }
                        InfoType::Error(e) => {
                            win.show_error_dialog_processor(e);
                            win.update_ui()
                        }
                        InfoType::None => (),
                    }
                }
            }));
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.update_memory_view(0); // Atualiza o memory-view

            let (tx, rx) = async_channel::bounded(1);
            self.processor_start(tx.clone(), rx);

            let event_controller = gtk::EventControllerKey::new();
            event_controller.connect_key_pressed(
                clone!(@strong self.toggle_mode_debug as toggle_debug, @strong obj as win,
                    @strong tx
                    => move |_, key, _, _| {
                    match key {
                        gdk::Key::F1 => {
                            tx.
                            send_blocking(InfoType::UpdateMode(Some(RunMode::Run))).unwrap();
                            toggle_debug.set_active(false);
                        }
                        gdk::Key::F2 => {
                            tx.
                            send_blocking(InfoType::UpdateMode(Some(RunMode::Debug))).unwrap();
                            toggle_debug.set_active(true);
                        }
                        _ => (),
                    };
                    glib::Propagation::Proceed
                }),
            );
            self.obj().add_controller(event_controller);

            // Processor Screen
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
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for Window {}

    // Trait shared by all windows
    impl WindowImpl for Window {}

    // Trait shared by all application windows
    impl ApplicationWindowImpl for Window {}
}

use std::error::Error;
use std::ops::RangeBounds;

use crate::mem_row::MemoryCellRow;
use crate::processor::RunMode;
use adw::prelude::*;
use adw::Application;
use adw::{gio, glib};
use async_channel::Sender;
use glib::Object;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::MessageType;
use log::error;
use processor::errors::ProcessorError;
use processor::modules::video::Pixelmap;

use crate::processor::{instructions::InstructionDisplay, ProcessorManager};

pub enum InfoType<T> {
    UpdateUI,
    UpdateScreen(Pixelmap, usize),
    UpdateMode(Option<RunMode>),
    Error(T),
    None,
}

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

    // fn number_format(&self, value: MemoryCell) -> String {
    //     match self.imp().data.borrow().number_fomat {
    //         NumberFormat::Binary => format!("{:016b}", value),
    //         NumberFormat::Decimal => format!("{}", value),
    //         NumberFormat::Hexadecimal => format!("{:#06X}", value),
    //     }
    // }

    pub fn update_ui(&self) {
        let mut pc = 0;
        match self.imp().data.borrow().processor_manager.processor.lock() {
            Ok(p) => {
                pc = p.pc();
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
            Err(e) => error!("Falha na comunicação entre UI e processador: {e}"),
        }
        self.update_memory_view(pc);
    }

    pub fn update_memory_view(&self, addr: usize) {
        // let mut i = addr.clamp(0, processor::MEMORY_SIZE - 1);
        //
        // if let Ok(p) = self.imp().data.borrow().processor_manager.processor.lock() {
        //     if let Some(mut cell) = self
        //         .imp()
        //         .box_memory_cells
        //         .first_child()
        //         .and_downcast::<MemoryCellRow>()
        //     {
        //         loop {
        //             // Label do PC ou SP
        //             let float = match i {
        //                 n if n == p.pc() => Some("<b>PC</b>"),
        //                 n if n == p.sp() => Some("<b>SP</b>"),
        //                 _ => None,
        //             };
        //
        //             // Verifica qual área da memória será mostrada.
        //             match i {
        //                 addr if isa::memory::layout::ADDR_PROG_AND_VAR.contains(&addr) => {
        //                     let raw = p.mem(i).expect(
        //                         "Foi utilizado um índice inválido para atualizar o memory-view.",
        //                     );
        //
        //                     let inst = isa::Instruction::get_instruction(raw);
        //                     cell.update(Some(inst), i, &inst.display(i, &p), raw, float);
        //                     cell.set_float_address(&isa::memory::layout::data_area(i));
        //                     cell.set_float_instruction("Instruction");
        //                     i = i.saturating_add(1).clamp(0, processor::MEMORY_SIZE - 1);
        //                     cell.remove_css_class("error");
        //
        //                     match inst {
        //                         isa::Instruction::InvalidInstruction => cell.add_css_class("error"),
        //                         isa::Instruction::LOAD
        //                         | isa::Instruction::STORE
        //                         | isa::Instruction::JZ
        //                         | isa::Instruction::JC
        //                         | isa::Instruction::JN
        //                         | isa::Instruction::JMP
        //                         | isa::Instruction::JEQ
        //                         | isa::Instruction::JNE
        //                         | isa::Instruction::JNZ
        //                         | isa::Instruction::JNC
        //                         | isa::Instruction::JGR
        //                         | isa::Instruction::JLE
        //                         | isa::Instruction::JEG
        //                         | isa::Instruction::JEL
        //                         | isa::Instruction::JOV
        //                         | isa::Instruction::JNO
        //                         | isa::Instruction::JDZ
        //                         | isa::Instruction::CZ
        //                         | isa::Instruction::CC
        //                         | isa::Instruction::CN
        //                         | isa::Instruction::CEQ
        //                         | isa::Instruction::CNE
        //                         | isa::Instruction::CNZ
        //                         | isa::Instruction::CNC
        //                         | isa::Instruction::CGR
        //                         | isa::Instruction::CEG
        //                         | isa::Instruction::CEL
        //                         | isa::Instruction::COV
        //                         | isa::Instruction::CNO
        //                         | isa::Instruction::CDZ
        //                         | isa::Instruction::CLE
        //                         | isa::Instruction::CALL => {
        //                             if let Some(n) =
        //                                 cell.next_sibling().and_downcast::<MemoryCellRow>()
        //                             {
        //                                 cell = n;
        //                                 cell.update(
        //                                     None,
        //                                     i,
        //                                     &format!("#0x{:X}", p.mem(i).unwrap()),
        //                                     p.mem(i).unwrap(),
        //                                     None,
        //                                 );
        //                                 cell.set_float_address(&isa::memory::layout::data_area(i));
        //                                 cell.set_float_instruction("Address");
        //                                 i = i
        //                                     .saturating_add(1)
        //                                     .clamp(0, processor::MEMORY_SIZE - 1);
        //                             }
        //                         }
        //
        //                         isa::Instruction::LOADI
        //                         | isa::Instruction::STOREI
        //                         | isa::Instruction::MOV
        //                         | isa::Instruction::INCHAR
        //                         | isa::Instruction::OUTCHAR
        //                         | isa::Instruction::ADD
        //                         | isa::Instruction::SUB
        //                         | isa::Instruction::ADDC
        //                         | isa::Instruction::SUBC
        //                         | isa::Instruction::MUL
        //                         | isa::Instruction::DIV
        //                         | isa::Instruction::INC
        //                         | isa::Instruction::DEC
        //                         | isa::Instruction::MOD
        //                         | isa::Instruction::AND
        //                         | isa::Instruction::OR
        //                         | isa::Instruction::XOR
        //                         | isa::Instruction::NOT
        //                         | isa::Instruction::CMP
        //                         | isa::Instruction::ROTL
        //                         | isa::Instruction::ROTR
        //                         | isa::Instruction::SHIFTL0
        //                         | isa::Instruction::SHIFTL1
        //                         | isa::Instruction::SHIFTR0
        //                         | isa::Instruction::SHIFTR1
        //                         | isa::Instruction::RTS
        //                         | isa::Instruction::RTI
        //                         | isa::Instruction::POP
        //                         | isa::Instruction::PUSH
        //                         | isa::Instruction::NOP
        //                         | isa::Instruction::HALT
        //                         | isa::Instruction::SETC
        //                         | isa::Instruction::CLEARC
        //                         | isa::Instruction::BREAKP => (),
        //
        //                         isa::Instruction::LOADN => {
        //                             if let Some(n) =
        //                                 cell.next_sibling().and_downcast::<MemoryCellRow>()
        //                             {
        //                                 cell = n;
        //                                 cell.update(
        //                                     None,
        //                                     i,
        //                                     &format!("#0x{:X}", p.mem(i).unwrap()),
        //                                     p.mem(i).unwrap(),
        //                                     None,
        //                                 );
        //                                 cell.set_float_address(&isa::memory::layout::data_area(i));
        //                                 cell.set_float_instruction("Data");
        //                                 i = i
        //                                     .saturating_add(1)
        //                                     .clamp(0, processor::MEMORY_SIZE - 1);
        //                             }
        //                         }
        //
        //                         isa::Instruction::STOREN => {
        //                             if let Some(next) =
        //                                 cell.next_sibling().and_downcast::<MemoryCellRow>()
        //                             {
        //                                 cell = next;
        //                                 cell.update(
        //                                     None,
        //                                     i,
        //                                     &format!("#0x{:X}", p.mem(i).unwrap()),
        //                                     p.mem(i).unwrap(),
        //                                     None,
        //                                 );
        //                                 cell.set_float_address(&isa::memory::layout::data_area(i));
        //                                 cell.set_float_instruction("Address");
        //                                 i = i
        //                                     .saturating_add(1)
        //                                     .clamp(0, processor::MEMORY_SIZE - 1);
        //                             }
        //
        //                             if let Some(next) =
        //                                 cell.next_sibling().and_downcast::<MemoryCellRow>()
        //                             {
        //                                 cell = next;
        //                                 cell.update(
        //                                     None,
        //                                     i,
        //                                     &format!("#0x{:X}", p.mem(i).unwrap()),
        //                                     p.mem(i).unwrap(),
        //                                     None,
        //                                 );
        //                                 cell.set_float_address(&isa::memory::layout::data_area(i));
        //                                 cell.set_float_instruction("Data");
        //                                 i = i
        //                                     .saturating_add(1)
        //                                     .clamp(0, processor::MEMORY_SIZE - 1);
        //                             }
        //                         }
        //
        //                         isa::Instruction::INPUT => {
        //                             unimplemented!("A instrução INPUT não foi implementada!")
        //                         }
        //                         isa::Instruction::OUTPUT => {
        //                             unimplemented!("A instrução OUTPUT não foi implementada!")
        //                         }
        //
        //                         isa::Instruction::SOUND => todo!(),
        //                     }
        //                 }
        //                 _ => {
        //                     let value = p.mem(i).unwrap();
        //                     cell.update(None, i, &format!("#{}", value), value, float);
        //                     cell.set_float_address(&isa::memory::layout::data_area(i));
        //                     cell.set_float_instruction("Data");
        //                     i = i.saturating_add(1).clamp(0, processor::MEMORY_SIZE - 1);
        //                     cell.remove_css_class("error");
        //                 }
        //             }
        //
        //             match cell.next_sibling().and_downcast::<MemoryCellRow>() {
        //                 Some(next) => cell = next,
        //                 None => break,
        //             }
        //         }
        //     }
        // } else {
        //     return;
        // }
        //
        // self.imp().data.borrow_mut().top_index = addr.clamp(0, processor::MEMORY_SIZE - 1);
    }

    pub fn show_info(&self, msg_type: MessageType, title: &str, subtitle: &str) {
        self.imp().revealer_info_top.set_reveal_child(true);
        self.imp().info_bar_top.set_message_type(msg_type);
        self.imp()
            .action_row_info
            .set_title(&format!("<b>{}</b>", title));
        self.imp()
            .action_row_info
            .set_subtitle(&format!("{}", subtitle));
    }

    pub fn close_info(&self) {
        self.imp().revealer_info_top.set_reveal_child(false);
    }

    pub fn show_error_dialog_processor(&self, error: ProcessorError) {
        match error {
            ProcessorError::SegmentationFault { .. } => self.show_info(
                MessageType::Error,
                "Segmentation fault",
                "O registrador <b>PC</b> está apontando para um endereço de uma região da memória não permitida.",
            ),

            ProcessorError::StackOverflow(_) => self.show_info(
                MessageType::Error,
                "Stack Overflow",
                "O registrador <b>SP</b> está apontando para um endereço menor que o topo da pilha."
            ),

            ProcessorError::StackUnderflow(_) => self.show_info(
                MessageType::Error,
                "Stack Underflow",
                "O registrador <b>SP</b> está apontando para um endereço maior que a base da pilha."
            ),

            ProcessorError::InvalidAddress(addr) => self.show_info(
                MessageType::Error,
                "Endereço inválido",
                &format!("O endereço {:06x} não é um endereço válido.", addr)
            ),

            ProcessorError::InvalidInstruction(inst) => self.show_info(
                MessageType::Error,
                "Instrução inválida",
                &format!(
                    "O valor {:016b} não corresponde a nenhuma instrução válida. \
                    Verifique se o arquivo <b>.mif</b> está correto ou se o \
                    conjunto de instruções utilizado é compatível com a versão do simulador.",
                    inst
                ),
            ),

            ProcessorError::InvalidRegister(reg) => self.show_info(
                MessageType::Error,
                "Registrador inválido",
                &format!("O registrador {} não existe.", reg)
            ),

            ProcessorError::InvalidFlag(bit) => self.show_info(
                MessageType::Error,
                "<i>Flag</i> inválida",
                &format!("O <i>bit</i> {} não representa nenhuma <i>flag</i> do <i>Flag Register</i>.", bit)
            ),

            ProcessorError::Generic { title, description } => self.show_info(
                MessageType::Error,
                &title,
                &description
            ),
        }
    }
}

pub enum NumberFormat {
    Binary,
    Decimal,
    Hexadecimal,
}

pub struct WindowData {
    pub processor_manager: ProcessorManager,
    pub number_fomat: NumberFormat,
    pub top_index: usize,
    pub charmap: Vec<u8>,
}

impl Default for WindowData {
    fn default() -> Self {
        Self {
            processor_manager: Default::default(),
            number_fomat: NumberFormat::Decimal,
            top_index: 0,
            charmap: Vec::new(),
        }
    }
}
