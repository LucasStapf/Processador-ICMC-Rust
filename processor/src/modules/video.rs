use std::{
    error::Error,
    fmt::Display,
    sync::{Arc, Mutex},
};

use async_channel::{Receiver, Sender};

pub const VIDEO_WIDTH: usize = 40;
pub const VIDEO_HEIGHT: usize = 30;
pub const VIDEO_BUFFER_LENGHT: usize = VIDEO_WIDTH * VIDEO_HEIGHT;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Brown,
    Olive,
    Navy,
    Purple,
    Teal,
    Silver,
    Gray,
    Lime,
    Yellow,
    Fuchsia,
    Aqua,
}

impl Color {
    pub fn rgba(&self) -> (f64, f64, f64, f64) {
        match self {
            Color::Black => (0.0, 0.0, 0.0, 1.0),
            Color::White => (1.0, 1.0, 1.0, 1.0),
            Color::Red => (1.0, 0.0, 0.0, 1.0),
            Color::Green => (0.0, 1.0, 0.0, 1.0),
            Color::Blue => (0.0, 0.0, 1.0, 1.0),
            Color::Brown => (0.647058824, 0.164705882, 0.164705882, 1.0),
            Color::Olive => (0.419607843, 0.556862745, 0.137254902, 1.0),
            Color::Navy => (0.137254902, 0.137254902, 0.556862745, 1.0),
            Color::Purple => (0.529411765, 0.121568627, 0.470588235, 1.0),
            Color::Teal => (0.000000000, 0.501960784, 0.501960784, 1.0),
            Color::Silver => (0.901960784, 0.909803922, 0.980392157, 1.0),
            Color::Gray => (0.745098039, 0.745098039, 0.745098039, 1.0),
            Color::Lime => (0.196078431, 0.803921569, 0.196078431, 1.0),
            Color::Yellow => (1.000000000, 1.000000000, 0.000000000, 1.0),
            Color::Fuchsia => (1.000000000, 0.109803922, 0.682352941, 1.0),
            Color::Aqua => (0.478431373, 0.858823529, 0.576470588, 1.0),
        }
    }

    pub fn color(n: usize) -> Option<Self> {
        match n {
            0 => Some(Self::White),
            1 => Some(Self::Brown),
            2 => Some(Self::Green),
            3 => Some(Self::Olive),
            4 => Some(Self::Navy),
            5 => Some(Self::Purple),
            6 => Some(Self::Teal),
            7 => Some(Self::Silver),
            8 => Some(Self::Gray),
            9 => Some(Self::Red),
            10 => Some(Self::Lime),
            11 => Some(Self::Yellow),
            12 => Some(Self::Blue),
            13 => Some(Self::Fuchsia),
            14 => Some(Self::Aqua),
            15 => Some(Self::Black),
            _ => None,
        }
    }
}

pub type Pixelmap = (u8, Color);
pub type FrameBuffer = Vec<Pixelmap>;

#[derive(Debug, Clone)]
pub struct VideoModule {
    /// Largura do vídeo, em pixels.
    width: usize,
    /// Altura do vídeo, em pixels.
    height: usize,
    /// Buffer de vídeo que armazena o código do *pixelmap* a ser desenhado e sua cor.
    frame_buffer: Arc<Mutex<FrameBuffer>>,

    draw_signal_recv: Receiver<()>,
    draw_signal_send: Sender<()>,
}

impl Default for VideoModule {
    /// Cria um novo módulo de vídeo, inicializando o *buffer* ([`VIDEO_WIDTH`] x [`VIDEO_HEIGHT`])
    /// com o *pixelmap* de código 0 e cor [`Color::Black`].
    fn default() -> Self {
        let mut buffer = FrameBuffer::with_capacity(VIDEO_HEIGHT * VIDEO_WIDTH);
        for _ in 0..buffer.capacity() {
            buffer.push((0, Color::Black))
        }
        let arc_buffer = Arc::new(Mutex::new(buffer));

        // let (clear_s, clear_r) = async_channel::bounded(1);
        let (draw_s, draw_r) = async_channel::bounded(1);

        Self {
            width: VIDEO_WIDTH,
            height: VIDEO_HEIGHT,
            frame_buffer: arc_buffer,
            draw_signal_recv: draw_r,
            draw_signal_send: draw_s,
        }
    }
}

impl VideoModule {
    /// Cria um novo módulo de vídeo, inicializando o *buffer* (`width` x `height`)
    /// com o *pixelmap* de código 0 e cor [`Color::Black`].
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = FrameBuffer::with_capacity(width * height);
        for _ in 0..buffer.capacity() {
            buffer.push((0, Color::Black))
        }
        let arc_buffer = Arc::new(Mutex::new(buffer));
        Self {
            frame_buffer: arc_buffer,
            width,
            height,
            ..Default::default()
        }
    }

    pub fn frame_buffer(&self) -> Arc<Mutex<FrameBuffer>> {
        self.frame_buffer.clone()
    }

    pub fn draw_signal(&self) -> Receiver<()> {
        self.draw_signal_recv.clone()
    }

    fn send_draw_signal(&self) {
        match self.draw_signal_send.send_blocking(()) {
            Ok(_) => (),
            Err(e) => log::error!("Falha ao enviar o sinal draw_signal: {e}"),
        }
    }

    pub fn draw(&self, pos: usize, map: Pixelmap) {
        match self.frame_buffer.lock() {
            Ok(mut fb) => match fb.get_mut(pos) {
                Some(p) => {
                    *p = map;
                    self.send_draw_signal();
                }
                None => log::error!("Falha ao desenhar o pixel map: Posição {pos} inválida"),
            },
            Err(e) => {
                log::error!("Falha ao desenhar o desenhar o pixelmap {map:?} na posição {pos}: {e}")
            }
        }
    }
}
