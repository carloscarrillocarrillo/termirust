use std::collections::VecDeque;

#[derive(Clone)]
pub struct CommandHistory {
    entries: VecDeque<CommandEntry>,
    max_entries: usize,
}

#[derive(Debug, Clone)]
pub struct CommandEntry {
    pub command: String,
    pub output: Vec<String>,
    pub timestamp: std::time::SystemTime,
    pub success: bool,
    pub error_message: Option<String>,
}

impl CommandHistory {
    /// Crea una nueva instancia del historial de comandos
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries,
        }
    }
    
    /// Agrega una nueva entrada al historial
    pub fn add_entry(&mut self, command: String, output: Vec<String>, success: bool, error_message: Option<String>) {
        let entry = CommandEntry {
            command,
            output,
            timestamp: std::time::SystemTime::now(),
            success,
            error_message,
        };
        
        self.entries.push_back(entry);
        
        // Mantener solo el número máximo de entradas
        if self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }
    
    /// Obtiene todas las entradas del historial
    #[allow(dead_code)]
    pub fn get_entries(&self) -> &VecDeque<CommandEntry> {
        &self.entries
    }
    
    /// Obtiene las últimas N entradas del historial
    pub fn get_last_entries(&self, count: usize) -> Vec<&CommandEntry> {
        self.entries
            .iter()
            .rev()
            .take(count)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
    
    /// Obtiene el comando más reciente
    #[allow(dead_code)]
    pub fn get_last_command(&self) -> Option<&str> {
        self.entries.back().map(|entry| entry.command.as_str())
    }
    
    /// Busca comandos en el historial que coincidan con un patrón
    pub fn search_commands(&self, pattern: &str) -> Vec<&CommandEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.command.to_lowercase().contains(&pattern.to_lowercase()))
            .collect()
    }
    
    /// Limpia todo el historial
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    /// Obtiene estadísticas del historial
    pub fn get_stats(&self) -> HistoryStats {
        let total_commands = self.entries.len();
        let successful_commands = self.entries.iter().filter(|entry| entry.success).count();
        let failed_commands = total_commands - successful_commands;
        
        HistoryStats {
            total_commands,
            successful_commands,
            failed_commands,
            success_rate: if total_commands > 0 {
                (successful_commands as f64 / total_commands as f64) * 100.0
            } else {
                0.0
            },
        }
    }
    
    /// Formatea una entrada del historial para mostrar
    pub fn format_entry(&self, entry: &CommandEntry) -> Vec<String> {
        let mut lines = Vec::new();
        
        // Formatear timestamp
        let timestamp = entry.timestamp
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let time_str = format!("[{}]", timestamp);
        
        // Formatear comando con indicador de éxito/fallo
        let status_icon = if entry.success { "✅" } else { "❌" };
        let command_line = format!("{} {} {}", time_str, status_icon, entry.command);
        lines.push(command_line);
        
        // Agregar salida del comando
        for output_line in &entry.output {
            lines.push(format!("  {}", output_line));
        }
        
        // Agregar mensaje de error si existe
        if let Some(error) = &entry.error_message {
            lines.push(format!("  Error: {}", error));
        }
        
        lines
    }
    
    /// Formatea todo el historial para mostrar
    pub fn format_full_history(&self) -> Vec<String> {
        let mut all_lines = Vec::new();
        
        for entry in self.entries.iter() {
            let entry_lines = self.format_entry(entry);
            all_lines.extend(entry_lines);
            all_lines.push("".to_string()); // Línea vacía entre entradas
        }
        
        all_lines
    }
    
    /// Formatea las últimas N entradas del historial
    pub fn format_recent_history(&self, count: usize) -> Vec<String> {
        let mut all_lines = Vec::new();
        
        for entry in self.get_last_entries(count) {
            let entry_lines = self.format_entry(entry);
            all_lines.extend(entry_lines);
            all_lines.push("".to_string()); // Línea vacía entre entradas
        }
        
        all_lines
    }
}

#[derive(Debug, Clone)]
pub struct HistoryStats {
    pub total_commands: usize,
    pub successful_commands: usize,
    pub failed_commands: usize,
    pub success_rate: f64,
}

impl HistoryStats {
    /// Formatea las estadísticas para mostrar
    pub fn format_stats(&self) -> Vec<String> {
        vec![
            format!("📊 Estadísticas del Historial:"),
            format!("  Total de comandos: {}", self.total_commands),
            format!("  Comandos exitosos: {} ✅", self.successful_commands),
            format!("  Comandos fallidos: {} ❌", self.failed_commands),
            format!("  Tasa de éxito: {:.1}%", self.success_rate),
        ]
    }
}

// Funciones de utilidad para formateo de texto
pub struct CommandHistoryText;

impl CommandHistoryText {
    /// Formatea un mensaje de comando ejecutado
    #[allow(dead_code)]
    pub fn format_command_executed(command: &str) -> String {
        format!("Comando ejecutado: {}", command)
    }
    
    /// Formatea un mensaje de comando exitoso
    pub fn format_command_success(command: &str) -> String {
        format!("✅ {} - Ejecutado exitosamente", command)
    }
    
    /// Formatea un mensaje de comando fallido
    #[allow(dead_code)]
    pub fn format_command_failed(command: &str, error: &str) -> String {
        format!("❌ {} - Error: {}", command, error)
    }
    
    /// Formatea el encabezado del historial
    pub fn format_history_header() -> String {
        "📜 Historial de Comandos:".to_string()
    }
    
    /// Formatea un separador de entrada
    #[allow(dead_code)]
    pub fn format_entry_separator() -> String {
        "─".repeat(50)
    }
    
    /// Formatea un mensaje de historial vacío
    pub fn format_empty_history() -> String {
        "📭 No hay comandos en el historial".to_string()
    }
    
    /// Formatea un mensaje de búsqueda sin resultados
    pub fn format_no_search_results(pattern: &str) -> String {
        format!("🔍 No se encontraron comandos que coincidan con '{}'", pattern)
    }
}
