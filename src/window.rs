use minifb::{Key, Window as FBWindow, WindowOptions};
#[derive(Debug)]
pub enum WindowError {
    CreationError
}

pub struct Window {
    window: FBWindow,
}

impl Window {
    pub fn new(width: usize, height: usize, title: &str) -> Result<Window, Box<dyn std::error::Error>> {
        let mut fb_window = FBWindow::new(
            title,
            width,
            height,
            WindowOptions::default(),
        ).unwrap();
        // .map_err(|_| {
        //     Err(Box::new(WindowError::CreationError))
        // }).unwrap();


        // Limit to max ~60 fps update rate
        fb_window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut buffer = vec![0; width * height];

        while fb_window.is_open() && !fb_window.is_key_down(Key::Escape) {
            fb_window
                .update_with_buffer(&buffer, width, height)
                .unwrap();
    //         .update_with_buffer(&buffer, WIDTH, HEIGHT)
    //         .unwrap();
        }

        Ok(Window {
            window: fb_window
        })
    }

    // pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        

        
    // }
}