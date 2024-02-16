pub mod entry_register;
pub mod memory_view;
pub mod processor_screen;

mod imp {

    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use gtk::CompositeTemplate;

    use super::entry_register;
    use super::memory_view;
    use super::processor_screen;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/br/com/processador/processor-window.ui")]
    pub struct ProcessorWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for ProcessorWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "ProcessorWindow";
        type Type = super::ProcessorWindow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            memory_view::MemoryView::ensure_type();
            processor_screen::ProcessorScreen::ensure_type();
            entry_register::EntryRegister::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for ProcessorWindow {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for ProcessorWindow {}

    impl BoxImpl for ProcessorWindow {}
}

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
}
