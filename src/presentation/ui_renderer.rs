use eframe::egui;
use std::time::Instant;

use crate::application::use_cases::HandleInputUseCase;
use crate::application::history_commands::HistoryCommandsUseCase;
use crate::presentation::texts::{ErrorMessages, SystemIndicators, PromptText, DebugMessages};

pub struct UIRenderer {
    terminal_width: f32,
    terminal_height: f32,
}

impl UIRenderer {
    pub fn new() -> Self {
        Self {
            terminal_width: 1200.0,
            terminal_height: 800.0,
        }
    }

    pub fn update_dimensions(&mut self, width: f32, height: f32) {
        self.terminal_width = width;
        self.terminal_height = height;
    }

    pub fn draw_command_history(
        &self,
        painter: &egui::Painter,
        input_handler: &HandleInputUseCase,
        history_commands: &HistoryCommandsUseCase,
    ) {
        let output_lines = input_handler.get_output_lines();
        let mut y_offset = 20.0;
        
        for line in output_lines.iter().take(30) {
            if y_offset < self.terminal_height - 120.0 {
                self.draw_output_line(painter, line, y_offset);
                y_offset += 20.0;
            }
        }
        
        self.draw_history_stats(painter, history_commands);
    }

    fn draw_output_line(&self, painter: &egui::Painter, line: &str, y_offset: f32) {
        let text_rect = egui::Rect::from_min_size(
            egui::pos2(15.0, y_offset - 2.0),
            egui::vec2(line.len() as f32 * 8.5 + 10.0, 18.0),
        );
        painter.rect_filled(
            text_rect,
            2.0,
            egui::Color32::from_rgba_premultiplied(0, 0, 0, 180),
        );
        
        let text_color = self.get_line_color(line);
        
        painter.text(
            egui::pos2(20.0, y_offset),
            egui::Align2::LEFT_TOP,
            line,
            egui::FontId::monospace(14.0),
            text_color,
        );
    }

    fn get_line_color(&self, line: &str) -> egui::Color32 {
        if line.contains(":~$ ") {
            egui::Color32::from_rgb(0, 255, 255)  // Cyan para comandos
        } else if ErrorMessages::is_error_message(line) {
            egui::Color32::from_rgb(255, 100, 100)  // Rojo para errores
        } else if line.starts_with("Bienvenido") {
            egui::Color32::from_rgb(255, 255, 0)  // Amarillo para mensajes de bienvenida
        } else {
            egui::Color32::from_rgb(0, 255, 0)  // Verde para resultados
        }
    }
    
    fn draw_history_stats(&self, painter: &egui::Painter, history_commands: &HistoryCommandsUseCase) {
        let stats = history_commands.get_history_stats();
        let stats_lines = stats.format_stats();
        
        let start_x = 20.0;
        let start_y = self.terminal_height - 150.0;
        
        for (i, line) in stats_lines.iter().enumerate() {
            let y_pos = start_y + (i as f32 * 16.0);
            
            let text_rect = egui::Rect::from_min_size(
                egui::pos2(start_x - 5.0, y_pos - 2.0),
                egui::vec2(line.len() as f32 * 7.5 + 10.0, 16.0),
            );
            painter.rect_filled(
                text_rect,
                2.0,
                egui::Color32::from_rgba_premultiplied(0, 0, 0, 200),
            );
            
            let text_color = self.get_stats_color(line);
            
            painter.text(
                egui::pos2(start_x, y_pos),
                egui::Align2::LEFT_TOP,
                line,
                egui::FontId::monospace(12.0),
                text_color,
            );
        }
    }

    fn get_stats_color(&self, line: &str) -> egui::Color32 {
        if line.contains("📊") {
            egui::Color32::from_rgb(255, 255, 0)  // Amarillo para el título
        } else if line.contains("✅") {
            egui::Color32::from_rgb(0, 255, 0)    // Verde para éxitos
        } else if line.contains("❌") {
            egui::Color32::from_rgb(255, 100, 100) // Rojo para fallos
        } else {
            egui::Color32::from_rgb(200, 200, 200) // Gris para el resto
        }
    }

    pub fn draw_prompt(
        &self,
        painter: &egui::Painter,
        input_handler: &HandleInputUseCase,
        last_input_time: Instant,
    ) {
        let command_buffer = input_handler.get_command_buffer();
        let prompt_text = PromptText::format_full_prompt(&command_buffer);
        
        let prompt_y = self.terminal_height - 60.0;
        
        let prompt_rect = egui::Rect::from_min_size(
            egui::pos2(15.0, prompt_y - 5.0),
            egui::vec2(prompt_text.len() as f32 * 9.5 + 10.0, 30.0),
        );
        painter.rect_filled(
            prompt_rect,
            3.0,
            egui::Color32::from_rgba_premultiplied(0, 0, 0, 200),
        );
        
        painter.text(
            egui::pos2(20.0, prompt_y),
            egui::Align2::LEFT_TOP,
            &prompt_text,
            egui::FontId::monospace(16.0),
            egui::Color32::from_rgb(0, 255, 0),
        );
        
        self.draw_debug_info(painter, input_handler, prompt_y);
        self.draw_cursor(painter, input_handler, prompt_y, last_input_time);
    }

