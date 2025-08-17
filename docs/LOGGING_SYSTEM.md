# Sistema de Logs - Termirust

## Descripción

El sistema de logs de Termirust permite registrar y monitorear todas las actividades de la aplicación, facilitando la detección y resolución de errores. Cada ejecución genera un archivo de log único con timestamp.

## Características

### 📁 Estructura de Archivos
- **Ubicación**: Misma carpeta que el ejecutable de la aplicación
- **Directorio**: `logs/` (dentro del directorio del ejecutable)
- **Formato de archivo**: `termirust_YYYYMMDD_HHMMSS.log`
- **Ejemplo**: `termirust_20241201_143052.log`
- **Ruta completa**: `C:\ruta\al\ejecutable\logs\termirust_20241201_143052.log`

### 📊 Niveles de Log
- **ERROR**: Errores críticos que afectan la funcionalidad
- **WARN**: Advertencias que no impiden la ejecución
- **INFO**: Información general de la aplicación
- **DEBUG**: Información detallada para debugging

### 🔍 Eventos Registrados

#### Inicio y Cierre de Aplicación
```
[2024-12-01 14:30:52.123] [INFO] [termirust] === APLICACIÓN TERMIRUST INICIADA ===
[2024-12-01 14:30:52.124] [INFO] [termirust] Versión: 0.1.0
[2024-12-01 14:30:52.125] [INFO] [termirust] Timestamp de inicio: 2024-12-01 14:30:52
```

#### Ejecución de Comandos
```
[2024-12-01 14:31:15.456] [INFO] [termirust] Comando ejecutado exitosamente: 'ls' - Salida: archivo1.txt archivo2.txt
[2024-12-01 14:31:20.789] [ERROR] [termirust] Comando falló: 'comando_inexistente' - Error: comando no encontrado
```

#### Comandos del Historial
```
[2024-12-01 14:32:10.123] [INFO] [termirust] Ejecutando comando del historial: '!history'
```

#### Estadísticas del Sistema
```
[2024-12-01 14:33:00.456] [DEBUG] [termirust] Estadísticas del sistema - Memoria: 45.2%, CPU: 23.1%, Red: 15MB recibidos, 8MB transmitidos
```

#### Entrada de Usuario
```
[2024-12-01 14:33:30.123] [DEBUG] [termirust] Procesando 3 eventos de entrada
```

## Uso del Sistema

### Inicialización Automática
El sistema de logs se inicializa automáticamente al iniciar la aplicación:

```rust
// En main.rs
if let Err(e) = Logger::init() {
    eprintln!("Error inicializando el sistema de logs: {}", e);
}
log_application_start();
```

### Funciones de Logging Disponibles

#### Logging Básico
```rust
use crate::infrastructure::logging::{log_info, log_error, log_warning, log_debug};

log_info("Mensaje informativo");
log_error("Mensaje de error");
log_warning("Mensaje de advertencia");
log_debug("Mensaje de debug");
```

#### Logging Específico
```rust
use crate::infrastructure::logging::{
    log_command_execution,
    log_system_stats,
    log_matrix_effect_update,
    log_input_handling
};

// Log de comandos
log_command_execution("ls", true, "archivo1.txt archivo2.txt");
log_command_execution("comando_erroneo", false, "comando no encontrado");

// Log de estadísticas del sistema
log_system_stats(45.2, 23.1, (15728640, 8388608)); // memoria%, cpu%, (recibidos, transmitidos)

// Log de efectos Matrix
log_matrix_effect_update(15); // número de gotas activas

// Log de entrada de usuario
log_input_handling("ls", true);
```

## Configuración

### Nivel de Log
El nivel de log se puede configurar en `src/infrastructure/logging.rs`:

```rust
Ok(Logger {
    file,
    level: LevelFilter::Debug, // Cambiar aquí: Error, Warn, Info, Debug
})
```

### Frecuencia de Logging
- **Estadísticas del sistema**: Cada 60 segundos
- **Efectos Matrix**: Cada 100 frames
- **Comandos**: En cada ejecución
- **Entrada de usuario**: En cada evento de teclado

## Análisis de Logs

### Búsqueda de Errores
```bash
# Buscar todos los errores (desde el directorio del ejecutable)
grep "ERROR" logs/termirust_*.log

# Buscar errores de comandos específicos
grep "Comando falló" logs/termirust_*.log

# Buscar por fecha
grep "2024-12-01" logs/termirust_*.log
```

### Monitoreo de Rendimiento
```bash
# Ver estadísticas del sistema
grep "Estadísticas del sistema" logs/termirust_*.log

# Ver uso de memoria alto
grep "Memoria: [8-9][0-9]" logs/termirust_*.log

# Ver uso de CPU alto
grep "CPU: [8-9][0-9]" logs/termirust_*.log
```

### Análisis de Comandos
```bash
# Ver comandos más ejecutados
grep "Comando ejecutado exitosamente" logs/termirust_*.log | cut -d"'" -f2 | sort | uniq -c | sort -nr

# Ver comandos que fallan más
grep "Comando falló" logs/termirust_*.log | cut -d"'" -f2 | sort | uniq -c | sort -nr
```

## Mantenimiento

### Limpieza de Logs Antiguos
Se recomienda limpiar logs antiguos periódicamente:

```bash
# Eliminar logs de más de 30 días (desde el directorio del ejecutable)
find logs/ -name "termirust_*.log" -mtime +30 -delete

# Comprimir logs antiguos
find logs/ -name "termirust_*.log" -mtime +7 -exec gzip {} \;

# En Windows (PowerShell)
Get-ChildItem logs\termirust_*.log | Where-Object {$_.LastWriteTime -lt (Get-Date).AddDays(-30)} | Remove-Item
```

### Rotación de Logs
El sistema actual crea un archivo por ejecución. Para implementar rotación automática, se puede modificar el logger para:

1. Limitar el tamaño de archivo
2. Rotar automáticamente cuando se alcance el límite
3. Mantener un número máximo de archivos

## Troubleshooting

### Problemas Comunes

#### Error al crear directorio de logs
```
Error inicializando el sistema de logs: Permission denied
```
**Solución**: Verificar permisos de escritura en el directorio del proyecto.

#### Archivo de log no se crea
**Verificar**:
1. Que el directorio `logs/` existe
2. Que hay permisos de escritura
3. Que no hay errores en la inicialización

#### Logs no aparecen
**Verificar**:
1. Que el nivel de log no es demasiado restrictivo
2. Que los eventos se están registrando correctamente
3. Que el archivo se está escribiendo (verificar tamaño)

### Debugging del Sistema de Logs
Para debuggear el propio sistema de logs, agregar logs adicionales:

```rust
log::error!("Error en el sistema de logs: {}", error);
eprintln!("Error crítico del sistema de logs: {}", error);
```

## Extensiones Futuras

### Funcionalidades Planificadas
1. **Logging asíncrono**: Para mejorar rendimiento
2. **Compresión automática**: Para ahorrar espacio
3. **Filtros avanzados**: Por tipo de evento, usuario, etc.
4. **Integración con sistemas externos**: Envío a servidores de logs
5. **Dashboard de logs**: Interfaz web para visualizar logs
6. **Alertas automáticas**: Notificaciones por errores críticos

### Configuración Avanzada
```rust
pub struct LoggerConfig {
    pub max_file_size: usize,
    pub max_files: usize,
    pub compression_enabled: bool,
    pub async_logging: bool,
    pub external_logging: Option<String>,
}
```
