use std::fs;
use std::sync::{Once, Mutex};
use chrono::{DateTime, Utc};
use log::{LevelFilter, Log, Metadata, Record};
use std::io::Write;

static INIT: Once = Once::new();

pub struct Logger {
    file: Mutex<std::fs::File>,
    level: LevelFilter,
}

impl Logger {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Obtener el directorio del ejecutable
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().ok_or("No se pudo obtener el directorio del ejecutable")?;
        
        // Crear directorio de logs en la misma ubicación que el ejecutable
        let logs_dir = exe_dir.join("logs");
        if !logs_dir.exists() {
            fs::create_dir(&logs_dir)?;
        }

        // Generar nombre de archivo con timestamp
        let now: DateTime<Utc> = Utc::now();
        let filename = logs_dir.join(format!(
            "termirust_{}.log",
            now.format("%Y%m%d_%H%M%S")
        ));

        // Crear archivo de log
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)?;

        Ok(Logger {
            file: Mutex::new(file),
            level: LevelFilter::Debug,
        })
    }

    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        INIT.call_once(|| {
            let logger = Logger::new().expect("No se pudo crear el logger");
            log::set_boxed_logger(Box::new(logger))
                .expect("No se pudo establecer el logger");
            log::set_max_level(LevelFilter::Debug);
        });
        Ok(())
    }

    fn format_log_record(&self, record: &Record) -> String {
        let now: DateTime<Utc> = Utc::now();
        let level = record.level();
        let target = record.target();
        let args = record.args();
        
        format!(
            "[{}] [{}] [{}] {}",
            now.format("%Y-%m-%d %H:%M:%S%.3f"),
            level,
            target,
            args
        )
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log_entry = self.format_log_record(record);
            
            // Escribir al archivo
            if let Ok(mut file) = self.file.lock() {
                if let Err(e) = writeln!(*file, "{}", log_entry) {
                    eprintln!("Error escribiendo al log: {}", e);
                }
            }
            
            // También escribir a la consola para errores críticos
            if record.level() >= log::Level::Error {
                eprintln!("{}", log_entry);
            }
        }
    }

    fn flush(&self) {
        if let Ok(file) = self.file.lock() {
            if let Err(e) = file.sync_all() {
                eprintln!("Error sincronizando el archivo de log: {}", e);
            }
        }
    }
}

// Funciones de conveniencia para logging
#[allow(dead_code)]
pub fn log_info(message: &str) {
    log::info!("{}", message);
}

#[allow(dead_code)]
pub fn log_error(message: &str) {
    log::error!("{}", message);
}

#[allow(dead_code)]
pub fn log_warning(message: &str) {
    log::warn!("{}", message);
}

#[allow(dead_code)]
pub fn log_debug(message: &str) {
    log::debug!("{}", message);
}

pub fn log_command_execution(command: &str, success: bool, output: &str) {
    if success {
        log::info!("Comando ejecutado exitosamente: '{}' - Salida: {}", command, output);
    } else {
        log::error!("Comando falló: '{}' - Error: {}", command, output);
    }
}

pub fn log_system_stats(memory_usage: f64, cpu_usage: f64, network_usage: (u64, u64)) {
    log::debug!(
        "Estadísticas del sistema - Memoria: {:.1}%, CPU: {:.1}%, Red: {}MB recibidos, {}MB transmitidos",
        memory_usage,
        cpu_usage,
        network_usage.0 / 1024 / 1024,
        network_usage.1 / 1024 / 1024
    );
}

pub fn log_matrix_effect_update(_drops_count: usize) {
    // log::debug!("Efectos Matrix actualizados - Gotas activas: {}", drops_count);
}

#[allow(dead_code)]
pub fn log_input_handling(input: &str, processed: bool) {
    log::debug!("Entrada procesada: '{}' - Procesada: {}", input, processed);
}

pub fn log_application_start() {
    log::info!("=== APLICACIÓN TERMIRUST INICIADA ===");
    log::info!("Versión: 0.1.0");
    log::info!("Timestamp de inicio: {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
}

pub fn log_application_shutdown() {
    log::info!("=== APLICACIÓN TERMIRUST CERRADA ===");
    log::info!("Timestamp de cierre: {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
}
