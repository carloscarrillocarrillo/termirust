use crate::domain::entities::{Command, TerminalState};
use crate::domain::repositories::CommandRepository;

/// Servicio que maneja la lógica de comandos
pub struct CommandService<R>
where
    R: CommandRepository,
{
    repository: R,
}

impl<R> CommandService<R>
where
    R: CommandRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute_command(&self, command: &Command) -> Result<Command, String> {
        self.repository.execute_command(command)
    }

    pub fn parse_command(&self, input: &str) -> Command {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Command::new(String::new(), Vec::new());
        }

        let name = parts[0].to_string();
        let args = parts[1..].iter().map(|s| s.to_string()).collect();

        Command::new(name, args)
    }
}



/// Servicio que maneja la lógica de la terminal
pub struct TerminalService {
    state: TerminalState,
}

impl TerminalService {
    pub fn new() -> Self {
        Self {
            state: TerminalState::default(),
        }
    }

    pub fn get_state(&self) -> &TerminalState {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut TerminalState {
        &mut self.state
    }

    pub fn switch_mode(&mut self, mode: crate::domain::entities::TerminalMode) {
        self.state.mode = mode;
    }

    pub fn add_output_line(&mut self, line: String) {
        self.state.output_lines.push(line);
        if self.state.output_lines.len() > 1000 {
            self.state.output_lines.remove(0);
        }
    }

    pub fn clear_output(&mut self) {
        self.state.output_lines.clear();
    }

    pub fn should_exit(&self) -> bool {
        self.state.should_exit
    }
}
