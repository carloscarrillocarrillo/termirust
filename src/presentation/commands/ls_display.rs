use crate::domain::entities::{FileInfo, LsResult};
use crate::application::ls_commands::LsOptions;
use std::time::{SystemTime, UNIX_EPOCH};

/// Formateador para la salida del comando ls
pub struct LsDisplayFormatter;

impl LsDisplayFormatter {
    /// Formatea el resultado del comando ls en formato tabular
    pub fn format_ls_result(result: &LsResult, options: &LsOptions) -> Vec<String> {
        let mut output = Vec::new();

        // Agregar encabezado del directorio
        output.push(format!("📁 Directorio: {}", result.current_directory));
        output.push(String::new());

        if result.files.is_empty() {
            output.push("📭 El directorio está vacío".to_string());
            return output;
        }

        // Agregar estadísticas
        if options.long_format {
            output.extend(Self::format_statistics(result));
            output.push(String::new());
        }

        // Formatear la lista de archivos
        if options.long_format {
            output.extend(Self::format_long_listing(result, options));
        } else {
            output.extend(Self::format_simple_listing(result));
        }

        output
    }

    /// Formatea las estadísticas del directorio
    fn format_statistics(result: &LsResult) -> Vec<String> {
        vec![
            format!("📊 Total: {} elementos", result.files.len()),
            format!("📁 Directorios: {}", result.total_directories),
            format!("📄 Archivos: {}", result.total_files),
            format!("💾 Tamaño total: {}", Self::format_size(result.total_size)),
        ]
    }

    /// Formatea la lista en formato simple (sin detalles)
    fn format_simple_listing(result: &LsResult) -> Vec<String> {
        let mut output = Vec::new();
        let mut current_line = String::new();
        let max_width = 80; // Ancho máximo de la terminal
        let mut current_width = 0;

        for file in &result.files {
            let display_name = if file.is_directory {
                format!("📁 {}", file.name)
            } else {
                format!("📄 {}", file.name)
            };

            let name_width = display_name.chars().count();
            
            if current_width + name_width + 2 > max_width {
                output.push(current_line);
                current_line = display_name;
                current_width = name_width;
            } else {
                if !current_line.is_empty() {
                    current_line.push_str("  ");
                    current_width += 2;
                }
                current_line.push_str(&display_name);
                current_width += name_width;
            }
        }

        if !current_line.is_empty() {
            output.push(current_line);
        }

        output
    }

    /// Formatea la lista en formato largo (con detalles)
    fn format_long_listing(result: &LsResult, options: &LsOptions) -> Vec<String> {
        let mut output = Vec::new();

        // Encabezado de la tabla
        let header = format!(
            "{:<12} {:<8} {:<8} {:<12} {:<20} {:<}",
            "Permisos", "Propietario", "Grupo", "Tamaño", "Modificado", "Nombre"
        );
        output.push(header.clone());
        output.push("-".repeat(header.len()));

        // Filas de datos
        for file in &result.files {
            let row = Self::format_file_row(file, options);
            output.push(row);
        }

        output
    }

    /// Formatea una fila individual de archivo
    fn format_file_row(file: &FileInfo, options: &LsOptions) -> String {
        let permissions = &file.permissions;
        let owner = &file.owner;
        let group = &file.group;
        let size = if options.human_readable {
            Self::format_size(file.size)
        } else {
            file.size.to_string()
        };
        let modified = Self::format_date(&file.modified);
        let name = if file.is_directory {
            format!("📁 {}", file.name)
        } else {
            format!("📄 {}", file.name)
        };

        format!(
            "{:<12} {:<8} {:<8} {:<12} {:<20} {:<}",
            permissions, owner, group, size, modified, name
        )
    }

    /// Formatea el tamaño en formato legible
    fn format_size(size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        match size {
            0..KB => format!("{} B", size),
            KB..MB => format!("{:.1} KB", size as f64 / KB as f64),
            MB..GB => format!("{:.1} MB", size as f64 / MB as f64),
            _ => format!("{:.1} GB", size as f64 / GB as f64),
        }
    }

    /// Formatea la fecha de modificación
    fn format_date(system_time: &SystemTime) -> String {
        if let Ok(duration) = system_time.duration_since(UNIX_EPOCH) {
            let timestamp = duration.as_secs();
            let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0);
            
            if let Some(dt) = datetime {
                dt.format("%d/%m/%Y %H:%M").to_string()
            } else {
                "Fecha inválida".to_string()
            }
        } else {
            "Fecha inválida".to_string()
        }
    }

    /// Formatea un mensaje de error
    pub fn format_error(error: &str) -> Vec<String> {
        vec![
            "❌ Error en comando ls:".to_string(),
            format!("   {}", error),
        ]
    }

    /// Formatea la ayuda del comando ls
    #[allow(dead_code)]
    pub fn format_help() -> Vec<String> {
        vec![
            "📖 Ayuda del comando ls:".to_string(),
            String::new(),
            "Uso: ls [opciones] [directorio]".to_string(),
            String::new(),
            "Opciones:".to_string(),
            "  -a    Mostrar archivos ocultos".to_string(),
            "  -d    Mostrar solo directorios".to_string(),
            "  -f    Mostrar solo archivos".to_string(),
            "  -l    Formato largo (detallado)".to_string(),
            "  -h    Tamaños en formato legible".to_string(),
            "  -S    Ordenar por tamaño".to_string(),
            "  -t    Ordenar por fecha de modificación".to_string(),
            String::new(),
            "Ejemplos:".to_string(),
            "  ls                    # Listar archivos del directorio actual".to_string(),
            "  ls -la               # Listar todos los archivos con detalles".to_string(),
            "  ls -lh /ruta         # Listar con tamaños legibles".to_string(),
            "  ls -t                # Ordenar por fecha de modificación".to_string(),
        ]
    }
}

/// Componente de visualización para el comando ls
pub struct LsDisplayComponent;

impl LsDisplayComponent {
    /// Renderiza el resultado del comando ls
    pub fn render(result: &LsResult, options: &LsOptions) -> Vec<String> {
        LsDisplayFormatter::format_ls_result(result, options)
    }

    /// Renderiza un error
    pub fn render_error(error: &str) -> Vec<String> {
        LsDisplayFormatter::format_error(error)
    }

    /// Renderiza la ayuda
    #[allow(dead_code)]
    pub fn render_help() -> Vec<String> {
        LsDisplayFormatter::format_help()
    }

    /// Renderiza una vista previa del directorio
    #[allow(dead_code)]
    pub fn render_preview(result: &LsResult) -> Vec<String> {
        let mut output = Vec::new();
        output.push(format!("📁 Vista previa de: {}", result.current_directory));
        output.push(String::new());

        let mut directories = Vec::new();
        let mut files = Vec::new();

        for file in &result.files {
            if file.is_directory {
                directories.push(&file.name);
            } else {
                files.push(&file.name);
            }
        }

        if !directories.is_empty() {
            output.push("📁 Directorios:".to_string());
            for dir in directories.iter().take(5) {
                output.push(format!("   📁 {}", dir));
            }
            if directories.len() > 5 {
                output.push(format!("   ... y {} más", directories.len() - 5));
            }
            output.push(String::new());
        }

        if !files.is_empty() {
            output.push("📄 Archivos:".to_string());
            for file in files.iter().take(5) {
                output.push(format!("   📄 {}", file));
            }
            if files.len() > 5 {
                output.push(format!("   ... y {} más", files.len() - 5));
            }
        }

        output
    }
}
