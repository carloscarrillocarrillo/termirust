mod domain;
mod infrastructure;
mod application;
mod presentation;

use domain::services::{CommandService, TerminalService};
use infrastructure::repositories::SystemCommandRepository;
use infrastructure::logging::{Logger, log_application_start};
use application::use_cases::{ExecuteCommandUseCase, HandleInputUseCase};
use presentation::gui_terminal::MatrixTerminalApp;

fn main() -> Result<(), eframe::Error> {
    // Inicializar sistema de logs
    if let Err(e) = Logger::init() {
        eprintln!("Error inicializando el sistema de logs: {}", e);
    }
    
    // Registrar inicio de la aplicación
    log_application_start();
    
    // Configurar opciones de la ventana
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_decorations(true)
            .with_transparent(false),
        ..Default::default()
    };

    // Configurar dependencias (Dependency Injection)
    let command_repository = SystemCommandRepository;
    let command_service = CommandService::new(command_repository);
    let terminal_service = TerminalService::new();
    
    // Crear casos de uso
    let input_handler = HandleInputUseCase::new(terminal_service);
    let command_executor = ExecuteCommandUseCase::new(
        command_service, 
        TerminalService::new()
    );
    
    // Crear la aplicación GUI
    let app = MatrixTerminalApp::new(
        input_handler, 
        command_executor
    );
    
    // Ejecutar la aplicación
    eframe::run_native(
        "Termirust",
        options,
        Box::new(|_cc| Box::new(app)),
    )
}
