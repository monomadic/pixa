mod window;
use window::Window;

// const CHARACTER: char = '$';
// const SIZE: f32 = 100.0;

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;

    let mut window = Window::new(WIDTH, HEIGHT, "test");

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // // Loading and rasterization
    // let font = include_bytes!("../resources/fonts/Roboto-Regular.ttf") as &[u8];
    // let settings = fontdue::FontSettings {
    //     scale: SIZE,
    //     ..fontdue::FontSettings::default()
    // };
    // let font = fontdue::Font::from_bytes(font, settings).unwrap();
    // let (metrics, bitmap) = font.rasterize(CHARACTER, SIZE);
    // // buffer.insert(0, bitmap);

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     // for (i, b) in buffer.iter_mut().enumerate() {
    //     //     *b = bitmap[i] as u32;
    //     // }

    //     for b in bitmap.iter() {

    //     }

    //     let buffer_index = (pixel_y as usize) * WIDTH + (pixel_x as usize);

    //     buffer[buffer_index] = color;

    //     // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    //     window
    //         .update_with_buffer(&buffer, WIDTH, HEIGHT)
    //         .unwrap();
    // }
}
