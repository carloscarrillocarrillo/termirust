# Guía de Comandos del Historial - Termirust

Este documento describe los comandos especiales disponibles para gestionar el historial de comandos en Termirust.

## Comandos Disponibles

### `history` o `hist`
Muestra todo el historial de comandos ejecutados.

**Ejemplo:**
```bash
history
```

**Salida:**
```
📜 Historial de Comandos:
[1703123456] ✅ ls -la
  archivo1.txt
  archivo2.txt

[1703123460] ❌ invalid_command
  Error: Comando no encontrado
```

### `history -n 10` o `hist -n 10`
Muestra los últimos 10 comandos del historial.

**Ejemplo:**
```bash
history -n 10
```

### `history -c` o `hist -c`
Limpia todo el historial de comandos.

**Ejemplo:**
```bash
history -c
```

**Salida:**
```
🗑️ Historial limpiado
```

### `history -s` o `hist -s`
Muestra estadísticas del historial de comandos.

**Ejemplo:**
```bash
history -s
```

**Salida:**
```
📊 Estadísticas del Historial:
  Total de comandos: 25
  Comandos exitosos: 22 ✅
  Comandos fallidos: 3 ❌
  Tasa de éxito: 88.0%
```

### `history -g <patrón>` o `hist -g <patrón>`
Busca comandos en el historial que coincidan con el patrón especificado.

**Ejemplos:**
```bash
history -g ls
history -g "git"
history -g "python"
```

**Salida:**
```
🔍 Resultados para 'ls':
[1703123456] ✅ ls -la
  archivo1.txt
  archivo2.txt

[1703123500] ✅ ls -l
  directorio1/
  directorio2/
```

## Características del Historial

### Almacenamiento
- **Límite**: Hasta 100 comandos (configurable)
- **Persistencia**: Se mantiene durante la sesión actual
- **Timestamps**: Cada comando incluye marca de tiempo

### Información Registrada
- **Comando**: El comando ejecutado
- **Salida**: Resultados del comando
- **Estado**: Éxito (✅) o fallo (❌)
- **Error**: Mensaje de error si falló
- **Timestamp**: Momento de ejecución

### Visualización
- **Colores**: Diferentes colores para éxitos y fallos
- **Formato**: Estructurado y fácil de leer
- **Estadísticas**: Información resumida en tiempo real

## Integración con la Interfaz

### Estadísticas en Tiempo Real
Las estadísticas del historial se muestran automáticamente en la esquina inferior izquierda de la pantalla:
- Total de comandos ejecutados
- Número de éxitos y fallos
- Tasa de éxito porcentual

### Colores de Indicadores
- **Verde**: Comandos exitosos
- **Rojo**: Comandos fallidos
- **Amarillo**: Títulos y encabezados
- **Gris**: Información general

## Casos de Uso

### Desarrollo y Debugging
```bash
# Ver comandos recientes para debugging
history -n 5

# Buscar comandos específicos
history -g "cargo"

# Ver estadísticas de éxito
history -s
```

### Administración del Sistema
```bash
# Ver comandos de administración
history -g "sudo"

# Limpiar historial sensible
history -c
```

### Análisis de Productividad
```bash
# Ver tasa de éxito general
history -s

# Buscar comandos que fallaron
history -g "Error"
```

## Notas Técnicas

### Rendimiento
- El historial se mantiene en memoria
- Búsquedas son case-insensitive
- Límite automático para evitar uso excesivo de memoria

### Compatibilidad
- Los comandos del historial no interfieren con comandos del sistema
- Se pueden usar junto con cualquier comando normal
- No afectan el funcionamiento del terminal

### Personalización
- El límite de comandos se puede ajustar en el código
- Los colores y formato se pueden personalizar
- Se pueden agregar nuevos comandos especiales
