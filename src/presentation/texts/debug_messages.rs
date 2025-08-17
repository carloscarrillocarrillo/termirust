pub struct DebugMessages;

impl DebugMessages {
    /// Formatea el mensaje de debug del buffer y posición del cursor
    pub fn format_buffer_debug(command_buffer: &str, cursor_position: usize) -> String {
        format!(
            "[DEBUG] Buffer: '{}' ({} chars, cursor at {})", 
            command_buffer, 
            command_buffer.len(), 
            cursor_position
        )
    }
    
    /// Retorna el prefijo de debug estándar
    #[allow(dead_code)]
    pub fn debug_prefix() -> &'static str {
        "[DEBUG]"
    }
    
    /// Verifica si una línea contiene un mensaje de debug
    #[allow(dead_code)]
    pub fn is_debug_message(line: &str) -> bool {
        line.starts_with(Self::debug_prefix())
    }
    
    /// Formatea un mensaje de debug genérico
    #[allow(dead_code)]
    pub fn format_generic_debug(message: &str) -> String {
        format!("{} {}", Self::debug_prefix(), message)
    }
}
