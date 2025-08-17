use crate::domain::entities::Command;
use crate::domain::repositories::{CommandRepository, FileSystemRepository};
use crate::application::ls_commands::{LsCommandsUseCase, LsCommandParser};
use crate::presentation::commands::ls_display::LsDisplayComponent;
use crate::application::commands::exit_commands::ExitCommandsUseCase;
use crate::presentation::commands::exit_display::ExitDisplayComponent;
use std::process::Command as ProcessCommand;
use std::env;
use std::path::Path;

/// Implementación concreta del repositorio de comandos
pub struct SystemCommandRepository;

impl CommandRepository for SystemCommandRepository {
    fn execute_command(&self, command: &Command) -> Result<Command, String> {
        if command.name.is_empty() {
            return Ok(command.clone());
        }

        // Comandos internos
        match command.name.as_str() {
            "ls" | "dir" => {
                // Usar nuestro comando ls avanzado
                let ls_use_case = LsCommandsUseCase::new();
                
                // Reconstruir el comando completo para el parser
                let mut full_command = String::from("ls");
                for arg in &command.args {
                    full_command.push(' ');
                    full_command.push_str(arg);
                }
                
                let parsed = LsCommandParser::parse_command(&full_command);
                match parsed.execute(&ls_use_case) {
                    Ok(result) => {
                        // Detectar opciones del comando
                        let mut options = crate::application::ls_commands::LsOptions::default();
                        if full_command.contains("-l") {
                            options.long_format = true;
                        }
                        if full_command.contains("-a") {
                            options.show_hidden = true;
                        }
                        if full_command.contains("-h") {
                            options.human_readable = true;
                        }
                        if full_command.contains("-S") {
                            options.sort_by = crate::application::ls_commands::SortBy::Size;
                        }
                        if full_command.contains("-t") {
                            options.sort_by = crate::application::ls_commands::SortBy::Modified;
                        }
                        
                        let output_lines = LsDisplayComponent::render(&result, &options);
                        let output = output_lines.join("\n");
                        Ok(command.clone().with_output(output).with_exit_code(0))
                    }
                    Err(e) => {
                        let error_lines = LsDisplayComponent::render_error(&e);
                        let error_output = error_lines.join("\n");
                        Ok(command.clone().with_output(error_output).with_exit_code(1))
                    }
                }
            }
            "cd" => {
                let path = command.args.first().map(|s| s.to_string()).unwrap_or_else(|| ".".to_string());
                let fs_repo = SystemFileSystemRepository;
                match fs_repo.change_directory(&path) {
                    Ok(_) => Ok(command.clone().with_output("".to_string()).with_exit_code(0)),
                    Err(e) => Ok(command.clone().with_output(e).with_exit_code(1))
                }
            }
            "pwd" => {
                let fs_repo = SystemFileSystemRepository;
                match fs_repo.get_current_directory() {
                    Ok(path) => Ok(command.clone().with_output(path).with_exit_code(0)),
                    Err(e) => Ok(command.clone().with_output(e).with_exit_code(1))
                }
            }
            "clear" => {
                Ok(command.clone().with_output("CLEAR".to_string()).with_exit_code(0))
            }
            "help" => {
                let help_text = r#"
Comandos disponibles:
- ls, dir: Listar archivos y directorios (con opciones: -a, -l, -h, -S, -t)
- cd <directorio>: Cambiar directorio
- pwd: Mostrar directorio actual
- clear: Limpiar pantalla
- help: Mostrar esta ayuda
- exit, quit: Cerrar la aplicación Termirust

Opciones del comando ls:
- ls -a: Mostrar archivos ocultos
- ls -l: Formato largo con detalles
- ls -h: Tamaños en formato legible
- ls -S: Ordenar por tamaño
- ls -t: Ordenar por fecha de modificación
- ls -la: Combinar opciones
"#;
                Ok(command.clone().with_output(help_text.to_string()).with_exit_code(0))
            }
            "exit" | "quit" => {
                // Usar nuestro comando exit avanzado
                let exit_use_case = ExitCommandsUseCase::new();
                
                match exit_use_case.execute_exit(command) {
                    Ok(result) => {
                        let output_lines = ExitDisplayComponent::render(&result);
                        let output = output_lines.join("\n");
                        Ok(command.clone().with_output(output).with_exit_code(result.exit_code))
                    }
                    Err(e) => {
                        let error_lines = ExitDisplayComponent::render_error(&e);
                        let error_output = error_lines.join("\n");
                        Ok(command.clone().with_output(error_output).with_exit_code(1))
                    }
                }
            }
            _ => {
                // Intentar ejecutar comando del sistema
                let mut process = ProcessCommand::new(&command.name);
                process.args(&command.args);
                
                match process.output() {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let combined_output = if !stderr.is_empty() {
                            format!("{}\n{}", stdout, stderr)
                        } else {
                            stdout.to_string()
                        };
                        
                        Ok(command.clone()
                            .with_output(combined_output)
                            .with_exit_code(output.status.code().unwrap_or(1)))
                    }
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
                }
            }
        }
    }
}

/// Implementación concreta del repositorio del sistema de archivos
pub struct SystemFileSystemRepository;

impl FileSystemRepository for SystemFileSystemRepository {
    fn list_directory(&self, path: &str) -> Result<Vec<String>, String> {
        let path = Path::new(path);
        if !path.exists() {
            return Err(format!("El directorio '{}' no existe", path.display()));
        }

        if !path.is_dir() {
            return Err(format!("'{}' no es un directorio", path.display()));
        }

        match std::fs::read_dir(path) {
            Ok(entries) => {
                let mut files = Vec::new();
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let name = entry.file_name().to_string_lossy().to_string();
                            let metadata = entry.metadata().unwrap();
                            let is_dir = metadata.is_dir();
                            let size = metadata.len();
                            
                            let display_name = if is_dir {
                                format!("📁 {}", name)
                            } else {
                                format!("📄 {} ({} bytes)", name, size)
                            };
                            
                            files.push(display_name);
                        }
                        Err(e) => {
                            files.push(format!("Error leyendo entrada: {}", e));
                        }
                    }
                }
                files.sort();
                Ok(files)
            }
            Err(e) => Err(format!("Error leyendo directorio: {}", e))
        }
    }

    fn get_current_directory(&self) -> Result<String, String> {
        env::current_dir()
            .map(|path| path.to_string_lossy().to_string())
            .map_err(|e| format!("Error obteniendo directorio actual: {}", e))
    }

    fn change_directory(&self, path: &str) -> Result<(), String> {
        let new_path = if path == "." {
            env::current_dir().unwrap()
        } else if path == ".." {
            env::current_dir()
                .unwrap()
                .parent()
                .unwrap_or(Path::new("."))
                .to_path_buf()
        } else {
            Path::new(path).to_path_buf()
        };

        if !new_path.exists() {
            return Err(format!("El directorio '{}' no existe", new_path.display()));
        }

        if !new_path.is_dir() {
            return Err(format!("'{}' no es un directorio", new_path.display()));
        }

        env::set_current_dir(new_path)
            .map_err(|e| format!("Error cambiando directorio: {}", e))
    }
}
