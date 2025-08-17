use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};
use std::time::Duration;

use crate::application::use_cases::{ExecuteCommandUseCase, HandleInputUseCase};
use crate::domain::entities::TerminalMode;
use crate::domain::repositories::CommandRepository;
use crate::presentation::matrix_effects::MatrixEffectManager;

pub struct TerminalUI<R>
where
    R: CommandRepository,
{
    matrix_effects: MatrixEffectManager,
    input_handler: HandleInputUseCase,
    command_executor: ExecuteCommandUseCase<R>,
    running: bool,
}

impl<R> TerminalUI<R>
where
    R: CommandRepository,
{
    pub fn new(
        matrix_effects: MatrixEffectManager,
        input_handler: HandleInputUseCase,
        command_executor: ExecuteCommandUseCase<R>,
    ) -> Self {
        Self {
            matrix_effects,
            input_handler,
            command_executor,
            running: true,
        }
    }

    pub fn init(&mut self) -> std::io::Result<()> {
        execute!(stdout(), EnterAlternateScreen, Hide)?;
        crossterm::terminal::enable_raw_mode()?;
        Ok(())
    }

    pub fn cleanup(&self) -> std::io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen, Show)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn handle_events(&mut self) -> std::io::Result<()> {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            self.running = false;
                        }
                        KeyCode::Esc => {
                            if self.input_handler.get_current_mode() == TerminalMode::Command {
                                self.input_handler.switch_to_matrix_mode();
                            } else {
                                self.running = false;
                            }
                        }
                        KeyCode::Enter => {
                            if self.input_handler.get_current_mode() == TerminalMode::Command {
                                let command = self.input_handler.get_command_buffer();
                                if !command.trim().is_empty() {
                                    if let Err(e) = self.command_executor.execute(&command) {
                                        self.input_handler.add_output_line(format!("Error: {}", e));
                                    }
                                    self.input_handler.clear_buffer();
                                }
                            } else {
                                self.input_handler.switch_to_command_mode();
                            }
                        }
                        KeyCode::Char(ch) => {
                            self.input_handler.handle_key_press(ch);
                        }
                        KeyCode::Backspace => {
                            self.input_handler.handle_backspace();
                        }
                        KeyCode::Left => {
                            self.input_handler.handle_arrow_left();
                        }
                        KeyCode::Right => {
                            self.input_handler.handle_arrow_right();
                        }
                        KeyCode::Up => {
                            self.input_handler.handle_arrow_up();
                        }
                        KeyCode::Down => {
                            self.input_handler.handle_arrow_down();
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    pub fn draw(&self) -> std::io::Result<()> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))?;

        match self.input_handler.get_current_mode() {
            TerminalMode::Matrix => {
                // Dibujar efectos Matrix
                self.matrix_effects.draw(&mut stdout)?;
                
                // Información de ayuda
                let help_text = "MATRIX TERMINAL - Presiona ENTER para comandos, ESC para salir";
                execute!(
                    stdout,
                    crossterm::cursor::MoveTo(0, self.matrix_effects.terminal_height - 1),
                    SetForegroundColor(Color::AnsiValue(10)),
                    Print(help_text),
                    ResetColor
                )?;
            }
            TerminalMode::Command => {
                // Dibujar salida de comandos
                let output_lines = self.input_handler.get_output_lines();
                let start_y = 0;
                let max_lines = (self.matrix_effects.terminal_height - 3) as usize;
                
                for (i, line) in output_lines.iter().rev().take(max_lines).enumerate() {
                    let y = start_y + i as u16;
                    execute!(
                        stdout,
                        crossterm::cursor::MoveTo(0, y),
                        SetForegroundColor(Color::AnsiValue(10)),
                        Print(line),
                        ResetColor
                    )?;
                }

                // Dibujar prompt
                let current_dir = std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let prompt = format!("matrix@{}:~$ ", current_dir);
                let prompt_len = prompt.len();
                
                let prompt_y = self.matrix_effects.terminal_height - 2;
                execute!(
                    stdout,
                    crossterm::cursor::MoveTo(0, prompt_y),
                    SetForegroundColor(Color::AnsiValue(10)),
                    Print(&prompt),
                    ResetColor
                )?;

                // Dibujar línea de comando
                let command_buffer = self.input_handler.get_command_buffer();
                let cursor_pos = self.input_handler.get_cursor_position();
                
                execute!(
                    stdout,
                    crossterm::cursor::MoveTo(prompt_len as u16, prompt_y),
                    SetForegroundColor(Color::AnsiValue(10)),
                    Print(&command_buffer),
                    ResetColor
                )?;

                // Posicionar cursor
                execute!(
                    stdout,
                    crossterm::cursor::MoveTo((prompt_len + cursor_pos) as u16, prompt_y)
                )?;

                // Información de ayuda
                let help_text = "Presiona ESC para volver al modo Matrix";
                execute!(
                    stdout,
                    crossterm::cursor::MoveTo(0, self.matrix_effects.terminal_height - 1),
                    SetForegroundColor(Color::AnsiValue(82)),
                    Print(help_text),
                    ResetColor
                )?;
            }
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn update_matrix_effects(&mut self) {
        self.matrix_effects.add_random_drops();
        self.matrix_effects.update_drops();
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        self.init()?;

        while self.running {
            self.handle_events()?;
            
            if self.input_handler.get_current_mode() == TerminalMode::Matrix {
                self.update_matrix_effects();
            }
            
            self.draw()?;
            std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
        }

        self.cleanup()?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
