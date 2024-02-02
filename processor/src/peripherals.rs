#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
}

type PixelBuffer = Vec<(u8, Color)>;

pub struct Video {
    width: usize,
    height: usize,
    pub buffer: PixelBuffer,
}

impl Video {
    pub fn new(width: usize, height: usize) -> Self {
        let mut buf = PixelBuffer::with_capacity(width * height);
        for _ in 0..buf.capacity() {
            buf.push((0, Color::Black))
        }

        Self {
            width,
            height,
            buffer: buf,
        }
    }

    pub fn set_pixel(&mut self, pos: usize, ch: u8, color: Color) {
        *self.buffer.get_mut(pos).unwrap() = (ch, color);
    }
}
