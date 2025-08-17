pub struct SystemIndicators;

impl SystemIndicators {
    /// Formatea el indicador de memoria
    pub fn format_memory_usage(usage: f64) -> String {
        format!("💾 RAM: {:.1}%", usage)
    }
    
    /// Formatea el indicador de CPU
    pub fn format_cpu_usage(usage: f64) -> String {
        format!("⚡ CPU: {:.1}%", usage)
    }
    
    /// Formatea el indicador de red
    pub fn format_network_usage(received_mb: f64, transmitted_mb: f64) -> String {
        format!("🌐 NET: ↓{:.1}MB ↑{:.1}MB", received_mb, transmitted_mb)
    }
    
    /// Convierte bytes a megabytes
    pub fn bytes_to_mb(bytes: u64) -> f64 {
        bytes as f64 / (1024.0 * 1024.0)
    }
    
    /// Retorna el color apropiado para el uso de memoria
    pub fn get_memory_color(usage: f64) -> &'static str {
        if usage > 80.0 {
            "high"  // Rojo
        } else if usage > 60.0 {
            "medium"  // Amarillo
        } else {
            "low"  // Verde
        }
    }
    
    /// Retorna el color apropiado para el uso de CPU
    pub fn get_cpu_color(usage: f64) -> &'static str {
        if usage > 80.0 {
            "high"  // Rojo
        } else if usage > 60.0 {
            "medium"  // Amarillo
        } else {
            "low"  // Verde
        }
    }
    
    /// Retorna el color para indicadores de red
    #[allow(dead_code)]
    pub fn get_network_color() -> &'static str {
        "blue"  // Azul
    }
}
