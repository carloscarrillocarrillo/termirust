use crate::domain::entities::Command;

/// Trait que define el repositorio de comandos
pub trait CommandRepository {
    fn execute_command(&self, command: &Command) -> Result<Command, String>;
}

/// Trait que define el repositorio de archivos del sistema
pub trait FileSystemRepository {
    #[allow(dead_code)]
    fn list_directory(&self, path: &str) -> Result<Vec<String>, String>;
    fn get_current_directory(&self) -> Result<String, String>;
    fn change_directory(&self, path: &str) -> Result<(), String>;
}
