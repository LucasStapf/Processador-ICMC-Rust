mod imp {

    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use gtk::CompositeTemplate;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/br/com/processador/memory-view.ui")]
    pub struct MemoryView {}

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

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};

glib::wrapper! {
    pub struct MemoryView(ObjectSubclass<imp::MemoryView>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MemoryView {
    pub fn build() -> Self {
        glib::Object::builder().build()
    }
}
