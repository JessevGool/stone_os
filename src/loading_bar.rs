use crate::vga_buffer::{Color, ColorCode, WRITER};

pub struct LoadingBar {
    width: usize,
    current: usize,
    foreground: Color,
    background: Color,
    x: usize,
    y: usize,
}

impl LoadingBar {
    pub fn new(width: usize, x: usize, y: usize, foreground: Color, background: Color) -> Self {
        LoadingBar {
            width,
            current: 0,
            foreground,
            background,
            x,
            y,
        }
    }

    pub fn update(&mut self, current: usize, total: usize) {
        let new_progress = (current * self.width) / total;
        if new_progress != self.current {
            self.current = new_progress;
            self.draw();
        }
    }

    pub fn increment(&mut self, total: usize) {
        self.update(self.current + 1, total);
    }

    pub fn clear(&self) {
        use x86_64::instructions::interrupts;
        let color = ColorCode::new(self.foreground, self.background);
        
        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            
            for row_offset in 0..=2 {
                for i in 0..self.width + 2 {
                    let col = self.x + i;
                    writer.write_at(self.y + row_offset, col, b' ', color);
                }
            }
        });
    }

    fn draw(&self) {
        use x86_64::instructions::interrupts;
        let color = ColorCode::new(self.foreground, self.background);
        
        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            
            for i in 0..self.width + 2 {
                let col = self.x + i;
                writer.write_at(self.y, col, b'-', color);
            }

            for i in 0..self.width {
                let col = self.x + i + 1;
                
                if i == 0 {
                    writer.write_at(self.y + 1, self.x, b'[', color);
                }
                
                let filled = if i < self.current { b'=' } else { b' ' };
                writer.write_at(self.y + 1, col, filled, color);
                
                if i == self.width - 1 {
                    writer.write_at(self.y + 1, self.x + self.width + 1, b']', color);
                }
            }

            for i in 0..self.width + 2 {
                let col = self.x + i;
                writer.write_at(self.y + 2, col, b'-', color);
            }
        });
    }
}