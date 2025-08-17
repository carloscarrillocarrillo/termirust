pub struct WelcomeMessages;

impl WelcomeMessages {
    /// Retorna el mensaje principal de bienvenida
    pub fn main_welcome() -> String {
        "Bienvenido a Termirust - Terminal Matrix".to_string()
    }
    
    /// Retorna las instrucciones de uso
    pub fn usage_instructions() -> String {
        "Escribe comandos y presiona ENTER para ejecutarlos".to_string()
    }
    
    /// Retorna una línea vacía para separación
    pub fn empty_line() -> String {
        "".to_string()
    }
    
    /// Retorna todos los mensajes de bienvenida en orden
    pub fn all_welcome_messages() -> Vec<String> {
        vec![
            Self::main_welcome(),
            Self::usage_instructions(),
            Self::empty_line(),
        ]
    }
}
