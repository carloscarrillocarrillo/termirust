# Módulo de Textos - Termirust

Este módulo contiene todas las secciones de texto que se imprimen en pantalla, organizadas en archivos separados para mejor mantenimiento y reutilización.

## Estructura

### `welcome_messages.rs`
Contiene los mensajes de bienvenida que se muestran al iniciar la aplicación:
- Mensaje principal de bienvenida
- Instrucciones de uso
- Líneas vacías para separación

### `error_messages.rs`
Maneja el formato y detección de mensajes de error:
- Formateo de errores con prefijo "Error:"
- Detección de líneas que contienen errores
- Prefijo estándar para errores

### `system_indicators.rs`
Gestiona los indicadores del sistema en tiempo real:
- Formateo de uso de memoria (RAM)
- Formateo de uso de CPU
- Formateo de estadísticas de red
- Conversión de bytes a megabytes
- Lógica de colores según el nivel de uso

### `prompt_text.rs`
Maneja el texto del prompt del terminal:
- Generación del prompt con directorio actual
- Formateo del prompt completo con comando
- Cálculo de posiciones del cursor
- Obtención del directorio actual

### `debug_messages.rs`
Contiene los mensajes de depuración:
- Formateo de información del buffer
- Formateo de posición del cursor
- Detección de mensajes de debug
- Prefijo estándar para debug

### `command_history.rs`
Gestiona el historial de comandos ejecutados:
- Almacenamiento de comandos con timestamps
- Seguimiento de éxitos y fallos
- Búsqueda de comandos por patrón
- Estadísticas del historial
- Formateo para visualización
- Comandos especiales del historial

## Uso

```rust
use crate::presentation::texts::{WelcomeMessages, ErrorMessages, SystemIndicators, PromptText, DebugMessages, CommandHistory, CommandHistoryText};

// Mensajes de bienvenida
for message in WelcomeMessages::all_welcome_messages() {
    println!("{}", message);
}

// Formatear error
let error_msg = ErrorMessages::format_error("Comando no encontrado");

// Formatear indicadores del sistema
let memory_text = SystemIndicators::format_memory_usage(75.5);

// Generar prompt
let prompt = PromptText::format_full_prompt("ls -la");

// Mensaje de debug
let debug_msg = DebugMessages::format_buffer_debug("comando", 5);

// Historial de comandos
let mut history = CommandHistory::new(100);
history.add_entry("ls -la".to_string(), vec!["archivo1.txt".to_string()], true, None);
let stats = history.get_stats();
```

## Ventajas de esta separación

1. **Mantenibilidad**: Cada tipo de texto está en su propio archivo
2. **Reutilización**: Los métodos pueden ser reutilizados en diferentes partes del código
3. **Consistencia**: Formato estándar para cada tipo de mensaje
4. **Testabilidad**: Cada módulo puede ser probado independientemente
5. **Localización**: Fácil agregar soporte para múltiples idiomas en el futuro
