use std::fs;
use std::path::{Path, PathBuf};
use crate::domain::entities::{FileInfo, LsResult};

/// Caso de uso para el comando ls
pub struct LsCommandsUseCase;

impl LsCommandsUseCase {
    /// Crea una nueva instancia del caso de uso
    pub fn new() -> Self {
        Self
    }

    /// Ejecuta el comando ls básico - lista archivos y directorios
    pub fn list_directory(&self, path: Option<&str>) -> Result<LsResult, String> {
        let target_path = match path {
            Some(p) => PathBuf::from(p),
            None => std::env::current_dir().map_err(|e| format!("Error obteniendo directorio actual: {}", e))?,
        };

        if !target_path.exists() {
            return Err(format!("El directorio '{}' no existe", target_path.display()));
        }

        if !target_path.is_dir() {
            return Err(format!("'{}' no es un directorio", target_path.display()));
        }

        let mut result = LsResult::new(target_path.to_string_lossy().to_string());

        let entries = fs::read_dir(&target_path)
            .map_err(|e| format!("Error leyendo directorio: {}", e))?;

        for entry in entries {
            if let Ok(entry) = entry {
                let file_info = self.create_file_info(&entry.path())?;
                result.add_file(file_info);
            }
        }

        result.sort_by_name();
        Ok(result)
    }

    /// Ejecuta el comando ls con opciones específicas
    pub fn list_directory_with_options(&self, path: Option<&str>, options: &LsOptions) -> Result<LsResult, String> {
        let mut result = self.list_directory(path)?;

        // Aplicar ordenamiento según las opciones
        match options.sort_by {
            SortBy::Name => result.sort_by_name(),
            SortBy::Size => result.sort_by_size(),
            SortBy::Modified => result.sort_by_modified(),
        }

        // Aplicar filtros
        if options.show_hidden {
            // Por defecto ya incluimos archivos ocultos
        } else {
            result.files.retain(|file| !file.name.starts_with('.'));
        }

        if options.show_only_directories {
            result.files.retain(|file| file.is_directory);
        }

        if options.show_only_files {
            result.files.retain(|file| !file.is_directory);
        }

        Ok(result)
    }

    /// Crea información detallada de un archivo
    fn create_file_info(&self, path: &Path) -> Result<FileInfo, String> {
        let metadata = fs::metadata(path)
            .map_err(|e| format!("Error obteniendo metadatos de '{}': {}", path.display(), e))?;

        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let is_directory = metadata.is_dir();
        let size = if is_directory { 0 } else { metadata.len() };
        let modified = metadata.modified()
            .map_err(|e| format!("Error obteniendo fecha de modificación: {}", e))?;

        let permissions = self.format_permissions(&metadata);
        let (owner, group) = self.get_owner_and_group(path)?;

        Ok(FileInfo::new(name, path.to_string_lossy().to_string())
            .with_directory(is_directory)
            .with_size(size)
            .with_modified(modified)
            .with_permissions(permissions)
            .with_owner(owner)
            .with_group(group))
    }

    /// Formatea los permisos del archivo (compatible con Windows)
    fn format_permissions(&self, metadata: &fs::Metadata) -> String {
        let mut perms = String::new();

        // Tipo de archivo
        perms.push(if metadata.is_dir() { 'd' } else { '-' });

        // En Windows, simulamos permisos básicos
        // En sistemas Unix, usaríamos metadata.permissions().mode()
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            
            // Permisos del propietario
            perms.push(if mode & 0o400 != 0 { 'r' } else { '-' });
            perms.push(if mode & 0o200 != 0 { 'w' } else { '-' });
            perms.push(if mode & 0o100 != 0 { 'x' } else { '-' });

            // Permisos del grupo
            perms.push(if mode & 0o040 != 0 { 'r' } else { '-' });
            perms.push(if mode & 0o020 != 0 { 'w' } else { '-' });
            perms.push(if mode & 0o010 != 0 { 'x' } else { '-' });

            // Permisos de otros
            perms.push(if mode & 0o004 != 0 { 'r' } else { '-' });
            perms.push(if mode & 0o002 != 0 { 'w' } else { '-' });
            perms.push(if mode & 0o001 != 0 { 'x' } else { '-' });
        }

        #[cfg(not(unix))]
        {
            // En Windows, simulamos permisos básicos
            perms.push_str("rw-r--r--");
        }

