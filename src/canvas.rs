
pub struct Canvas {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            buffer: vec![0; width * height], // allocate now
            width,
            height,
        }
    }

    pub fn draw_point(&mut self, x: usize, y: usize, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0xFF0000;
        }
    }
}
