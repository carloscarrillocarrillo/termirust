use crate::presentation::texts::{CommandHistory, CommandHistoryText};

/// Casos de uso para los comandos del historial
pub struct HistoryCommandsUseCase {
    command_history: CommandHistory,
}

impl HistoryCommandsUseCase {
    /// Crea una nueva instancia del caso de uso
    pub fn new(max_entries: usize) -> Self {
        Self {
            command_history: CommandHistory::new(max_entries),
        }
    }
    
    /// Obtiene una referencia al historial de comandos
    #[allow(dead_code)]
    pub fn get_history(&self) -> &CommandHistory {
        &self.command_history
    }
    
    /// Obtiene una referencia mutable al historial de comandos
    #[allow(dead_code)]
    pub fn get_history_mut(&mut self) -> &mut CommandHistory {
        &mut self.command_history
    }
    
    /// Ejecuta el comando `history` - muestra todo el historial
    pub fn show_full_history(&self) -> Vec<String> {
        let history_lines = self.command_history.format_full_history();
        if history_lines.is_empty() {
            vec![CommandHistoryText::format_empty_history()]
        } else {
            let mut result = vec![CommandHistoryText::format_history_header()];
            result.extend(history_lines);
            result
        }
    }
    
    /// Ejecuta el comando `history -n <count>` - muestra los últimos N comandos
    pub fn show_recent_history(&self, count: usize) -> Vec<String> {
        let history_lines = self.command_history.format_recent_history(count);
        if history_lines.is_empty() {
            vec![CommandHistoryText::format_empty_history()]
        } else {
            let mut result = vec![CommandHistoryText::format_history_header()];
            result.extend(history_lines);
            result
        }
    }
    
    /// Ejecuta el comando `history -c` - limpia el historial
    pub fn clear_history(&mut self) -> Vec<String> {
        self.command_history.clear();
        vec!["🗑️ Historial limpiado".to_string()]
    }
    
    /// Ejecuta el comando `history -s` - muestra estadísticas
    pub fn show_statistics(&self) -> Vec<String> {
        let stats = self.command_history.get_stats();
        stats.format_stats()
    }
    
    /// Ejecuta el comando `history -g <pattern>` - busca comandos
    pub fn search_commands(&self, pattern: &str) -> Vec<String> {
        if pattern.is_empty() {
            vec!["Uso: history -g <patrón>".to_string()]
        } else {
            let results = self.command_history.search_commands(pattern);
            if results.is_empty() {
                vec![CommandHistoryText::format_no_search_results(pattern)]
            } else {
                let mut result = vec![format!("🔍 Resultados para '{}':", pattern)];
                for entry in results {
                    let entry_lines = self.command_history.format_entry(entry);
                    result.extend(entry_lines);
                }
                result
            }
        }
    }
    
    /// Agrega una nueva entrada al historial
    pub fn add_command_entry(&mut self, command: String, output: Vec<String>, success: bool, error_message: Option<String>) {
        self.command_history.add_entry(command, output, success, error_message);
    }
    
    /// Obtiene las estadísticas del historial
    pub fn get_history_stats(&self) -> crate::presentation::texts::HistoryStats {
        self.command_history.get_stats()
    }
}

/// Parser para comandos del historial
pub struct HistoryCommandParser;

impl HistoryCommandParser {
    /// Parsea un comando y determina qué acción ejecutar
    pub fn parse_command(command: &str) -> HistoryCommand {
        let trimmed = command.trim();
        
        match trimmed {
            "history" | "hist" => HistoryCommand::ShowFull,
            "history -n 10" | "hist -n 10" => HistoryCommand::ShowRecent(10),
            "history -c" | "hist -c" => HistoryCommand::Clear,
            "history -s" | "hist -s" => HistoryCommand::ShowStats,
            cmd if cmd.starts_with("history -g ") || cmd.starts_with("hist -g ") => {
                let pattern = cmd.split_whitespace().nth(2).unwrap_or("").to_string();
                HistoryCommand::Search(pattern)
            }
            cmd if cmd.starts_with("history -n ") || cmd.starts_with("hist -n ") => {
                if let Some(count_str) = cmd.split_whitespace().nth(2) {
                    if let Ok(count) = count_str.parse::<usize>() {
                        return HistoryCommand::ShowRecent(count);
                    }
                }
                HistoryCommand::Invalid("Formato inválido para history -n".to_string())
            }
            _ => HistoryCommand::NotHistoryCommand,
        }
    }
    
    /// Verifica si un comando es un comando del historial
    #[allow(dead_code)]
    pub fn is_history_command(command: &str) -> bool {
        matches!(Self::parse_command(command), HistoryCommand::NotHistoryCommand)
    }
}

/// Enum que representa los diferentes comandos del historial
#[derive(Debug, Clone)]
pub enum HistoryCommand {
    ShowFull,
    ShowRecent(usize),
    Clear,
    ShowStats,
    Search(String),
    Invalid(String),
    NotHistoryCommand,
}

impl HistoryCommand {
    /// Ejecuta el comando usando el caso de uso proporcionado
    pub fn execute(self, use_case: &mut HistoryCommandsUseCase) -> Vec<String> {
        match self {
            HistoryCommand::ShowFull => use_case.show_full_history(),
            HistoryCommand::ShowRecent(count) => use_case.show_recent_history(count),
            HistoryCommand::Clear => use_case.clear_history(),
            HistoryCommand::ShowStats => use_case.show_statistics(),
            HistoryCommand::Search(pattern) => use_case.search_commands(&pattern),
            HistoryCommand::Invalid(message) => vec![format!("Error: {}", message)],
            HistoryCommand::NotHistoryCommand => vec![],
        }
    }
}

/// Interfaz para el repositorio de comandos del historial
pub trait HistoryCommandRepository {
    /// Guarda el historial de comandos
    #[allow(dead_code)]
    fn save_history(&self, history: &CommandHistory) -> Result<(), String>;
    
    /// Carga el historial de comandos
    #[allow(dead_code)]
    fn load_history(&self) -> Result<CommandHistory, String>;
    
    /// Verifica si existe un historial guardado
    #[allow(dead_code)]
    fn history_exists(&self) -> bool;
}

/// Implementación por defecto del repositorio (en memoria)
pub struct InMemoryHistoryRepository {
    history: Option<CommandHistory>,
}

impl InMemoryHistoryRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { history: None }
    }
}

impl HistoryCommandRepository for InMemoryHistoryRepository {
    fn save_history(&self, _history: &CommandHistory) -> Result<(), String> {
        // En esta implementación en memoria, no hacemos nada
        // En una implementación real, guardaríamos en archivo o base de datos
        Ok(())
    }
    
    fn load_history(&self) -> Result<CommandHistory, String> {
        if let Some(history) = &self.history {
            Ok(history.clone())
        } else {
            Ok(CommandHistory::new(100))
        }
    }
    
    fn history_exists(&self) -> bool {
        self.history.is_some()
    }
}
