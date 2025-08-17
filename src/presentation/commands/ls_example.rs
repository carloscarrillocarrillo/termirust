use crate::application::ls_commands::{LsCommandsUseCase, LsCommandParser};
use crate::presentation::commands::ls_display::LsDisplayComponent;

/// Ejemplo de uso del comando ls
#[allow(dead_code)]
pub struct LsExample;

impl LsExample {
    /// Ejecuta ejemplos del comando ls
    #[allow(dead_code)]
    pub fn run_examples() -> Vec<String> {
        let mut output = Vec::new();
        let use_case = LsCommandsUseCase::new();

        output.push("🚀 Ejemplos del comando ls:".to_string());
        output.push(String::new());

        // Ejemplo 1: ls básico
        output.push("1️⃣ Comando básico 'ls':".to_string());
        let command1 = "ls";
        let parsed1 = LsCommandParser::parse_command(command1);
        match parsed1.execute(&use_case) {
            Ok(result) => {
                let display_result = LsDisplayComponent::render(&result, &crate::application::ls_commands::LsOptions::default());
                output.extend(display_result);
            }
            Err(e) => {
                output.extend(LsDisplayComponent::render_error(&e));
            }
        }
        output.push(String::new());

        // Ejemplo 2: ls con formato largo
        output.push("2️⃣ Comando 'ls -l' (formato largo):".to_string());
        let command2 = "ls -l";
        let parsed2 = LsCommandParser::parse_command(command2);
        match parsed2.execute(&use_case) {
            Ok(result) => {
                let mut options = crate::application::ls_commands::LsOptions::default();
                options.long_format = true;
                let display_result = LsDisplayComponent::render(&result, &options);
                output.extend(display_result);
            }
            Err(e) => {
                output.extend(LsDisplayComponent::render_error(&e));
            }
        }
        output.push(String::new());

        // Ejemplo 3: ls con archivos ocultos
        output.push("3️⃣ Comando 'ls -a' (archivos ocultos):".to_string());
        let command3 = "ls -a";
        let parsed3 = LsCommandParser::parse_command(command3);
        match parsed3.execute(&use_case) {
            Ok(result) => {
                let mut options = crate::application::ls_commands::LsOptions::default();
                options.show_hidden = true;
                let display_result = LsDisplayComponent::render(&result, &options);
                output.extend(display_result);
            }
            Err(e) => {
                output.extend(LsDisplayComponent::render_error(&e));
            }
        }
        output.push(String::new());

        // Ejemplo 4: ls con tamaños legibles
        output.push("4️⃣ Comando 'ls -lh' (tamaños legibles):".to_string());
        let command4 = "ls -lh";
        let parsed4 = LsCommandParser::parse_command(command4);
        match parsed4.execute(&use_case) {
            Ok(result) => {
                let mut options = crate::application::ls_commands::LsOptions::default();
                options.long_format = true;
                options.human_readable = true;
                let display_result = LsDisplayComponent::render(&result, &options);
                output.extend(display_result);
            }
            Err(e) => {
                output.extend(LsDisplayComponent::render_error(&e));
            }
        }
        output.push(String::new());

        // Ejemplo 5: ls ordenado por tamaño
        output.push("5️⃣ Comando 'ls -S' (ordenado por tamaño):".to_string());
        let command5 = "ls -S";
        let parsed5 = LsCommandParser::parse_command(command5);
        match parsed5.execute(&use_case) {
            Ok(result) => {
                let mut options = crate::application::ls_commands::LsOptions::default();
                options.sort_by = crate::application::ls_commands::SortBy::Size;
                let display_result = LsDisplayComponent::render(&result, &options);
                output.extend(display_result);
            }
            Err(e) => {
                output.extend(LsDisplayComponent::render_error(&e));
            }
        }
        output.push(String::new());

        // Mostrar ayuda
        output.push("📖 Ayuda del comando ls:".to_string());
        output.extend(LsDisplayComponent::render_help());

        output
    }

    /// Ejecuta un comando ls específico
    #[allow(dead_code)]
    pub fn execute_ls_command(command: &str) -> Vec<String> {
        let use_case = LsCommandsUseCase::new();
        let parsed = LsCommandParser::parse_command(command);

        match parsed.execute(&use_case) {
            Ok(result) => {
                let mut options = crate::application::ls_commands::LsOptions::default();
                
                // Detectar opciones del comando
                if command.contains("-l") {
                    options.long_format = true;
                }
                if command.contains("-a") {
                    options.show_hidden = true;
                }
                if command.contains("-h") {
                    options.human_readable = true;
                }
                if command.contains("-S") {
                    options.sort_by = crate::application::ls_commands::SortBy::Size;
                }
                if command.contains("-t") {
                    options.sort_by = crate::application::ls_commands::SortBy::Modified;
                }

                LsDisplayComponent::render(&result, &options)
            }
            Err(e) => LsDisplayComponent::render_error(&e),
        }
    }

    /// Muestra una vista previa del directorio actual
    #[allow(dead_code)]
    pub fn show_preview() -> Vec<String> {
        let use_case = LsCommandsUseCase::new();
        
        match use_case.list_directory(None) {
            Ok(result) => LsDisplayComponent::render_preview(&result),
            Err(e) => LsDisplayComponent::render_error(&e),
        }
    }
}
