mod imp {

    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use gtk::CompositeTemplate;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/br/com/processador/simulator-window.ui")]
    pub struct SimulatorWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for SimulatorWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "SimulatorWindow";
        type Type = super::SimulatorWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            super::super::processor_window::ProcessorWindow::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for SimulatorWindow {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for SimulatorWindow {}

    impl WindowImpl for SimulatorWindow {}

    impl ApplicationWindowImpl for SimulatorWindow {}
}

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::{gio, glib};

glib::wrapper! {
    pub struct SimulatorWindow(ObjectSubclass<imp::SimulatorWindow>)
    @extends gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl SimulatorWindow {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
