# Terminal Matrix en Rust

Una terminal con efectos visuales inspirada en la película "The Matrix", creada en Rust con arquitectura hexagonal limpia y principios SOLID.

## 🎯 Características

### Efectos Visuales
- **Efecto de lluvia digital**: Caracteres que caen desde la parte superior de la pantalla
- **Colores verde fosforescente**: Paleta de colores inspirada en Matrix
- **Caracteres aleatorios**: Los caracteres cambian dinámicamente mientras caen
- **Velocidades variables**: Cada "gota" de caracteres tiene su propia velocidad
- **Efectos de brillo**: Diferentes niveles de intensidad para crear profundidad visual

### Funcionalidades de Terminal
- **Modo Matrix**: Efectos visuales de lluvia digital
- **Modo Comando**: Terminal funcional con comandos del sistema
- **Historial de comandos**: Navegación con flechas arriba/abajo
- **Comandos internos**: ls, cd, pwd, clear, help, exit
- **Comandos del sistema**: Ejecución de cualquier comando del sistema operativo

## 🏗️ Arquitectura

El proyecto sigue **Clean Architecture** y **Arquitectura Hexagonal** con principios SOLID:

### Capas de la Arquitectura

#### 1. **Domain** (Dominio)
- **Entidades**: `Command`, `TerminalState`, `TerminalMode`
- **Repositorios**: `CommandRepository`, `FileSystemRepository`
- **Servicios**: `CommandService`, `FileSystemService`, `TerminalService`

#### 2. **Application** (Aplicación)
- **Casos de Uso**: `ExecuteCommandUseCase`, `HandleInputUseCase`
- Lógica de negocio y orquestación

#### 3. **Infrastructure** (Infraestructura)
- **Repositorios**: `SystemCommandRepository`, `SystemFileSystemRepository`
- Implementaciones concretas de acceso a datos

#### 4. **Presentation** (Presentación)
- **UI**: `TerminalUI`, `MatrixEffectManager`
- Interfaz de usuario y efectos visuales

### Principios SOLID Aplicados

- **S** - **Single Responsibility**: Cada clase tiene una responsabilidad única
- **O** - **Open/Closed**: Extensible sin modificar código existente
- **L** - **Liskov Substitution**: Repositorios intercambiables
- **I** - **Interface Segregation**: Interfaces específicas y cohesivas
- **D** - **Dependency Inversion**: Dependencias hacia abstracciones

## 🚀 Instalación y Uso

### Requisitos
- Rust 1.70 o superior
- Una terminal compatible con ANSI (Windows Terminal, PowerShell, CMD, etc.)

### Compilación
```bash
cargo build --release
```

### Ejecución
```bash
cargo run --release
```

## 🎮 Controles

### Modo Matrix
- **ENTER**: Cambiar a modo comando
- **ESC**: Salir de la aplicación
- **Q**: Salir de la aplicación

### Modo Comando
- **ENTER**: Ejecutar comando
- **ESC**: Volver al modo Matrix
- **↑/↓**: Navegar historial de comandos
- **←/→**: Mover cursor en línea de comando
- **BACKSPACE**: Borrar caracteres

## 📋 Comandos Disponibles

### Comandos Internos
- `ls` o `dir`: Listar archivos y directorios
- `cd <directorio>`: Cambiar directorio
- `pwd`: Mostrar directorio actual
- `clear`: Limpiar pantalla
- `help`: Mostrar ayuda
- `exit` o `quit`: Salir de la terminal

### Comandos del Sistema
- Cualquier comando del sistema operativo (ej: `echo`, `date`, etc.)

## 🛠️ Tecnologías Utilizadas

- **crossterm**: Manejo de terminal multiplataforma
- **rand**: Generación de números aleatorios
- **tokio**: Manejo asíncrono (preparado para futuras mejoras)
- **chrono**: Manejo de tiempo (preparado para futuras mejoras)

## 📁 Estructura del Proyecto

```
src/
├── domain/           # Lógica de negocio
│   ├── entities.rs   # Entidades del dominio
│   ├── repositories.rs # Interfaces de repositorios
│   └── services.rs   # Servicios del dominio
├── application/      # Casos de uso
│   └── use_cases.rs  # Lógica de aplicación
├── infrastructure/   # Implementaciones concretas
│   └── repositories.rs # Repositorios del sistema
├── presentation/     # Interfaz de usuario
│   ├── matrix_effects.rs # Efectos visuales
│   └── terminal_ui.rs # Interfaz de terminal
└── main.rs          # Punto de entrada
```

## 🔧 Personalización

### Efectos Matrix
- `MATRIX_CHARS`: Caracteres que aparecen en la lluvia
- `MATRIX_COLORS`: Paleta de colores
- Velocidades y frecuencias de las gotas

### Comandos
- Agregar nuevos comandos internos en `SystemCommandRepository`
- Implementar nuevos repositorios siguiendo las interfaces del dominio

## 🎨 Características Técnicas

- **Modo Raw**: Captura de eventos de teclado en tiempo real
- **Pantalla Alternativa**: Experiencia inmersiva
- **Gestión de Estado**: Estado centralizado y consistente
- **Inyección de Dependencias**: Acoplamiento débil entre capas
- **Manejo de Errores**: Gestión robusta de errores en todas las capas

## 🔮 Futuras Mejoras

- [ ] Efectos de sonido
- [ ] Más comandos internos
- [ ] Configuración persistente
- [ ] Temas personalizables
- [ ] Scripts y alias
- [ ] Autocompletado
- [ ] Soporte para plugins

## 📝 Notas

- La terminal utiliza modo raw para capturar eventos de teclado
- Se ejecuta en pantalla alternativa para una experiencia inmersiva
- Compatible con Windows, macOS y Linux
- Arquitectura preparada para testing y extensibilidad
