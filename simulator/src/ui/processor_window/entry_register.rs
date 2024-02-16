mod imp {

    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use gtk::CompositeTemplate;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/br/com/processador/entry-register.ui")]
    pub struct EntryRegister {}

    #[glib::object_subclass]
    impl ObjectSubclass for EntryRegister {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "EntryRegister";
        type Type = super::EntryRegister;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for EntryRegister {}

    // Trait shared by all widgets
    impl WidgetImpl for EntryRegister {}

    impl BoxImpl for EntryRegister {}
}

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};

glib::wrapper! {
    pub struct EntryRegister(ObjectSubclass<imp::EntryRegister>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl EntryRegister {
    pub fn build() -> Self {
        glib::Object::builder().build()
    }
}
