
pub struct Canvas {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            buffer: vec![0; width * height], // allocate now
            width,
            height,
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        
    }
}
