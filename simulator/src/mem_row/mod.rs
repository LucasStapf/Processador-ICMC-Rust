mod imp;
use adw::glib;
use adw::prelude::*;
use glib::Object;
use gtk::glib::subclass::types::ObjectSubclassIsExt;

glib::wrapper! {
    pub struct MemoryCellRow(ObjectSubclass<imp::MemoryCellRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MemoryCellRow {
    fn build() -> Self {
        Object::builder().build()
    }
    pub fn new(
        addr: usize,
        f_addr: &str,
        inst: &str,
        f_inst: &str,
        raw: usize,
        float: Option<&str>,
    ) -> Self {
        let mem = MemoryCellRow::build();
        mem.imp()
            .label_mem_addr
            .set_markup(&format!("{:#06X}", addr));
        mem.imp().label_mem_float_addr.set_markup(f_addr);

        mem.imp().label_mem_inst.set_markup(inst);
        mem.imp().label_mem_float_inst.set_markup(f_inst);

        mem.imp().label_mem_raw.set_markup(&format!("{:016b}", raw));
        if let Some(s) = float {
            mem.imp().label_mem_float_raw.set_markup(s);
            mem.imp().label_mem_float_raw.set_visible(true);
        }
        mem
    }

    pub fn update(
        &self,
        instruction: Option<isa::Instruction>,
        addr: usize,
        inst: &str,
        raw: usize,
    ) {
        self.imp()
            .label_mem_addr
            .set_markup(&format!("{:#06X}", addr));
        self.imp().label_mem_inst.set_markup(inst);

        match instruction {
            Some(i) => {
                let mask = i.mask();
                let raw_str = format!("{:016b}", raw);
                let result = mask
                    .chars()
                    .zip(raw_str.chars())
                    .map(|(c_m, c_r)| {
                        if c_m != '-' {
                            format!("<b>{}</b>", c_r)
                        } else {
                            c_r.to_string()
                        }
                    })
                    .collect::<String>();
                self.imp().label_mem_raw.set_markup(&result)
            }
            None => self
                .imp()
                .label_mem_raw
                .set_markup(&format!("{:016b}", raw)),
        }

        // match float {
        //     Some(s) => {
        //         self.imp().label_mem_float_raw.set_markup(s);
        //         self.imp().label_mem_float_raw.set_visible(true);
        //     }
        //     None => {
        //         self.imp().label_mem_float_raw.set_text("");
        //         self.imp().label_mem_float_raw.set_visible(false);
        //     }
        // }
    }

    pub fn set_float_address(&self, s: &str) {
        self.imp()
            .label_mem_float_addr
            .set_markup(&format!("<b>{s}</b>"))
    }

    pub fn set_float_instruction(&self, s: &str) {
        self.imp()
            .label_mem_float_inst
            .set_markup(&format!("<b>{s}</b>"))
    }

    pub fn set_float_raw(&self, s: Option<&str>) {
        match s {
            Some(s) => {
                self.imp().label_mem_float_raw.set_markup(s);
                self.imp().label_mem_float_raw.set_visible(true);
            }
            None => {
                self.imp().label_mem_float_raw.set_text("");
                self.imp().label_mem_float_raw.set_visible(false);
            }
        }
    }
}
