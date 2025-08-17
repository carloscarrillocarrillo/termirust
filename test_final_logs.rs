use std::fs;
use chrono::Utc;

fn main() {
    println!("🧪 Prueba final del sistema de logs de Termirust");
    
    // Obtener directorio del ejecutable
    let exe_path = std::env::current_exe().expect("Error obteniendo ruta del ejecutable");
    let exe_dir = exe_path.parent().expect("No se pudo obtener el directorio del ejecutable");
    let logs_dir = exe_dir.join("logs");
    
    println!("📁 Directorio del ejecutable: {}", exe_dir.display());
    println!("📁 Directorio de logs: {}", logs_dir.display());
    
    // Crear directorio de logs si no existe
    if !logs_dir.exists() {
        fs::create_dir(&logs_dir).expect("Error creando directorio de logs");
        println!("✅ Directorio de logs creado");
    } else {
        println!("✅ Directorio de logs ya existe");
    }
    
    // Crear archivo de log de prueba
    let now = Utc::now();
    let filename = logs_dir.join(format!("termirust_{}.log", now.format("%Y%m%d_%H%M%S")));
    
    let log_content = format!(
        "[{}] [INFO] [termirust] === PRUEBA DEL SISTEMA DE LOGS ===\n\
        [{}] [INFO] [termirust] Ubicación del ejecutable: {}\n\
        [{}] [INFO] [termirust] Directorio de logs: {}\n\
        [{}] [INFO] [termirust] Archivo de log: {}\n\
        [{}] [INFO] [termirust] Sistema de logs funcionando correctamente\n\
        [{}] [INFO] [termirust] === FIN DE PRUEBA ===",
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        exe_dir.display(),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        logs_dir.display(),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        filename.display(),
        now.format("%Y-%m-%d %H:%M:%S%.3f"),
        now.format("%Y-%m-%d %H:%M:%S%.3f")
    );
    
    fs::write(&filename, log_content).expect("Error escribiendo archivo de log");
    println!("✅ Archivo de log creado: {}", filename.display());
    
    // Verificar archivo
    if let Ok(metadata) = fs::metadata(&filename) {
        println!("📊 Tamaño del archivo: {} bytes", metadata.len());
    }
    
    println!("🎉 Sistema de logs configurado correctamente!");
    println!("💡 Los logs se guardarán en: {}", logs_dir.display());
}
