use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;

use crate::presentation::matrix_effects::MatrixDrop;
use crate::infrastructure::logging::log_matrix_effect_update;

pub struct MatrixManager {
    matrix_effects: Arc<Mutex<Vec<MatrixDrop>>>,
    terminal_width: f32,
    terminal_height: f32,
}

impl MatrixManager {
    pub fn new(terminal_width: f32, terminal_height: f32) -> Self {
        let matrix_effects = Arc::new(Mutex::new(Vec::new()));
        let matrix_effects_clone = Arc::clone(&matrix_effects);
        
        Self::start_matrix_thread(matrix_effects_clone, terminal_width, terminal_height);
        
        Self {
            matrix_effects,
            terminal_width,
            terminal_height,
        }
    }

    fn start_matrix_thread(
        matrix_effects: Arc<Mutex<Vec<MatrixDrop>>>,
        terminal_width: f32,
        terminal_height: f32,
    ) {
        thread::spawn(move || {
            let mut last_update = Instant::now();
            loop {
                let delta_time = last_update.elapsed().as_secs_f32();
                last_update = Instant::now();
                
                if let Ok(mut effects) = matrix_effects.lock() {
                    Self::add_new_drops(&mut effects, terminal_width, terminal_height);
                    Self::update_existing_drops(&mut effects, delta_time);
                    Self::log_matrix_updates(&effects);
                }
                
                thread::sleep(Duration::from_millis(16)); // ~60 FPS
            }
        });
    }

    fn add_new_drops(effects: &mut Vec<MatrixDrop>, terminal_width: f32, terminal_height: f32) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.3) && effects.len() < 20 {
            let x = rng.gen_range(0.0..terminal_width);
            effects.push(MatrixDrop::new(x, terminal_height));
        }
    }

    fn update_existing_drops(effects: &mut Vec<MatrixDrop>, delta_time: f32) {
        effects.retain_mut(|drop| !drop.update(delta_time));
    }

    fn log_matrix_updates(effects: &Vec<MatrixDrop>) {
        static mut FRAME_COUNT: u32 = 0;
        unsafe {
            FRAME_COUNT += 1;
            if FRAME_COUNT % 100 == 0 {
                log_matrix_effect_update(effects.len());
            }
        }
    }

    pub fn draw_matrix_effects(&self, painter: &eframe::egui::Painter, last_input_time: Instant) {
        let time_since_input = last_input_time.elapsed().as_millis();
        let alpha_factor = if time_since_input < 200 {
            (time_since_input as f32 / 200.0).min(1.0)
        } else {
            1.0
        };
        
        if let Ok(effects) = self.matrix_effects.lock() {
            for drop in effects.iter() {
                drop.draw_with_alpha(painter, alpha_factor);
            }
        }
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.terminal_width = width;
        self.terminal_height = height;
    }
}
