use eframe::egui;
use rand::Rng;

const MATRIX_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#$%^&*()_+-=[]{}|;:,.<>?";
const MATRIX_COLORS: [egui::Color32; 4] = [
    egui::Color32::from_rgb(0, 255, 0),      // Verde
    egui::Color32::from_rgb(0, 200, 0),      // Verde brillante
    egui::Color32::from_rgb(0, 150, 0),      // Verde oscuro
    egui::Color32::from_rgb(0, 255, 100),    // Verde fosforescente
];

#[derive(Debug, Clone)]
pub struct MatrixDrop {
    pub x: f32,
    pub y: f32,
    pub chars: Vec<char>,
    pub speed: f32,
    pub brightness: usize,
}

impl MatrixDrop {
    pub fn new(x: f32, _height: f32) -> Self {
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(5..=15);
        let chars: Vec<char> = (0..length)
            .map(|_| {
                MATRIX_CHARS
                    .chars()
                    .nth(rng.gen_range(0..MATRIX_CHARS.len()))
                    .unwrap()
            })
            .collect();

        MatrixDrop {
            x,
            y: -(length as f32 * 20.0),
            chars,
            speed: rng.gen_range(50.0..=150.0),
            brightness: rng.gen_range(1..=4),
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.y += self.speed * delta_time;
        
        // Cambiar caracteres aleatoriamente
        let mut rng = rand::thread_rng();
        for i in 0..self.chars.len() {
            if rng.gen_bool(0.1) {
                self.chars[i] = MATRIX_CHARS
                    .chars()
                    .nth(rng.gen_range(0..MATRIX_CHARS.len()))
                    .unwrap();
            }
        }
        
        // Retornar true si la gota está fuera de la pantalla
        self.y > 800.0
    }

    #[allow(dead_code)]
    pub fn draw(&self, painter: &egui::Painter) {
        self.draw_with_alpha(painter, 1.0);
    }

    pub fn draw_with_alpha(&self, painter: &egui::Painter, alpha_factor: f32) {
        for (i, &ch) in self.chars.iter().enumerate() {
            let y_pos = self.y - i as f32 * 20.0;
            if y_pos >= 0.0 && y_pos < 800.0 {
                let color_index = (self.brightness + i).min(MATRIX_COLORS.len() - 1);
                let base_color = MATRIX_COLORS[color_index];
                
                // Aplicar transparencia al color
                let color = egui::Color32::from_rgba_premultiplied(
                    base_color.r(),
                    base_color.g(),
                    base_color.b(),
                    (base_color.a() as f32 * alpha_factor) as u8,
                );
                
                painter.text(
                    egui::pos2(self.x, y_pos),
                    egui::Align2::LEFT_TOP,
                    ch.to_string(),
                    egui::FontId::monospace(16.0),
                    color,
                );
            }
        }
    }
}

pub struct MatrixEffectManager {
    pub drops: Vec<MatrixDrop>,
    pub terminal_width: u16,
    pub terminal_height: u16,
}

impl MatrixEffectManager {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        Self {
            drops: Vec::new(),
            terminal_width,
            terminal_height,
        }
    }

    pub fn add_random_drops(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.3) && self.drops.len() < 20 {
            let x = rng.gen_range(0..self.terminal_width);
            self.drops.push(MatrixDrop::new(x as f32, self.terminal_height as f32));
        }
    }

    pub fn update_drops(&mut self) {
        self.drops.retain_mut(|drop| !drop.update(1.0)); // Assuming delta_time is 1.0 for now
    }

    pub fn draw(&self, painter: &egui::Painter) -> std::io::Result<()> {
        for drop in &self.drops {
            drop.draw(painter);
        }
        Ok(())
    }
}
