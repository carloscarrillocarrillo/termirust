pub struct ErrorMessages;

impl ErrorMessages {
    /// Formatea un mensaje de error con el prefijo "Error:"
    pub fn format_error(error: &str) -> String {
        format!("Error: {}", error)
    }
    
    /// Formatea un mensaje específico para comandos no encontrados
    pub fn format_command_not_found(command_name: &str) -> String {
        format!("Error: El comando '{}' no se pudo encontrar", command_name)
    }
    
    /// Retorna el prefijo de error estándar
    pub fn error_prefix() -> &'static str {
        "Error:"
    }
    
    /// Verifica si una línea contiene un mensaje de error
    pub fn is_error_message(line: &str) -> bool {
        line.starts_with(Self::error_prefix())
    }
}
