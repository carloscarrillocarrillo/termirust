# Comandos del Historial - Capa de Aplicación

Este documento describe la implementación de los comandos del historial en la capa de aplicación, siguiendo los principios de Clean Architecture.

## Estructura

### `HistoryCommandsUseCase`
Caso de uso principal que maneja toda la lógica de negocio relacionada con los comandos del historial.

#### Métodos Principales:
- `show_full_history()` - Muestra todo el historial
- `show_recent_history(count)` - Muestra los últimos N comandos
- `clear_history()` - Limpia el historial
- `show_statistics()` - Muestra estadísticas
- `search_commands(pattern)` - Busca comandos por patrón
- `add_command_entry()` - Agrega una nueva entrada al historial

### `HistoryCommandParser`
Parser que analiza los comandos de entrada y determina qué acción ejecutar.

#### Funcionalidades:
- `parse_command(command)` - Parsea un comando y retorna el tipo de comando
- `is_history_command(command)` - Verifica si un comando es del historial

### `HistoryCommand`
Enum que representa los diferentes tipos de comandos del historial.

#### Variantes:
- `ShowFull` - Mostrar todo el historial
- `ShowRecent(usize)` - Mostrar últimos N comandos
- `Clear` - Limpiar historial
- `ShowStats` - Mostrar estadísticas
- `Search(String)` - Buscar comandos
- `Invalid(String)` - Comando inválido
- `NotHistoryCommand` - No es un comando del historial

### `HistoryCommandRepository`
Trait que define la interfaz para persistencia del historial.

#### Métodos:
- `save_history()` - Guarda el historial
- `load_history()` - Carga el historial
- `history_exists()` - Verifica si existe historial guardado

## Arquitectura

### Separación de Responsabilidades

1. **Capa de Presentación** (`gui_terminal.rs`):
   - Solo maneja la interfaz de usuario
   - Delega la lógica de comandos al caso de uso
   - No contiene lógica de negocio

2. **Capa de Aplicación** (`history_commands.rs`):
   - Contiene toda la lógica de negocio
   - Maneja el parsing de comandos
   - Coordina las operaciones del historial

3. **Capa de Dominio** (`command_history.rs` en texts):
   - Define las estructuras de datos
   - Contiene la lógica de formateo
   - No depende de otras capas

### Flujo de Ejecución

```
1. Usuario escribe comando
   ↓
2. GUI detecta comando
   ↓
3. Parser analiza comando
   ↓
4. Caso de uso ejecuta lógica
   ↓
5. Resultado se muestra en GUI
```

## Ventajas de esta Arquitectura

### 1. Testabilidad
```rust
#[test]
fn test_show_full_history() {
    let mut use_case = HistoryCommandsUseCase::new(100);
    use_case.add_command_entry("ls".to_string(), vec!["file.txt".to_string()], true, None);
    
    let result = use_case.show_full_history();
    assert!(result.contains(&"📜 Historial de Comandos:".to_string()));
}
```

### 2. Mantenibilidad
- Cada componente tiene una responsabilidad específica
- Fácil agregar nuevos comandos
- Lógica centralizada en un lugar

### 3. Reutilización
- El caso de uso puede ser usado por diferentes interfaces
- El parser puede ser reutilizado en otros contextos
- La lógica de negocio es independiente de la UI

### 4. Extensibilidad
- Fácil agregar nuevos tipos de comandos
- Soporte para diferentes formatos de salida
- Integración con diferentes repositorios

## Ejemplos de Uso

### Agregar un Nuevo Comando

```rust
// 1. Agregar nueva variante al enum
pub enum HistoryCommand {
    // ... otras variantes
    Export(String), // Nuevo comando para exportar
}

// 2. Actualizar el parser
impl HistoryCommandParser {
    pub fn parse_command(command: &str) -> HistoryCommand {
        match command.trim() {
            // ... otros casos
            cmd if cmd.starts_with("history -e ") => {
                let format = cmd.split_whitespace().nth(2).unwrap_or("txt").to_string();
                HistoryCommand::Export(format)
            }
            _ => HistoryCommand::NotHistoryCommand,
        }
    }
}

// 3. Implementar la lógica en el caso de uso
impl HistoryCommandsUseCase {
    pub fn export_history(&self, format: &str) -> Vec<String> {
        // Lógica de exportación
    }
}

// 4. Agregar al método execute
impl HistoryCommand {
    pub fn execute(self, use_case: &mut HistoryCommandsUseCase) -> Vec<String> {
        match self {
            // ... otros casos
            HistoryCommand::Export(format) => use_case.export_history(&format),
        }
    }
}
```

### Integración con Diferentes Interfaces

```rust
// Para una interfaz web
pub struct WebHistoryHandler {
    use_case: HistoryCommandsUseCase,
}

impl WebHistoryHandler {
    pub fn handle_request(&mut self, command: &str) -> Json {
        let history_command = HistoryCommandParser::parse_command(command);
        let result = history_command.execute(&mut self.use_case);
        Json::from(result)
    }
}

// Para una interfaz CLI
pub struct CliHistoryHandler {
    use_case: HistoryCommandsUseCase,
}

impl CliHistoryHandler {
    pub fn handle_command(&mut self, command: &str) {
        let history_command = HistoryCommandParser::parse_command(command);
        let result = history_command.execute(&mut self.use_case);
        for line in result {
            println!("{}", line);
        }
    }
}
```

## Patrones de Diseño Utilizados

### 1. Command Pattern
- Cada comando se encapsula en una variante del enum
- Permite fácil extensión y modificación

### 2. Strategy Pattern
- Diferentes estrategias de parsing y ejecución
- Fácil cambiar la implementación

### 3. Repository Pattern
- Abstracción para persistencia de datos
- Permite diferentes implementaciones (memoria, archivo, base de datos)

### 4. Use Case Pattern
- Encapsula la lógica de negocio
- Define claramente las operaciones disponibles

## Consideraciones de Rendimiento

### 1. Parsing Eficiente
- Los comandos se parsean una sola vez
- Uso de pattern matching para mejor rendimiento

### 2. Gestión de Memoria
- Límite configurable de entradas en el historial
- Eliminación automática de entradas antiguas

### 3. Búsqueda Optimizada
- Búsqueda case-insensitive
- Filtrado eficiente de resultados

## Próximos Pasos

### 1. Persistencia
- Implementar `FileHistoryRepository` para guardar en archivo
- Implementar `DatabaseHistoryRepository` para base de datos

### 2. Funcionalidades Avanzadas
- Búsqueda con expresiones regulares
- Filtros por fecha y estado
- Exportación a diferentes formatos

### 3. Testing
- Tests unitarios para cada componente
- Tests de integración
- Tests de rendimiento

### 4. Documentación
- Documentación de API
- Ejemplos de uso
- Guías de migración
