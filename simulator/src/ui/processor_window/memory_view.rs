mod imp {

    use std::borrow::Borrow;
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::process::Output;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::Mutex;

    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use gtk::template_callbacks;
    use gtk::CompositeTemplate;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/br/com/processador/memory-view.ui")]
    pub struct MemoryView {
        #[template_child]
        pub scrolled_memory_cells: TemplateChild<gtk::ScrolledWindow>,

        #[template_child]
        pub box_memory_cells: TemplateChild<gtk::Box>,

        pub memory_info: RefCell<Vec<super::MemoryValue>>,
        pub addr: Cell<isa::MemoryCell>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MemoryView {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "MemoryView";
        type Type = super::MemoryView;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            crate::mem_row::MemoryCellRow::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for MemoryView {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for MemoryView {}

    impl WindowImpl for MemoryView {}

    impl BoxImpl for MemoryView {}
}

use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};
use cairo::glib::property::PropertySet;

use crate::mem_row::MemoryCellRow;
use crate::processor::instructions::InstructionDisplay;

pub enum MemoryValue {
    Instruction,
    Address,
    Data,
}

glib::wrapper! {
    pub struct MemoryView(ObjectSubclass<imp::MemoryView>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MemoryView {
    pub fn build() -> Self {
        glib::Object::builder().build()
    }

    pub fn setup(&self, processor: &processor::Processor) {
        self.update_memory_info(processor);
        self.update(0, processor);
    }

    pub fn update_memory_info(&self, p: &processor::Processor) {
        self.imp().memory_info.borrow_mut().clear();
        let mut addr = 0;
        let mut vec = self.imp().memory_info.borrow_mut();

        loop {
            match addr {
                a if isa::memory::layout::ADDR_PROG_AND_VAR.contains(&a) => {
                    let inst = isa::Instruction::get_instruction(p.mem(a).unwrap());

                    if let Ok(inst) = inst {
                        match inst {
                            isa::Instruction::LOAD
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
                                vec.push(MemoryValue::Instruction);
                                addr += 1;
                                vec.push(MemoryValue::Address);
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
                            | isa::Instruction::BREAKP => vec.push(MemoryValue::Instruction),

                            isa::Instruction::LOADN => {
                                vec.push(MemoryValue::Instruction);
                                addr += 1;
                                vec.push(MemoryValue::Data);
                            }

                            isa::Instruction::STOREN => {
                                vec.push(MemoryValue::Instruction);
                                addr += 1;
                                vec.push(MemoryValue::Address);
                                addr += 1;
                                vec.push(MemoryValue::Data);
                            }

                            isa::Instruction::INPUT => {
                                unimplemented!("A instrução INPUT não foi implementada!")
                            }

                            isa::Instruction::OUTPUT => {
                                unimplemented!("A instrução OUTPUT não foi implementada!")
                            }

                            isa::Instruction::SOUND => {
                                unimplemented!("A instrução OUTPUT não foi implementada!")
                            }
                        }
                    } else {
                        vec.push(MemoryValue::Instruction);
                    }
                }

                a if isa::memory::layout::ADDR_STATIC_DATA.contains(&a)
                    | isa::memory::layout::ADDR_DYNAMIC_DATA.contains(&a)
                    | isa::memory::layout::ADDR_SYSTEM_CALL.contains(&a)
                    | isa::memory::layout::ADDR_GAP_TOP_STACK.contains(&a)
                    | isa::memory::layout::ADDR_STACK.contains(&a)
                    | isa::memory::layout::ADDR_GAP_BOTTOM_STACK.contains(&a)
                    | isa::memory::layout::ADDR_RX.contains(&a)
                    | isa::memory::layout::ADDR_TX.contains(&a)
                    | isa::memory::layout::ADDR_TIMER.contains(&a)
                    | isa::memory::layout::ADDR_ARGS.contains(&a)
                    | isa::memory::layout::ADDR_RETURN.contains(&a)
                    | isa::memory::layout::ADDR_INTERRUPTIONS.contains(&a) =>
                {
                    vec.push(MemoryValue::Data)
                }
                _ => break,
            }
            addr += 1;
        }
    }

    pub fn update(&self, addr: isa::MemoryCell, p: &processor::Processor) {
        let mut i = addr.clamp(0, processor::MEMORY_SIZE - 1);

        if let Some(mut cell) = self
            .imp()
            .box_memory_cells
            .first_child()
            .and_downcast::<MemoryCellRow>()
        {
            loop {
                // Label do PC ou SP
                let float = match i {
                    n if n == p.pc() => cell.set_float_raw(Some("<b>PC</b>")),
                    n if n == p.sp() => cell.set_float_raw(Some("<b>SP</b>")),
                    _ => cell.set_float_raw(None),
                };

                cell.remove_css_class("error");

                match self
                    .imp()
                    .memory_info
                    .borrow()
                    .get(i)
                    .expect("Índice inválido")
                {
                    MemoryValue::Instruction => {
                        let raw = p.mem(i).unwrap();
                        let inst = isa::Instruction::get_instruction(raw);

                        if let Ok(inst) = inst {
                            cell.update(Some(inst), i, &inst.display_row(i, &p), raw);
                            cell.set_float_instruction("Instruction");
                        } else {
                            cell.add_css_class("error");
                            cell.update(None, i, "Invalid Instruction", raw);
                            cell.set_float_instruction("Instruction");
                        }
                    }
                    MemoryValue::Address => {
                        cell.update(
                            None,
                            i,
                            &format!("#0x{:X}", p.mem(i).unwrap()),
                            p.mem(i).unwrap(),
                        );
                        cell.set_float_instruction("Address");
                    }
                    MemoryValue::Data => {
                        cell.update(
                            None,
                            i,
                            &format!("#0x{:X}", p.mem(i).unwrap()),
                            p.mem(i).unwrap(),
                        );
                        cell.set_float_instruction("Data");
                    }
                }

                cell.set_float_address(&isa::memory::layout::data_area(i));
                i = i.saturating_add(1).clamp(0, processor::MEMORY_SIZE - 1);

                match cell.next_sibling().and_downcast::<MemoryCellRow>() {
                    Some(next) => cell = next,
                    None => break,
                }
            }
        } else {
            return;
        }

        self.imp()
            .addr
            .set(addr.clamp(0, processor::MEMORY_SIZE - 1));
    }
}
