# Manejo de Errores para Comandos No Encontrados

## Descripción

Se ha implementado un sistema de manejo de errores específico para cuando un comando no se puede encontrar en el sistema. Esto mejora la experiencia del usuario al proporcionar mensajes de error más claros y descriptivos.

## Implementación

### 1. Mensajes de Error Específicos

Se agregó un nuevo método en `ErrorMessages` para formatear errores de comandos no encontrados:

```rust
pub fn format_command_not_found(command_name: &str) -> String {
    format!("Error: El comando '{}' no se pudo encontrar", command_name)
}
```

### 2. Detección de Comandos No Encontrados

En el `SystemCommandRepository`, se modificó la lógica para detectar específicamente cuando un comando del sistema no se encuentra:

```rust
Err(e) => {
    // Detectar específicamente si el comando no se encuentra
    let error_msg = if e.kind() == std::io::ErrorKind::NotFound {
        format!("El comando '{}' no se pudo encontrar", command.name)
    } else {
        format!("Error: {}", e)
    };
    
    Ok(command.clone()
        .with_output(error_msg)
        .with_exit_code(1))
}
```

## Flujo de Ejecución

1. **Entrada del Usuario**: El usuario ingresa un comando que no existe
2. **Análisis del Comando**: El sistema intenta ejecutar el comando del sistema
3. **Detección de Error**: Se detecta que el comando no se encuentra (`std::io::ErrorKind::NotFound`)
4. **Mensaje de Error**: Se genera un mensaje específico: "El comando 'nombre_comando' no se pudo encontrar"
5. **Visualización**: El mensaje se muestra en la terminal con el formato de error estándar

## Ejemplo de Uso

Cuando el usuario ingresa un comando que no existe:

```
usuario@terminal:~$ comando_inexistente
Error: El comando 'comando_inexistente' no se pudo encontrar
```

## Beneficios

- **Mensajes Claros**: Los usuarios reciben información específica sobre qué comando no se encontró
- **Mejor UX**: La experiencia del usuario es más amigable con mensajes descriptivos
- **Consistencia**: Se mantiene el formato estándar de errores del sistema
- **Debugging**: Facilita la identificación de problemas para desarrolladores

## Archivos Modificados

- `src/presentation/texts/error_messages.rs`: Agregado método para comandos no encontrados
- `src/infrastructure/repositories.rs`: Implementada detección específica de errores de comandos no encontrados

## Pruebas

Se realizaron pruebas para verificar que:
- El sistema detecta correctamente cuando un comando no existe
- Se genera el mensaje de error apropiado
- El código compila sin errores
- La funcionalidad se integra correctamente con el flujo existente
