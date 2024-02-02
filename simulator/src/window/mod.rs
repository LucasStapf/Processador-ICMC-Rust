mod imp;

use std::borrow::Borrow;
use std::borrow::BorrowMut;

use crate::mem_row::MemoryCellRow;
use adw::prelude::*;
use adw::Application;
use adw::{gio, glib};
use glib::Object;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::MessageType;
use isa::MemoryCell;
use log::debug;
use log::error;
use processor::errors::ProcError;

use crate::processor::{instructions::InstructionDisplay, ProcessorManager};

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

    fn number_format(&self, value: MemoryCell) -> String {
        match self.imp().data.borrow().number_fomat {
            NumberFormat::Binary => format!("{:016b}", value),
            NumberFormat::Decimal => format!("{}", value),
            NumberFormat::Hexadecimal => format!("{:#06X}", value),
        }
    }

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
        let mut i = addr.clamp(0, processor::MEMORY_SIZE - 1);

        if let Ok(p) = self.imp().data.borrow().processor_manager.processor.lock() {
            if let Some(mut cell) = self
                .imp()
                .box_memory_cells
                .first_child()
                .and_downcast::<MemoryCellRow>()
            {
                loop {
                    let float = match i {
                        n if n == p.pc() => Some("<b>PC</b>"),
                        _ => None,
                    };

                    let raw = p
                        .mem(i)
                        .expect("Foi utilizado um índice inválido para atualizar o memory-view.");

                    let inst = isa::Instruction::get_instruction(raw);
                    cell.update(i, &inst.display(i, &p), raw, float);
                    i = i.saturating_add(1).clamp(0, processor::MEMORY_SIZE - 1);

                    cell.remove_css_class("error");
                    match inst {
                        isa::Instruction::InvalidInstruction => cell.add_css_class("error"),
                        isa::Instruction::LOAD
                        | isa::Instruction::LOADN
                        | isa::Instruction::STORE
                        | isa::Instruction::JZ
                        | isa::Instruction::JC
                        | isa::Instruction::JN
                        | isa::Instruction::JMP
                        | isa::Instruction::JEQ
                        | isa::Instruction::JNE
                        | isa::Instruction::JNZ
                        | isa::Instruction::JNC
                        | isa::Instruction::JGR
                        | isa::Instruction::JLE
                        | isa::Instruction::JEG
                        | isa::Instruction::JEL
                        | isa::Instruction::JOV
                        | isa::Instruction::JNO
                        | isa::Instruction::JDZ
                        | isa::Instruction::CZ
                        | isa::Instruction::CC
                        | isa::Instruction::CN
                        | isa::Instruction::CEQ
                        | isa::Instruction::CNE
                        | isa::Instruction::CNZ
                        | isa::Instruction::CNC
                        | isa::Instruction::CGR
                        | isa::Instruction::CEG
                        | isa::Instruction::CEL
                        | isa::Instruction::COV
                        | isa::Instruction::CNO
                        | isa::Instruction::CDZ
                        | isa::Instruction::CLE
                        | isa::Instruction::CALL => {
                            if let Some(n) = cell.next_sibling().and_downcast::<MemoryCellRow>() {
                                cell = n;
                                cell.update(
                                    i,
                                    &format!("#0x{:X}", p.mem(i).unwrap()),
                                    p.mem(i).unwrap(),
                                    None,
                                );
                                i = i.saturating_add(1).clamp(0, processor::MEMORY_SIZE - 1);
                            }
                        }

                        isa::Instruction::LOADI
                        | isa::Instruction::STOREI
                        | isa::Instruction::MOV
                        | isa::Instruction::INCHAR
                        | isa::Instruction::OUTCHAR
                        | isa::Instruction::ADD
                        | isa::Instruction::SUB
                        | isa::Instruction::ADDC
                        | isa::Instruction::SUBC
                        | isa::Instruction::MUL
                        | isa::Instruction::DIV
                        | isa::Instruction::INC
                        | isa::Instruction::DEC
                        | isa::Instruction::MOD
                        | isa::Instruction::AND
                        | isa::Instruction::OR
                        | isa::Instruction::XOR
                        | isa::Instruction::NOT
                        | isa::Instruction::CMP
                        | isa::Instruction::ROTL
                        | isa::Instruction::ROTR
                        | isa::Instruction::SHIFTL0
                        | isa::Instruction::SHIFTL1
                        | isa::Instruction::SHIFTR0
                        | isa::Instruction::SHIFTR1
                        | isa::Instruction::RTS
                        | isa::Instruction::RTI
                        | isa::Instruction::POP
                        | isa::Instruction::PUSH
                        | isa::Instruction::NOP
                        | isa::Instruction::HALT
                        | isa::Instruction::SETC
                        | isa::Instruction::CLEARC
                        | isa::Instruction::BREAKP => (),

                        isa::Instruction::STOREN => {
                            if let Some(next) = cell.next_sibling().and_downcast::<MemoryCellRow>()
                            {
                                cell = next;
                                cell.update(
                                    i,
                                    &format!("#0x{:X}", p.mem(i).unwrap()),
                                    p.mem(i).unwrap(),
                                    None,
                                );
                                i = i.saturating_add(1).clamp(0, processor::MEMORY_SIZE - 1);
                            }

                            if let Some(next) = cell.next_sibling().and_downcast::<MemoryCellRow>()
                            {
                                cell = next;
                                cell.update(
                                    i,
                                    &format!("#0x{:X}", p.mem(i).unwrap()),
                                    p.mem(i).unwrap(),
                                    None,
                                );
                                i = i.saturating_add(1).clamp(0, processor::MEMORY_SIZE - 1);
                            }
                        }

                        isa::Instruction::INPUT => todo!(),
                        isa::Instruction::OUTPUT => todo!(),
                        isa::Instruction::SOUND => todo!(),
                    }

                    match cell.next_sibling().and_downcast::<MemoryCellRow>() {
                        Some(next) => cell = next,
                        None => break,
                    }
                }
            }
        } else {
            return;
        }

        self.imp().data.borrow_mut().top_index = addr.clamp(0, processor::MEMORY_SIZE - 1);
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

    pub fn show_error_dialog_processor(&self, error: ProcError) {
        match error {
            ProcError::ProcessorPanic => self.show_info(
                MessageType::Error,
                "Erro interno",
                "Alguma <i>thread</i> que utilizava o processador falhou.",
            ),
            ProcError::ChannelClosed => self.show_info(
                MessageType::Error,
                "Erro interno",
                "Não foi possível trocar dados entre as <i>threads</i> do programa.",
            ),
            ProcError::ChannelEmpty => todo!(),
            ProcError::MaximumMemoryReached => self.show_info(
                MessageType::Error,
                "Limite máximo da memória atingido",
                &format!(
                    "O registrador <b>PC</b> tentou ultrapassar o limite da memória ({}). \
                    Dica: utilize a instrução <i>HALT</i> no fim do seu programa para evitar \
                    esse problema.",
                    processor::MEMORY_SIZE - 1
                ),
            ),
            ProcError::InvalidIndex(_, _) => todo!(),
            ProcError::InvalidMemoryIndex(i) => self.show_info(
                MessageType::Error,
                "Índice inválido",
                &format!(
                    "O índice {}, utilizado para acessar a memória do processador, é inválido. \
                    Índices válidos estão entre 0 e {}.",
                    i,
                    processor::MEMORY_SIZE - 1
                ),
            ),

            ProcError::InvalidInstruction(i) => self.show_info(
                MessageType::Error,
                "Instrução inválida",
                &format!(
                    "O valor {:016b} não corresponde a nenhuma instrução válida. \
                    Verifique se o arquivo <b>.mif</b> está correto ou se o \
                    conjunto de instruções utilizado é compatível com a versão do simulador.",
                    i
                ),
            ),
            ProcError::InvalidRegister(_) => todo!(),
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
