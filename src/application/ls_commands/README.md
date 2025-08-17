# Comando LS - Lógica de Negocio

## Descripción

Este módulo implementa la lógica de negocio para el comando `ls` (list directory contents) en la terminal Matrix. El comando permite listar archivos y directorios con diferentes opciones de visualización y ordenamiento.

## Estructura

### Entidades (Domain)

- **FileInfo**: Representa la información detallada de un archivo o directorio
- **LsResult**: Contiene el resultado completo del comando ls con estadísticas

### Caso de Uso (Application)

- **LsCommandsUseCase**: Implementa la lógica de negocio para listar directorios
- **LsCommandParser**: Parsea los comandos ls y sus opciones
- **LsCommand**: Enum que representa los diferentes tipos de comandos ls

### Opciones Soportadas

- `-a`: Mostrar archivos ocultos (que empiezan con punto)
- `-d`: Mostrar solo directorios
- `-f`: Mostrar solo archivos
- `-l`: Formato largo con detalles
- `-h`: Tamaños en formato legible (KB, MB, GB)
- `-S`: Ordenar por tamaño
- `-t`: Ordenar por fecha de modificación

## Funcionalidades

### 1. Listado Básico
```rust
let use_case = LsCommandsUseCase::new();
let result = use_case.list_directory(None)?;
```

### 2. Listado con Opciones
```rust
let mut options = LsOptions::default();
options.long_format = true;
options.show_hidden = true;
let result = use_case.list_directory_with_options(None, &options)?;
```

### 3. Parsing de Comandos
```rust
let command = "ls -la /ruta";
let parsed = LsCommandParser::parse_command(command);
let result = parsed.execute(&use_case)?;
```

## Información Recopilada

Para cada archivo/directorio se obtiene:

- **Nombre**: Nombre del archivo
- **Ruta**: Ruta completa
- **Tipo**: Archivo o directorio
- **Tamaño**: Tamaño en bytes
- **Fecha de modificación**: Timestamp de la última modificación
- **Permisos**: Permisos en formato Unix (rwxrwxrwx)
- **Propietario**: Usuario propietario
- **Grupo**: Grupo propietario

## Ordenamiento

El comando soporta tres tipos de ordenamiento:

1. **Por nombre** (alfabético, case-insensitive)
2. **Por tamaño** (de mayor a menor)
3. **Por fecha de modificación** (más reciente primero)

## Filtros

Se pueden aplicar filtros para mostrar:

- Solo archivos
- Solo directorios
- Incluir archivos ocultos
- Excluir archivos ocultos

## Manejo de Errores

El comando maneja los siguientes errores:

- Directorio no existe
- No es un directorio
- Error de permisos
- Error al leer el directorio
- Opciones inválidas

## Ejemplo de Uso

```rust
use crate::application::ls_commands::{LsCommandsUseCase, LsCommandParser};

// Crear instancia del caso de uso
let use_case = LsCommandsUseCase::new();

// Parsear y ejecutar comando
let command = "ls -lah";
let parsed = LsCommandParser::parse_command(command);
match parsed.execute(&use_case) {
    Ok(result) => {
        // Procesar resultado
        println!("Total de archivos: {}", result.files.len());
        for file in &result.files {
            println!("{} - {} bytes", file.name, file.size);
        }
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Integración con la Presentación

El resultado del comando ls se puede visualizar usando el componente `LsDisplayComponent` de la capa de presentación, que formatea la información en formato tabular con diferentes estilos de visualización.
