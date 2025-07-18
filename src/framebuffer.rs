use raylib::prelude::*;

pub struct Framebuffer {
    width: u32,
    height: u32,
    color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        Image::clear_background(&mut self.color_buffer, self.background_color);
    }


    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            Image::draw_pixel(
                &mut self.color_buffer,
                x as i32,
                y as i32,
                self.current_color,
            );
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread){
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer){
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}