    fn draw_debug_info(&self, painter: &egui::Painter, input_handler: &HandleInputUseCase, prompt_y: f32) {
        let cursor_position = input_handler.get_cursor_position();
        let command_buffer = input_handler.get_command_buffer();
        let debug_text = DebugMessages::format_buffer_debug(&command_buffer, cursor_position);
        
        painter.text(
            egui::pos2(20.0, prompt_y + 30.0),
            egui::Align2::LEFT_TOP,
            &debug_text,
            egui::FontId::monospace(12.0),
            egui::Color32::from_rgb(255, 255, 0),
        );
    }

    fn draw_cursor(
        &self,
        painter: &egui::Painter,
        input_handler: &HandleInputUseCase,
        prompt_y: f32,
        last_input_time: Instant,
    ) {
        let cursor_position = input_handler.get_cursor_position();
        let command_buffer = input_handler.get_command_buffer();
        let prompt = PromptText::get_prompt_only();
        let cursor_x = PromptText::calculate_cursor_x_with_buffer(&prompt, &command_buffer, cursor_position);
        
        let cursor_time = last_input_time.elapsed().as_secs_f32();
        if (cursor_time * 2.0) as i32 % 2 == 0 {
            painter.line_segment(
                [egui::pos2(cursor_x, prompt_y), egui::pos2(cursor_x, prompt_y + 20.0)],
                (3.0, egui::Color32::from_rgb(0, 255, 0)),
            );
        }
    }

    pub fn draw_system_indicators(&self, painter: &egui::Painter, system_stats: (f64, f64, (u64, u64))) {
        let (memory_usage, cpu_usage, network_usage) = system_stats;
        
        let memory_text = SystemIndicators::format_memory_usage(memory_usage);
        let cpu_text = SystemIndicators::format_cpu_usage(cpu_usage);
        
        let (received, transmitted) = network_usage;
        let received_mb = SystemIndicators::bytes_to_mb(received);
        let transmitted_mb = SystemIndicators::bytes_to_mb(transmitted);
        let network_text = SystemIndicators::format_network_usage(received_mb, transmitted_mb);
        
        let start_x = self.terminal_width - 400.0;
        let y_pos = 20.0;
        
        let indicator_rect = egui::Rect::from_min_size(
            egui::pos2(start_x - 10.0, y_pos - 5.0),
            egui::vec2(290.0, 30.0),
        );
        painter.rect_filled(
            indicator_rect,
            3.0,
            egui::Color32::from_rgba_premultiplied(0, 0, 0, 180),
        );
        
        let memory_color = self.get_memory_color(memory_usage);
        painter.text(
            egui::pos2(start_x, y_pos),
            egui::Align2::LEFT_TOP,
            &memory_text,
            egui::FontId::monospace(12.0),
            memory_color,
        );
        
        let cpu_color = self.get_cpu_color(cpu_usage);
        painter.text(
            egui::pos2(start_x + 100.0, y_pos),
            egui::Align2::LEFT_TOP,
            &cpu_text,
            egui::FontId::monospace(12.0),
            cpu_color,
        );
        
        let network_color = egui::Color32::from_rgb(0, 150, 255);
        painter.text(
            egui::pos2(start_x + 200.0, y_pos),
            egui::Align2::LEFT_TOP,
            &network_text,
            egui::FontId::monospace(12.0),
            network_color,
        );
    }

    fn get_memory_color(&self, memory_usage: f64) -> egui::Color32 {
        match SystemIndicators::get_memory_color(memory_usage) {
            "high" => egui::Color32::from_rgb(255, 100, 100),
            "medium" => egui::Color32::from_rgb(255, 255, 0),
            _ => egui::Color32::from_rgb(0, 255, 0),
        }
    }

    fn get_cpu_color(&self, cpu_usage: f64) -> egui::Color32 {
        match SystemIndicators::get_cpu_color(cpu_usage) {
            "high" => egui::Color32::from_rgb(255, 100, 100),
            "medium" => egui::Color32::from_rgb(255, 255, 0),
            _ => egui::Color32::from_rgb(0, 255, 0),
        }
    }
}
