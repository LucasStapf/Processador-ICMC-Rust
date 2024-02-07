pub const PIXEL_SIZE: f64 = 2.0;
pub const CHAR_SIZE: f64 = PIXEL_SIZE * CHAR_SIZE_PIXELS as f64;
pub const CHAR_SIZE_PIXELS: usize = 8;
pub const CHARMAP_DEPTH: usize = 8;

pub fn draw_pixel(
    _draw: &gtk::DrawingArea,
    cairo: &cairo::Context,
    x: f64,
    y: f64,
    size: f64,
    color: processor::modules::video::Color,
) {
    let rgba = color.rgba();

    cairo.rectangle(x, y, size, size);
    cairo.set_source_rgba(rgba.0, rgba.1, rgba.2, rgba.3);
    cairo.fill().expect("Falha ao desenhar um pixel.");
}

pub fn draw_pixelmap(
    draw: &gtk::DrawingArea,
    cairo: &cairo::Context,
    char: &[u8],
    x: f64,
    y: f64,
    color: processor::modules::video::Color,
) {
    for i in 0..CHAR_SIZE_PIXELS {
        for j in 0..CHAR_SIZE_PIXELS {
            if char[i * CHAR_SIZE_PIXELS + j] == 1 {
                draw_pixel(
                    draw,
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

pub fn draw_buffer(
    draw: &gtk::DrawingArea,
    cairo: &cairo::Context,
    buf: &[(u8, processor::modules::video::Color)],
    charmap: &[u8],
) {
    for (i, (ch, color)) in buf.iter().enumerate() {
        let size = CHARMAP_DEPTH * CHARMAP_DEPTH;
        let index = *ch as usize * size;
        draw_pixelmap(
            draw,
            cairo,
            &charmap[index..index + size],
            (i % 40) as f64 * CHAR_SIZE,
            (i / 40) as f64 * CHAR_SIZE,
            *color,
        );
    }
}
