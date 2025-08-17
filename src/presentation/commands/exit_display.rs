use crate::application::commands::exit_commands::{ExitResult, ExitCommandInfo};

/// Componente de presentación para el comando exit
pub struct ExitDisplayComponent;

impl ExitDisplayComponent {
    /// Renderiza el resultado del comando exit
    pub fn render(result: &ExitResult) -> Vec<String> {
        let mut output = Vec::new();
        
        // Agregar el mensaje principal
        output.push(result.message.clone());
        
        // Agregar información adicional si es necesario
        if result.should_exit {
            output.push("🔄 Preparando cierre de la aplicación...".to_string());
            output.push("📝 Guardando configuración...".to_string());
            output.push("👋 ¡Hasta luego!".to_string());
        }
        
        output
    }

    /// Renderiza información de ayuda del comando exit
    pub fn render_help(info: &ExitCommandInfo) -> Vec<String> {
        let mut output = Vec::new();
        
        // Título
        output.push("🚪 COMANDO EXIT".to_string());
        output.push("=".repeat(50));
        output.push(String::new());
        
        // Información básica
        output.push(format!("📝 Descripción: {}", info.description));
        output.push(format!("🔗 Alias: {}", info.aliases.join(", ")));
        output.push(format!("💻 Uso: {}", info.usage));
        output.push(String::new());
        
        // Texto de ayuda detallado
        let help_lines: Vec<&str> = info.help_text.lines().collect();
        for line in help_lines {
            if !line.trim().is_empty() {
                output.push(line.to_string());
            }
        }
        
        output
    }

    /// Renderiza un mensaje de error para el comando exit
    pub fn render_error(error: &str) -> Vec<String> {
        vec![
            "❌ Error en comando exit:".to_string(),
            format!("   {}", error),
            String::new(),
            "💡 Uso correcto:".to_string(),
            "   exit          - Cierra la aplicación".to_string(),
            "   exit --help   - Muestra ayuda".to_string(),
        ]
    }

    /// Renderiza un mensaje de confirmación para el comando exit
    pub fn render_confirmation() -> Vec<String> {
        vec![
            "⚠️  Confirmación de salida".to_string(),
            "¿Estás seguro de que quieres cerrar Termirust?".to_string(),
            String::new(),
            "Presiona 'y' para confirmar o 'n' para cancelar:".to_string(),
        ]
    }

    /// Renderiza un mensaje de cancelación del comando exit
    pub fn render_cancellation() -> Vec<String> {
        vec![
            "✅ Salida cancelada".to_string(),
            "La aplicación continuará ejecutándose.".to_string(),
        ]
    }

    /// Renderiza estadísticas del comando exit
    pub fn render_stats(execution_count: u32, last_execution: Option<String>) -> Vec<String> {
        let mut output = Vec::new();
        
        output.push("📊 Estadísticas del comando exit:".to_string());
        output.push("=".repeat(40));
        output.push(format!("🔄 Veces ejecutado: {}", execution_count));
        
        if let Some(last_time) = last_execution {
            output.push(format!("🕒 Última ejecución: {}", last_time));
        } else {
            output.push("🕒 Última ejecución: Nunca".to_string());
        }
        
        output.push(String::new());
        output.push("💡 El comando exit cierra la aplicación de manera limpia,".to_string());
        output.push("   guardando la configuración y cerrando todos los procesos.".to_string());
        
        output
    }

    /// Formatea un mensaje de despedida personalizado
    pub fn format_farewell_message(username: Option<&str>) -> String {
        match username {
            Some(name) => format!("👋 ¡Hasta luego, {}! Gracias por usar Termirust.", name),
            None => "👋 ¡Hasta luego! Gracias por usar Termirust.".to_string(),
        }
    }

    /// Renderiza un mensaje de progreso del cierre
    pub fn render_shutdown_progress(step: u32, total_steps: u32) -> Vec<String> {
        let progress = (step as f32 / total_steps as f32 * 100.0) as u32;
        let progress_bar = Self::create_progress_bar(progress);
        
        vec![
            "🔄 Cerrando Termirust...".to_string(),
            format!("📊 Progreso: {}%", progress),
            progress_bar,
            format!("📝 Paso {}/{}: {}", step, total_steps, Self::get_step_description(step)),
        ]
    }

    /// Crea una barra de progreso visual
    fn create_progress_bar(percentage: u32) -> String {
        let bar_width = 30;
        let filled_width = (percentage as f32 / 100.0 * bar_width as f32) as usize;
        let empty_width = bar_width - filled_width;
        
        let filled = "█".repeat(filled_width);
        let empty = "░".repeat(empty_width);
        
        format!("[{}] {}%", filled + &empty, percentage)
    }

    /// Obtiene la descripción del paso actual
    fn get_step_description(step: u32) -> &'static str {
        match step {
            1 => "Guardando configuración",
            2 => "Cerrando conexiones",
            3 => "Liberando recursos",
            4 => "Finalizando procesos",
            5 => "Cerrando aplicación",
            _ => "Procesando...",
        }
    }
}
