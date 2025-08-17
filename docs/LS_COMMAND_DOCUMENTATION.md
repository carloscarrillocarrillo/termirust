# Comando LS - Documentación Completa

## Resumen

El comando `ls` ha sido implementado siguiendo la arquitectura de Clean Architecture, separando la lógica de negocio (capa de aplicación) de la presentación (capa de presentación). El comando permite listar archivos y directorios con información detallada y múltiples opciones de visualización.

## Arquitectura Implementada

### Capa de Dominio (`src/domain/entities.rs`)
- **FileInfo**: Entidad que representa la información detallada de un archivo
- **LsResult**: Entidad que contiene el resultado completo del comando ls

### Capa de Aplicación (`src/application/ls_commands.rs`)
- **LsCommandsUseCase**: Caso de uso principal que implementa la lógica de negocio
- **LsCommandParser**: Parser que interpreta los comandos ls y sus opciones
- **LsCommand**: Enum que representa los diferentes tipos de comandos
- **LsOptions**: Estructura que define las opciones del comando

### Capa de Presentación (`src/presentation/commands/`)
- **LsDisplayFormatter**: Formateador que convierte datos en texto visual
- **LsDisplayComponent**: Componente de alto nivel para la visualización
- **LsExample**: Ejemplos de uso del comando

## Funcionalidades Implementadas

### ✅ Lógica de Negocio
- [x] Lectura de directorios del sistema de archivos
- [x] Obtención de metadatos detallados (tamaño, fecha, permisos)
- [x] Filtrado por tipo de archivo (archivos/directorios)
- [x] Filtrado por archivos ocultos
- [x] Ordenamiento por nombre, tamaño y fecha
- [x] Manejo de errores robusto
- [x] Parsing de comandos y opciones

### ✅ Visualización Tabulada
- [x] Formato simple con iconos
- [x] Formato largo con tabla detallada
- [x] Estadísticas del directorio
- [x] Tamaños en formato legible (KB, MB, GB)
- [x] Fechas formateadas
- [x] Permisos en formato Unix
- [x] Vista previa del directorio

### ✅ Opciones Soportadas
- [x] `-a`: Mostrar archivos ocultos
- [x] `-d`: Mostrar solo directorios
- [x] `-f`: Mostrar solo archivos
- [x] `-l`: Formato largo
- [x] `-h`: Tamaños legibles
- [x] `-S`: Ordenar por tamaño
- [x] `-t`: Ordenar por fecha

## Estructura de Archivos

```
src/
├── domain/
│   └── entities.rs              # Entidades FileInfo y LsResult
├── application/
│   ├── ls_commands.rs           # Lógica de negocio del comando ls
│   └── mod.rs                   # Exporta el módulo ls_commands
└── presentation/
    └── commands/
        ├── ls_display.rs        # Formateo y visualización
        ├── ls_example.rs        # Ejemplos de uso
        └── mod.rs               # Exporta los módulos
```

## Ejemplo de Uso Completo

```rust
use crate::application::ls_commands::{LsCommandsUseCase, LsCommandParser};
use crate::presentation::commands::LsDisplayComponent;

fn main() {
    // 1. Crear instancia del caso de uso
    let use_case = LsCommandsUseCase::new();
    
    // 2. Parsear comando
    let command = "ls -lah";
    let parsed = LsCommandParser::parse_command(command);
    
    // 3. Ejecutar comando
    match parsed.execute(&use_case) {
        Ok(result) => {
            // 4. Renderizar resultado
            let mut options = crate::application::ls_commands::LsOptions::default();
            options.long_format = true;
            options.human_readable = true;
            
            let output = LsDisplayComponent::render(&result, &options);
            for line in output {
                println!("{}", line);
            }
        }
        Err(e) => {
            let error_output = LsDisplayComponent::render_error(&e);
            for line in error_output {
                eprintln!("{}", line);
            }
        }
    }
}
```

## Salida de Ejemplo

### Comando: `ls -lah`

```
📁 Directorio: /d:/termirust

📊 Total: 25 elementos
📁 Directorios: 8
📄 Archivos: 17
💾 Tamaño total: 1.2 MB

Permisos     Propietario Grupo     Tamaño       Modificado           Nombre
------------ -------- -------- ------------ -------------------- ----
drwxr-xr-x   usuario  grupo    0 B          25/12/2023 14:30    📁 src
-rw-r--r--   usuario  grupo    1.2 KB       24/12/2023 10:15    📄 Cargo.toml
-rw-r--r--   usuario  grupo    45.7 KB      23/12/2023 16:45    📄 README.md
drwxr-xr-x   usuario  grupo    0 B          22/12/2023 09:20    📁 target
-rw-r--r--   usuario  grupo    2.1 KB       21/12/2023 11:30    📄 build.rs
```

## Características Técnicas

### Información Recopilada por Archivo
- ✅ Nombre del archivo
- ✅ Ruta completa
- ✅ Tipo (archivo/directorio)
- ✅ Tamaño en bytes
- ✅ Fecha de última modificación
- ✅ Permisos Unix
- ✅ Propietario (simulado en Windows)
- ✅ Grupo (simulado en Windows)

### Ordenamiento Implementado
- ✅ Alfabético (case-insensitive)
- ✅ Por tamaño (descendente)
- ✅ Por fecha de modificación (más reciente primero)

### Filtros Disponibles
- ✅ Solo archivos
- ✅ Solo directorios
- ✅ Incluir/excluir archivos ocultos

### Formato de Visualización
- ✅ Iconos para mejor identificación
- ✅ Tabla alineada para legibilidad
- ✅ Tamaños en unidades legibles
- ✅ Fechas en formato dd/mm/yyyy HH:MM
- ✅ Permisos en formato Unix estándar

## Manejo de Errores

El comando maneja los siguientes errores de forma elegante:

- ❌ Directorio no existe
- ❌ No es un directorio
- ❌ Error de permisos
- ❌ Error al leer el directorio
- ❌ Opciones inválidas
- ❌ Demasiados argumentos

## Integración con el Sistema

### Dependencias
- `std::fs`: Para operaciones de archivos
- `std::path`: Para manejo de rutas
- `std::os::unix::fs::PermissionsExt`: Para permisos Unix
- `chrono`: Para formateo de fechas

### Compatibilidad
- ✅ Windows (con limitaciones en permisos Unix)
- ✅ Linux/Unix
- ✅ macOS

## Próximas Mejoras

### Funcionalidades Adicionales
- [ ] Soporte para colores en la terminal
- [ ] Filtros por extensión de archivo
- [ ] Búsqueda de archivos por patrón
- [ ] Información de enlaces simbólicos
- [ ] Estadísticas más detalladas

### Optimizaciones
- [ ] Paginación para directorios grandes
- [ ] Caché de metadatos
- [ ] Procesamiento asíncrono
- [ ] Compresión de salida

## Conclusión

El comando `ls` ha sido implementado exitosamente siguiendo los principios de Clean Architecture, proporcionando una separación clara entre la lógica de negocio y la presentación. La implementación es robusta, extensible y proporciona una experiencia de usuario rica con múltiples opciones de visualización.

La arquitectura permite fácil mantenimiento y extensión, mientras que la capa de presentación ofrece una visualización atractiva y funcional de la información del sistema de archivos.
