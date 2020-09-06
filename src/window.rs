use crate::canvas::Canvas;
use minifb::{Key, Window as FBWindow, WindowOptions};

#[derive(Debug)]
pub enum WindowError {
    CreationError,
}

pub struct Window {
    window: FBWindow,
    canvas: Canvas,
}

impl Window {
    pub fn new(
        width: usize,
        height: usize,
        title: &str,
    ) -> Result<Window, Box<dyn std::error::Error>> {
        let mut fb_window = FBWindow::new(title, width, height, WindowOptions::default()).unwrap();
        // .map_err(|_| {
        //     Err(Box::new(WindowError::CreationError))
        // }).unwrap();

        // Limit to max ~60 fps update rate
        fb_window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        // let mut buffer = vec![0; width * height];

        let mut canvas = Canvas::new(width, height);
        canvas.draw_point(10, 10, 0xFF0000);

        Ok(Window {
            window: fb_window,
            canvas: canvas
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&self.canvas.buffer, self.canvas.width, self.canvas.height)
                .unwrap();
        }

        Ok(())
    }
}
