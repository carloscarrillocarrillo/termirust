use crate::domain::entities::Command;

/// Caso de uso para el comando exit
pub struct ExitCommandsUseCase;

impl ExitCommandsUseCase {
    /// Crea una nueva instancia del caso de uso
    pub fn new() -> Self {
        Self
    }

    /// Ejecuta el comando exit - prepara la aplicación para cerrarse
    pub fn execute_exit(&self, command: &Command) -> Result<ExitResult, String> {
        // Validar que es un comando exit válido
        if command.name != "exit" && command.name != "quit" {
            return Err("Comando no válido para exit".to_string());
        }

        // Crear el resultado del comando exit
        let result = ExitResult {
            should_exit: true,
            message: "Cerrando Termirust...".to_string(),
            exit_code: 0,
        };

        Ok(result)
    }

    /// Valida si el comando es un comando exit
    pub fn is_exit_command(&self, command_name: &str) -> bool {
        command_name == "exit" || command_name == "quit"
    }

    /// Obtiene información sobre el comando exit
    pub fn get_exit_info(&self) -> ExitCommandInfo {
        ExitCommandInfo {
            name: "exit".to_string(),
            aliases: vec!["quit".to_string()],
            description: "Cierra la aplicación Termirust".to_string(),
            usage: "exit [--help]".to_string(),
            help_text: r#"
Comando: exit
Alias: quit
Descripción: Cierra la aplicación Termirust

Uso:
  exit          - Cierra la aplicación inmediatamente
  exit --help   - Muestra esta ayuda
  quit          - Alias para exit

Ejemplos:
  exit          # Cierra la aplicación
  quit          # Cierra la aplicación (alias)
"#.to_string(),
        }
    }
}

/// Resultado de la ejecución del comando exit
#[derive(Debug, Clone)]
pub struct ExitResult {
    pub should_exit: bool,
    pub message: String,
    pub exit_code: i32,
}

/// Información del comando exit
#[derive(Debug, Clone)]
pub struct ExitCommandInfo {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub usage: String,
    pub help_text: String,
}

impl ExitResult {
    /// Crea un nuevo resultado de exit
    pub fn new(should_exit: bool, message: String) -> Self {
        Self {
            should_exit,
            message,
            exit_code: if should_exit { 0 } else { 1 },
        }
    }

    /// Crea un resultado de exit con código de salida personalizado
    pub fn with_exit_code(mut self, exit_code: i32) -> Self {
        self.exit_code = exit_code;
        self
    }
}

impl ExitCommandInfo {
    /// Crea nueva información del comando exit
    pub fn new() -> Self {
        Self {
            name: "exit".to_string(),
            aliases: vec!["quit".to_string()],
            description: "Cierra la aplicación Termirust".to_string(),
            usage: "exit [--help]".to_string(),
            help_text: "Comando para cerrar la aplicación".to_string(),
        }
    }

    /// Obtiene el texto de ayuda formateado
    pub fn get_formatted_help(&self) -> String {
        self.help_text.clone()
    }
}