        perms
    }

    /// Obtiene el propietario y grupo del archivo
    fn get_owner_and_group(&self, _path: &Path) -> Result<(String, String), String> {
        // En Windows, esto puede no funcionar completamente
        // Para simplificar, retornamos valores por defecto
        Ok(("usuario".to_string(), "grupo".to_string()))
    }

    /// Obtiene estadísticas del directorio
    pub fn get_directory_stats(&self, path: Option<&str>) -> Result<DirectoryStats, String> {
        let ls_result = self.list_directory(path)?;
        
        Ok(DirectoryStats {
            total_items: ls_result.files.len(),
            total_files: ls_result.total_files,
            total_directories: ls_result.total_directories,
            total_size: ls_result.total_size,
            current_directory: ls_result.current_directory,
        })
    }
}

/// Opciones para el comando ls
#[derive(Debug, Clone)]
pub struct LsOptions {
    pub show_hidden: bool,
    pub show_only_directories: bool,
    pub show_only_files: bool,
    pub sort_by: SortBy,
    pub long_format: bool,
    pub human_readable: bool,
}

impl Default for LsOptions {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_only_directories: false,
            show_only_files: false,
            sort_by: SortBy::Name,
            long_format: false,
            human_readable: false,
        }
    }
}

/// Criterio de ordenamiento
#[derive(Debug, Clone)]
pub enum SortBy {
    Name,
    Size,
    Modified,
}

/// Estadísticas del directorio
#[derive(Debug, Clone)]
pub struct DirectoryStats {
    #[allow(dead_code)]
    pub total_items: usize,
    #[allow(dead_code)]
    pub total_files: usize,
    #[allow(dead_code)]
    pub total_directories: usize,
    #[allow(dead_code)]
    pub total_size: u64,
    #[allow(dead_code)]
    pub current_directory: String,
}

/// Parser para comandos ls
pub struct LsCommandParser;

impl LsCommandParser {
    /// Parsea un comando ls y determina qué acción ejecutar
    pub fn parse_command(command: &str) -> LsCommand {
        let parts: Vec<&str> = command.split_whitespace().collect();
        
        if parts.is_empty() || parts[0] != "ls" {
            return LsCommand::NotLsCommand;
        }

        let mut options = LsOptions::default();
        let mut target_path = None;

        for part in &parts[1..] {
            if part.starts_with('-') {
                // Es una opción
                for flag in part[1..].chars() {
                    match flag {
                        'a' => options.show_hidden = true,
                        'd' => options.show_only_directories = true,
                        'f' => options.show_only_files = true,
                        'l' => options.long_format = true,
                        'h' => options.human_readable = true,
                        'S' => options.sort_by = SortBy::Size,
                        't' => options.sort_by = SortBy::Modified,
                        _ => return LsCommand::Invalid(format!("Opción desconocida: -{}", flag)),
                    }
                }
            } else {
                // Es un path
                if target_path.is_none() {
                    target_path = Some(part.to_string());
                } else {
                    return LsCommand::Invalid("Demasiados argumentos".to_string());
                }
            }
        }

        LsCommand::List(target_path, options)
    }

    /// Verifica si un comando es un comando ls
    #[allow(dead_code)]
    pub fn is_ls_command(command: &str) -> bool {
        !matches!(Self::parse_command(command), LsCommand::NotLsCommand)
    }
}

/// Enum que representa los diferentes comandos ls
#[derive(Debug, Clone)]
pub enum LsCommand {
    List(Option<String>, LsOptions),
    #[allow(dead_code)]
    Stats(Option<String>),
    Invalid(String),
    NotLsCommand,
}

impl LsCommand {
    /// Ejecuta el comando usando el caso de uso proporcionado
    pub fn execute(self, use_case: &LsCommandsUseCase) -> Result<LsResult, String> {
        match self {
            LsCommand::List(path, options) => {
                use_case.list_directory_with_options(path.as_deref(), &options)
            }
            LsCommand::Stats(path) => {
                use_case.get_directory_stats(path.as_deref())
                    .and_then(|_| use_case.list_directory(path.as_deref()))
            }
            LsCommand::Invalid(message) => Err(format!("Error: {}", message)),
            LsCommand::NotLsCommand => Err("No es un comando ls".to_string()),
        }
    }
}
