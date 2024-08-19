pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            current_color: 0x000000, // Default color
        }
    }

    // Método para establecer el color actual
    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    // Método para dibujar un punto en el framebuffer
    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.buffer[idx] = self.current_color;
        }
    }

    // Método para dibujar una línea en el framebuffer
    pub fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let mut x = x0 as i32;
        let mut y = y0 as i32;
        let dx = (x1 as i32 - x0 as i32).abs();
        let dy = (y1 as i32 - y0 as i32).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.point(x as usize, y as usize);
            if x == x1 as i32 && y == y1 as i32 { break; }
            let e2 = err * 2;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    // Método para limpiar el framebuffer
    pub fn clear(&mut self) {
        self.buffer.fill(0); // Asigna color de fondo (0 en este caso)
    }

}