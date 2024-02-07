pub const PIXEL_SIZE: f64 = 2.0;
pub const CHAR_SIZE: f64 = PIXEL_SIZE * CHAR_SIZE_PIXELS as f64;
pub const CHAR_SIZE_PIXELS: usize = 8;
pub const CHARMAP_DEPTH: usize = 8;

mod imp {
    use std::{cell::RefCell, rc::Rc};

    use adw::glib;
    use adw::subclass::prelude::*;

    use gtk::{
        prelude::DrawingAreaExtManual,
        subclass::{drawing_area::DrawingAreaImpl, widget::WidgetImpl},
    };
    use log::{debug, error};
    use processor::modules::video::{Color, Pixelmap};

    use crate::files::charmap::Charmap;

    pub struct Screen {
        pub pixbuf: Rc<RefCell<Vec<Pixelmap>>>,
        pub charmap: Rc<RefCell<crate::files::charmap::Charmap>>,
    }

    impl Default for Screen {
        fn default() -> Self {
            let mut vec = Vec::with_capacity(30 * 40);

            for _ in 0..vec.capacity() {
                vec.push((0 as u8, Color::Black));
            }

            Self {
                pixbuf: Rc::new(RefCell::new(vec)),
                charmap: Rc::new(RefCell::new(Charmap::default())),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Screen {
        const NAME: &'static str = "Screen";

        type Type = super::Screen;
        type ParentType = gtk::DrawingArea;
    }

    impl ObjectImpl for Screen {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            let buf = self.pixbuf.clone();
            let charmap = self.charmap.clone();

            self.obj()
                .set_draw_func(glib::clone!(@weak obj as screen => move |_, cr, w, h| {
                        cr.rectangle(0.0, 0.0, w.into(), h.into());
                        cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
                        let _ = cr.fill().map_err(|e| error!{"Falha ao preencher a tela de preto: {e}"});
                        screen.draw_buffer(cr, &buf.borrow(), &charmap.borrow().get());
                }));
        }
    }

    impl WidgetImpl for Screen {}

    impl DrawingAreaImpl for Screen {}
}

use adw::glib;
use cairo::glib::subclass::types::ObjectSubclassIsExt;
use glib::Object;
use gtk::prelude::WidgetExt;
use log::{debug, error};

glib::wrapper! {
    pub struct Screen(ObjectSubclass<imp::Screen>)
        @extends gtk::DrawingArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl Screen {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_pixelmap(&self, pixelmap: processor::modules::video::Pixelmap, index: usize) {
        *self.imp().pixbuf.borrow_mut().get_mut(index).unwrap() = pixelmap;
    }

    fn draw_pixel(
        &self,
        cairo: &cairo::Context,
        x: f64,
        y: f64,
        size: f64,
        color: processor::modules::video::Color,
    ) {
        let rgba = color.rgba();

        cairo.rectangle(x, y, size, size);
        cairo.set_source_rgba(rgba.0, rgba.1, rgba.2, rgba.3);
        let _ = cairo.fill().map_err(|e| error!("{e}"));
    }

    fn draw_pixelmap(
        &self,
        cairo: &cairo::Context,
        char: &[u8],
        x: f64,
        y: f64,
        color: processor::modules::video::Color,
    ) {
        for i in 0..CHAR_SIZE_PIXELS {
            for j in 0..CHAR_SIZE_PIXELS {
                if char[i * CHAR_SIZE_PIXELS + j] == 1 {
                    self.draw_pixel(
                        cairo,
                        (PIXEL_SIZE * j as f64) + x,
                        (PIXEL_SIZE * i as f64) + y,
                        PIXEL_SIZE,
                        color,
                    )
                }
            }
        }
    }

    fn draw_buffer(
        &self,
        cairo: &cairo::Context,
        buf: &[(u8, processor::modules::video::Color)],
        charmap: &[u8],
    ) {
        for (i, (ch, color)) in buf.iter().enumerate() {
            let size = CHARMAP_DEPTH * CHARMAP_DEPTH;
            let index = *ch as usize * size;
            self.draw_pixelmap(
                cairo,
                &charmap[index..index + size],
                (i % 40) as f64 * CHAR_SIZE,
                (i / 40) as f64 * CHAR_SIZE,
                *color,
            );
        }
    }

    pub fn draw(&self) {
        debug!("{:?}", self.imp().pixbuf.borrow().get(10).unwrap());
        self.queue_draw();
    }
}
