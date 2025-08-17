# Comando LS - Capa de Presentación

## Descripción

Este módulo implementa la capa de presentación para el comando `ls`, encargándose de formatear y visualizar la información de archivos y directorios de manera atractiva y organizada.

## Componentes

### LsDisplayFormatter

Formateador principal que convierte los datos del comando ls en texto formateado.

#### Funcionalidades

- **Formato Simple**: Lista básica de archivos con iconos
- **Formato Largo**: Tabla detallada con permisos, propietario, tamaño, etc.
- **Estadísticas**: Resumen del directorio con totales
- **Formato de Tamaños**: Conversión a unidades legibles (KB, MB, GB)
- **Formato de Fechas**: Conversión de timestamps a formato legible

### LsDisplayComponent

Componente de alto nivel que coordina la visualización.

#### Métodos Principales

- `render()`: Renderiza el resultado completo del comando ls
- `render_error()`: Muestra errores de forma amigable
- `render_help()`: Muestra la ayuda del comando
- `render_preview()`: Muestra una vista previa del directorio

## Formatos de Visualización

### 1. Formato Simple

```
📁 Directorio: /ruta/actual

📁 carpeta1  📁 carpeta2  📄 archivo1.txt  📄 archivo2.rs
📄 documento.pdf  📄 imagen.png
```

### 2. Formato Largo (Detallado)

```
📁 Directorio: /ruta/actual

📊 Total: 15 elementos
📁 Directorios: 3
📄 Archivos: 12
💾 Tamaño total: 2.5 MB

Permisos     Propietario Grupo     Tamaño       Modificado           Nombre
------------ -------- -------- ------------ -------------------- ----
drwxr-xr-x   usuario  grupo    0 B          25/12/2023 14:30    📁 carpeta1
-rw-r--r--   usuario  grupo    1.2 KB       24/12/2023 10:15    📄 archivo1.txt
-rw-r--r--   usuario  grupo    45.7 KB      23/12/2023 16:45    📄 imagen.png
```

### 3. Vista Previa

```
📁 Vista previa de: /ruta/actual

📁 Directorios:
   📁 carpeta1
   📁 carpeta2
   📁 carpeta3
   ... y 2 más

📄 Archivos:
   📄 archivo1.txt
   📄 archivo2.rs
   📄 documento.pdf
   ... y 9 más
```

## Características de Visualización

### Iconos
- 📁 Para directorios
- 📄 Para archivos
- 📊 Para estadísticas
- 💾 Para información de tamaño
- ❌ Para errores
- 📖 Para ayuda

### Colores y Formato
- Uso de emojis para mejor identificación visual
- Formato tabular alineado para mejor legibilidad
- Separadores visuales entre secciones
- Información organizada jerárquicamente

### Responsive Design
- Adaptación automática al ancho de la terminal
- Múltiples archivos por línea en formato simple
- Saltos de línea automáticos para evitar desbordamiento

## Opciones de Formato

### Tamaños Legibles
- Conversión automática de bytes a KB, MB, GB
- Formato: "1.2 KB", "45.7 MB", "2.1 GB"

### Fechas Formateadas
- Formato: "dd/mm/yyyy HH:MM"
- Conversión de timestamps Unix a formato legible

### Permisos Unix
- Formato estándar: "drwxr-xr-x"
- Identificación visual del tipo de archivo

## Integración con la Lógica de Negocio

```rust
use crate::application::ls_commands::{LsCommandsUseCase, LsCommandParser};
use crate::presentation::commands::LsDisplayComponent;

// Ejecutar comando
let use_case = LsCommandsUseCase::new();
let command = "ls -lah";
let parsed = LsCommandParser::parse_command(command);

match parsed.execute(&use_case) {
    Ok(result) => {
        // Renderizar resultado
        let output = LsDisplayComponent::render(&result, &options);
        for line in output {
            println!("{}", line);
        }
    }
    Err(e) => {
        // Renderizar error
        let error_output = LsDisplayComponent::render_error(&e);
        for line in error_output {
            eprintln!("{}", line);
        }
    }
}
```

## Personalización

### Modificar Iconos
Los iconos se pueden personalizar modificando las constantes en el formateador:

```rust
const DIR_ICON: &str = "📁";
const FILE_ICON: &str = "📄";
const ERROR_ICON: &str = "❌";
```

### Cambiar Formato de Fechas
El formato de fechas se puede modificar en el método `format_date()`:

```rust
dt.format("%Y-%m-%d %H:%M:%S").to_string()  // Formato ISO
dt.format("%b %d %H:%M").to_string()        // Formato compacto
```

### Ajustar Ancho de Tabla
El ancho de las columnas se puede ajustar modificando los especificadores de formato:

```rust
format!("{:<15} {:<10} {:<10} {:<15} {:<25} {:<}", ...)
```

## Ejemplos de Uso

### Mostrar Ayuda
```rust
let help_lines = LsDisplayComponent::render_help();
for line in help_lines {
    println!("{}", line);
}
```

### Vista Previa Rápida
```rust
let preview_lines = LsExample::show_preview();
for line in preview_lines {
    println!("{}", line);
}
```

### Ejecutar Comando Específico
```rust
let output = LsExample::execute_ls_command("ls -la");
for line in output {
    println!("{}", line);
}
```

## Consideraciones de Rendimiento

- El formateo se realiza de manera eficiente sin crear objetos innecesarios
- Las cadenas se construyen usando `String` y `format!()` para mejor rendimiento
- Los cálculos de tamaño se realizan una sola vez por archivo
- La conversión de fechas se optimiza usando `chrono`

## Extensibilidad

El sistema está diseñado para ser fácilmente extensible:

- Nuevos formatos de visualización se pueden agregar como métodos adicionales
- Los iconos y estilos se pueden personalizar sin modificar la lógica principal
- Nuevas opciones de formato se pueden integrar fácilmente
- El sistema de componentes permite agregar nuevas funcionalidades de visualización
