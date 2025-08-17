use eframe::egui;
use std::time::Instant;

use crate::application::use_cases::{ExecuteCommandUseCase, HandleInputUseCase};
use crate::application::history_commands::{HistoryCommandsUseCase, HistoryCommandParser, HistoryCommand};
use crate::domain::repositories::CommandRepository;
use crate::presentation::texts::{ErrorMessages, CommandHistoryText};
use crate::infrastructure::logging::log_command_execution;

pub struct InputHandler<R>
where
    R: CommandRepository,
{
    input_handler: HandleInputUseCase,
    command_executor: ExecuteCommandUseCase<R>,
    history_commands: HistoryCommandsUseCase,
    last_input_time: Instant,
    input_buffer: String,
    input_buffer_dirty: bool,
}

impl<R> InputHandler<R>
where
    R: CommandRepository,
{
    pub fn new(
        input_handler: HandleInputUseCase,
        command_executor: ExecuteCommandUseCase<R>,
    ) -> Self {
        Self {
            input_handler,
            command_executor,
            history_commands: HistoryCommandsUseCase::new(100),
            last_input_time: Instant::now(),
            input_buffer: String::new(),
            input_buffer_dirty: true,
        }
    }

    pub fn handle_keyboard_input(&mut self, input: &egui::InputState) -> bool {
        let mut input_processed = false;
        
        for event in &input.events {
            match event {
                egui::Event::Key { key, pressed, .. } if *pressed => {
                    input_processed = true;
                    self.update_input_time_for_key(key);
                    
                    match key {
                        egui::Key::Enter => {
                            self.handle_enter_key();
                        }
                        egui::Key::Backspace => {
                            self.input_handler.handle_backspace();
                            self.input_buffer_dirty = true;
                        }
                        egui::Key::Escape => {
                            self.clear_input();
                        }
                        egui::Key::F4 if input.modifiers.alt => {
                            std::process::exit(0);
                        }
                        egui::Key::Tab => {
                            input_processed = true;
                        }
                        egui::Key::ArrowLeft => {
                            self.input_handler.handle_arrow_left();
                            input_processed = true;
                        }
                        egui::Key::ArrowRight => {
                            self.input_handler.handle_arrow_right();
                            input_processed = true;
                        }
                        egui::Key::ArrowUp | egui::Key::ArrowDown => {
                            input_processed = true;
                        }
                        egui::Key::Home => {
                            self.input_handler.handle_home();
                            input_processed = true;
                        }
                        egui::Key::End => {
                            self.input_handler.handle_end();
                            input_processed = true;
                        }
                        egui::Key::Delete => {
                            input_processed = true;
                        }
                        _ => {}
                    }
                }
                egui::Event::Text(text) => {
                    input_processed = true;
                    self.handle_text_input(text);
                }
                _ => {}
            }
        }
        
        input_processed
    }

    fn update_input_time_for_key(&mut self, key: &egui::Key) {
        match key {
            egui::Key::Enter | egui::Key::Backspace | egui::Key::Escape => {
                self.last_input_time = Instant::now();
            }
            _ => {}
        }
    }

    fn handle_enter_key(&mut self) {
        let command = self.input_handler.get_command_buffer();
        if !command.trim().is_empty() {
            if self.handle_history_commands(&command) {
                self.clear_input();
            } else {
                self.execute_command(&command);
            }
        }
    }

    fn handle_text_input(&mut self, text: &str) {
        for ch in text.chars() {
            if ch.is_ascii() && !ch.is_control() {
                self.input_handler.handle_key_press(ch);
                self.input_buffer.push(ch);
                self.input_buffer_dirty = true;
                self.last_input_time = Instant::now();
            }
        }
    }

    fn handle_history_commands(&mut self, command: &str) -> bool {
        let history_command = HistoryCommandParser::parse_command(command);
        
        match history_command {
            HistoryCommand::NotHistoryCommand => false,
            _ => {
                log::info!("Ejecutando comando del historial: '{}'", command);
                
                let output_lines = history_command.execute(&mut self.history_commands);
                for line in output_lines {
                    self.input_handler.add_output_line(line);
                }
                true
            }
        }
    }

    fn execute_command(&mut self, command: &str) {
        let result = self.command_executor.execute(command);
        
        if self.command_executor.should_exit() {
            std::process::exit(0);
        }
        
        match &result {
            Ok(command_result) => {
                let output_lines: Vec<String> = if !command_result.output.is_empty() {
                    command_result.output.lines().map(|s| s.to_string()).collect()
                } else {
                    vec![CommandHistoryText::format_command_success(command)]
                };
                
                log_command_execution(command, true, &command_result.output);
                
                self.history_commands.add_command_entry(
                    command.to_string(),
                    output_lines,
                    true,
                    None,
                );
            }
            Err(e) => {
                let error_msg = e.to_string();
                
                log_command_execution(command, false, &error_msg);
                
                self.history_commands.add_command_entry(
                    command.to_string(),
                    vec![],
                    false,
                    Some(error_msg.clone()),
                );
                self.input_handler.add_output_line(ErrorMessages::format_error(&error_msg));
            }
        }
        
        self.clear_input();
    }

    fn clear_input(&mut self) {
        self.input_handler.clear_buffer();
        self.input_buffer.clear();
        self.input_buffer_dirty = true;
    }

    pub fn get_last_input_time(&self) -> Instant {
        self.last_input_time
    }

    pub fn get_input_handler(&self) -> &HandleInputUseCase {
        &self.input_handler
    }

    pub fn get_input_handler_mut(&mut self) -> &mut HandleInputUseCase {
        &mut self.input_handler
    }

    pub fn get_history_commands(&self) -> &HistoryCommandsUseCase {
        &self.history_commands
    }
}
