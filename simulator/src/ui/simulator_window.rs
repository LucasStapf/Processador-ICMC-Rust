mod imp {

    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use gtk::CompositeTemplate;
    use processor::errors::ProcessorError;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/br/com/processador/simulator-window.ui")]
    pub struct SimulatorWindow {
        #[template_child]
        pub revealer_info_top: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub info_bar_top: TemplateChild<gtk::InfoBar>,
        #[template_child]
        pub action_row_info: TemplateChild<adw::ActionRow>,

        #[template_child]
        pub stack_home_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub stack_processor_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub stack_asm_page: TemplateChild<adw::ViewStackPage>,
        #[template_child]
        pub stack_charmap_page: TemplateChild<adw::ViewStackPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimulatorWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "SimulatorWindow";
        type Type = super::SimulatorWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            super::super::processor_window::ProcessorWindow::ensure_type();
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl SimulatorWindow {
        #[template_callback]
        fn button_info_close_clicked(&self) {
            self.obj().needs_attention(super::Page::Processor, false);
            self.obj().close_info();
        }
    }
    // Trait shared by all GObjects
    impl ObjectImpl for SimulatorWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj()
                .show_error_dialog_processor(ProcessorError::StackOverflow(40000));
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
use gtk::MessageType;
use processor::errors::ProcessorError;

glib::wrapper! {
    pub struct SimulatorWindow(ObjectSubclass<imp::SimulatorWindow>)
    @extends gtk::Window, gtk::Widget,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

pub enum Page {
    Home,
    Processor,
    AsmEditor,
    CharmapEditor,
}

impl SimulatorWindow {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn needs_attention(&self, page: Page, needs_attention: bool) {
        match page {
            Page::Home => self
                .imp()
                .stack_home_page
                .set_needs_attention(needs_attention),
            Page::Processor => self
                .imp()
                .stack_processor_page
                .set_needs_attention(needs_attention),
            Page::AsmEditor => self
                .imp()
                .stack_asm_page
                .set_needs_attention(needs_attention),
            Page::CharmapEditor => self
                .imp()
                .stack_charmap_page
                .set_needs_attention(needs_attention),
        }
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
        self.needs_attention(Page::Processor, true);
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
