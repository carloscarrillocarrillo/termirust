use std::fs;
use std::path::Path;

fn main() {
    println!("Probando sistema de logs...");
    
    // Verificar si existe el directorio logs
    let logs_dir = Path::new("logs");
    if logs_dir.exists() {
        println!("✅ Directorio logs existe");
        
        // Listar archivos de log
        if let Ok(entries) = fs::read_dir(logs_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(".log") {
                            println!("📄 Archivo de log encontrado: {}", file_name);
                            
                            // Mostrar las primeras líneas del archivo
                            if let Ok(contents) = fs::read_to_string(entry.path()) {
                                let lines: Vec<&str> = contents.lines().take(10).collect();
                                println!("📝 Primeras {} líneas:", lines.len());
                                for line in lines {
                                    println!("   {}", line);
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("❌ Directorio logs no existe");
    }
    
    println!("Prueba completada.");
}
