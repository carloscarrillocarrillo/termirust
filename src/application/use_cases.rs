use crate::domain::entities::{Command, TerminalMode};
use crate::domain::services::{CommandService, TerminalService};
use crate::domain::repositories::CommandRepository;
use crate::application::commands::exit_commands::ExitCommandsUseCase;

/// Caso de uso para ejecutar comandos
pub struct ExecuteCommandUseCase<R>
where
    R: CommandRepository,
{
    command_service: CommandService<R>,
    terminal_service: TerminalService,
}

impl<R> ExecuteCommandUseCase<R>
where
    R: CommandRepository,
{
    pub fn new(command_service: CommandService<R>, terminal_service: TerminalService) -> Self {
        Self {
            command_service,
            terminal_service,
        }
    }

    pub fn execute(&mut self, input: &str) -> Result<Command, String> {
        // Parsear el comando
        let command = self.command_service.parse_command(input);
        
        // Agregar al historial
        self.terminal_service.get_state_mut().add_to_history(input.to_string());
        
        // Agregar el comando ejecutado al output
        let current_dir = std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let prompt = format!("{}:~$ ", current_dir);
        self.terminal_service.add_output_line(format!("{}{}", prompt, input));
        
        // Ejecutar el comando
        let result = self.command_service.execute_command(&command)?;
        
        // Manejar comandos especiales
        match result.name.as_str() {
            "clear" => {
                self.terminal_service.clear_output();
            }
            "exit" | "quit" => {
                // Usar el caso de uso específico del comando exit
                let exit_use_case = ExitCommandsUseCase::new();
                if exit_use_case.is_exit_command(&result.name) {
                    // Marcar que se debe cerrar la aplicación
                    self.terminal_service.get_state_mut().should_exit = true;
                }
            }
            _ => {
                // Agregar salida al terminal
                if !result.output.is_empty() {
                    for line in result.output.lines() {
                        self.terminal_service.add_output_line(line.to_string());
                    }
                }
            }
        }
        
        Ok(result)
    }

    pub fn should_exit(&self) -> bool {
        self.terminal_service.should_exit()
    }
}

/// Caso de uso para manejar la entrada de texto
pub struct HandleInputUseCase {
    terminal_service: TerminalService,
}

impl HandleInputUseCase {
    pub fn new(terminal_service: TerminalService) -> Self {
        Self { terminal_service }
    }

    pub fn add_output_line(&mut self, line: String) {
        self.terminal_service.add_output_line(line);
    }

    pub fn clear_buffer(&mut self) {
        self.terminal_service.get_state_mut().clear_buffer();
    }



    pub fn handle_key_press(&mut self, ch: char) {
        let state = self.terminal_service.get_state_mut();
        // Siempre procesar la entrada, independientemente del modo
        state.insert_char(ch);
    }

    pub fn handle_backspace(&mut self) {
        let state = self.terminal_service.get_state_mut();
        // Siempre procesar la entrada, independientemente del modo
        state.delete_char();
    }

    pub fn handle_arrow_left(&mut self) {
        let state = self.terminal_service.get_state_mut();
        if state.cursor_position > 0 {
            state.cursor_position -= 1;
        }
    }

    pub fn handle_arrow_right(&mut self) {
        let state = self.terminal_service.get_state_mut();
        if state.cursor_position < state.command_buffer.len() {
            state.cursor_position += 1;
        }
    }

    pub fn handle_home(&mut self) {
        let state = self.terminal_service.get_state_mut();
        state.cursor_position = 0;
    }

    pub fn handle_end(&mut self) {
        let state = self.terminal_service.get_state_mut();
        state.cursor_position = state.command_buffer.len();
    }

    pub fn get_command_buffer(&self) -> String {
        self.terminal_service.get_state().command_buffer.clone()
    }

    pub fn get_output_lines(&self) -> Vec<String> {
        self.terminal_service.get_state().output_lines.clone()
    }

    pub fn get_cursor_position(&self) -> usize {
        self.terminal_service.get_state().cursor_position
    }

    #[allow(dead_code)]
    pub fn get_current_mode(&self) -> TerminalMode {
        self.terminal_service.get_state().mode.clone()
    }

    #[allow(dead_code)]
    pub fn switch_to_command_mode(&mut self) {
        self.terminal_service.get_state_mut().mode = TerminalMode::Command;
    }

    #[allow(dead_code)]
    pub fn switch_to_matrix_mode(&mut self) {
        self.terminal_service.get_state_mut().mode = TerminalMode::Matrix;
    }
}
