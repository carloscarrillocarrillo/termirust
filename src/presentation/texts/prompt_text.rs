

pub struct PromptText;

impl PromptText {
    /// Genera el prompt del terminal con el directorio actual
    pub fn generate_prompt() -> String {
        let current_dir = Self::get_current_directory();
        format!("{}:~$ ", current_dir)
    }
    
    /// Obtiene el directorio actual como string
    fn get_current_directory() -> String {
        std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }
    
    /// Formatea el prompt completo con el comando actual
    pub fn format_full_prompt(command_buffer: &str) -> String {
        let prompt = Self::generate_prompt();
        format!("{}{}", prompt, command_buffer)
    }
    
    /// Obtiene solo el prompt sin el comando
    pub fn get_prompt_only() -> String {
        Self::generate_prompt()
    }
    
    /// Calcula la posición X del cursor basado en el prompt y la posición del cursor
    pub fn calculate_cursor_x(prompt: &str, cursor_position: usize) -> f32 {
        let prompt_width = prompt.len() as f32 * 9.5;
        let buffer_width = cursor_position as f32 * 9.5;
        20.0 + prompt_width + buffer_width
    }
    
    /// Calcula la posición X del cursor considerando el prompt completo y el comando
    pub fn calculate_cursor_x_with_buffer(prompt: &str, command_buffer: &String, cursor_position: usize) -> f32 {
        let prompt_width = prompt.len() as f32 * 9.5;
        // Calcula el ancho del texto del buffer hasta la posición del cursor
        let buffer_text = &command_buffer[..cursor_position.min(command_buffer.len())];
        let buffer_width = buffer_text.len() as f32 * 9.5;
        20.0 + prompt_width + buffer_width
    }
}
