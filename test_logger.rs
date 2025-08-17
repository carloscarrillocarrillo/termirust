use std::fs;
use std::path::Path;
use chrono::Utc;

fn main() {
    println!("🧪 Probando sistema de logs de Termirust...");
    
    // Obtener el directorio del ejecutable
    let exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            println!("❌ Error obteniendo ruta del ejecutable: {}", e);
            return;
        }
    };
    
    let exe_dir = match exe_path.parent() {
        Some(dir) => dir,
        None => {
            println!("❌ No se pudo obtener el directorio del ejecutable");
            return;
        }
    };
    
    println!("📁 Directorio del ejecutable: {}", exe_dir.display());
    
    // Simular la creación del directorio de logs
    let logs_dir = exe_dir.join("logs");
    if !logs_dir.exists() {
        match fs::create_dir(&logs_dir) {
            Ok(_) => println!("✅ Directorio logs creado en: {}", logs_dir.display()),
            Err(e) => {
                println!("❌ Error creando directorio logs: {}", e);
                return;
            }
        }
    } else {
        println!("✅ Directorio logs ya existe en: {}", logs_dir.display());
    }
    
    // Generar nombre de archivo con timestamp
    let now = Utc::now();
    let filename = logs_dir.join(format!(
        "termirust_{}.log",
        now.format("%Y%m%d_%H%M%S")
    ));
    
    println!("📄 Creando archivo de log: {}", filename);
    
    // Crear archivo de log de prueba
    let log_content = format!(
        "[{}] [INFO] [termirust] === APLICACIÓN TERMIRUST INICIADA ===\n\
        [{}] [INFO] [termirust] Versión: 0.1.0\n\
        [{}] [INFO] [termirust] Timestamp de inicio: {}\n\
        [{}] [INFO] [termirust] Sistema de logs inicializado correctamente\n\
        [{}] [DEBUG] [termirust] Probando diferentes niveles de log\n\
        [{}] [WARN] [termirust] Este es un mensaje de advertencia de prueba\n\
        [{}] [ERROR] [termirust] Este es un mensaje de error de prueba\n\
        [{}] [INFO] [termirust] Comando ejecutado exitosamente: 'ls' - Salida: archivo1.txt archivo2.txt\n\
        [{}] [ERROR] [termirust] Comando falló: 'comando_inexistente' - Error: comando no encontrado\n\
        [{}] [DEBUG] [termirust] Estadísticas del sistema - Memoria: 45.2%, CPU: 23.1%, Red: 15MB recibidos, 8MB transmitidos\n\
        [{}] [INFO] [termirust] === APLICACIÓN TERMIRUST CERRADA ===\n\
        [{}] [INFO] [termirust] Timestamp de cierre: {}",
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S")
    );
    
    // Escribir el archivo de log
    match fs::write(&filename, log_content) {
        Ok(_) => println!("✅ Archivo de log creado exitosamente"),
        Err(e) => {
            println!("❌ Error escribiendo archivo de log: {}", e);
            return;
        }
    }
    
    // Verificar que el archivo se creó
    if Path::new(&filename).exists() {
        println!("✅ Archivo de log verificado");
        
        // Mostrar estadísticas del archivo
        if let Ok(metadata) = fs::metadata(&filename) {
            println!("📊 Tamaño del archivo: {} bytes", metadata.len());
        }
        
        // Mostrar las primeras líneas
        if let Ok(contents) = fs::read_to_string(&filename) {
            let lines: Vec<&str> = contents.lines().take(5).collect();
            println!("📝 Primeras {} líneas del archivo:", lines.len());
            for line in lines {
                println!("   {}", line);
            }
        }
    } else {
        println!("❌ El archivo de log no se creó correctamente");
    }
    
    // Listar todos los archivos de log
    println!("\n📁 Archivos de log en el directorio:");
    if let Ok(entries) = fs::read_dir(&logs_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".log") {
                        if let Ok(metadata) = fs::metadata(entry.path()) {
                            println!("   📄 {} ({} bytes)", file_name, metadata.len());
                        } else {
                            println!("   📄 {}", file_name);
                        }
                    }
                }
            }
        }
    }
    
    println!("\n🎉 Prueba del sistema de logs completada exitosamente!");
    println!("💡 El sistema está listo para ser usado en la aplicación principal.");
}
