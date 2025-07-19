use raylib::prelude::*;

pub struct Framebuffer {
    width: i32,
    height: i32,
    pub color_buffer:Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width: width as i32,
            height: height as i32,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    // limpiar su buffer de colores
    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
    }

    // ponga un pixel en la pantalla, asegurese de que no se pueda salir del buffer
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        // Check bounds to ensure we don't go out of buffer
        if x < self.width as u32 && y < self.height as u32 {
            
         self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
            
        }
    }

    // setton el color de fondo
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    // setton el color
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    // guarden su framebuffer a un archivo usando un export
    pub fn render_to_file(&self, file_path: &str) {
        // Export the framebuffer to a file
        self.color_buffer.export_image(file_path);
    }

    pub fn swap_buffers(&self,
       window: &mut RaylibHandle,
       raylib_thread: &RaylibThread,) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
         
            let mut renderer = window.begin_drawing(raylib_thread);


            renderer.draw_texture(&texture,0,0,Color::WHITE);
        }
    }

    // Method to resize the framebuffer
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width as i32;
        self.height = new_height as i32;
        
        // Create a new image with the new dimensions
        self.color_buffer = Image::gen_image_color(
            new_width as i32, 
            new_height as i32, 
            self.background_color
        );
    }

    // Getter methods for width and height
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
    
    // Get the color of a pixel from the framebuffer
    pub fn get_color(&self, _x: u32, _y: u32) -> Color {
        // This is a simplified implementation that returns a fixed value
        // In a real implementation, we would need to access the pixel data
        // but that would require more complex interaction with the raylib Image API
        // For now, we're using direct grid management in our Game of Life implementation
        Color::BLACK
    }
}

